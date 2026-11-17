//! Main-thread macOS adapter. No content, clipboard or persisted UI drafts.
#![cfg_attr(feature = "native-probe", allow(dead_code))]
use crate::panel_session::{close_with_restore, hide_without_fallback, Session, Ticket};
use crate::window_geometry::{fit, native_frame, workbench_frame, Rect};
use objc2::rc::Retained;
use objc2_app_kit::{
    NSApplicationActivationOptions, NSRunningApplication, NSWindow, NSWindowCollectionBehavior,
    NSWorkspace,
};
use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicBool, Ordering},
        OnceLock,
    },
    time::{Duration, Instant},
};
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::TrayIconBuilder,
    window::{Effect, EffectState, EffectsBuilder},
    Emitter, Manager, WebviewWindow,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum LifecycleError {
    WindowOperationFailed,
    WindowUnavailable,
    DisplayUnavailable,
    FocusRestoreRejected,
    StaleOrBusy,
    ForbiddenWindow,
    StaleSession,
    ReturnToSearchFirst,
    ShortcutRegistrationFailed,
}
impl std::fmt::Display for LifecycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for LifecycleError {}
#[derive(Clone, Debug, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub version: u32,
    pub session: Option<u32>,
    pub view: View,
    pub transition: Option<u32>,
    pub settings_requested: bool,
    pub shortcut_available: bool,
    pub shortcut_failure: Option<String>,
    pub fault: Option<LifecycleError>,
    pub reduced_motion: bool,
    pub demo_paused: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum View {
    Search,
    History,
    Base64,
}
impl View {
    fn height(self) -> u32 {
        match self {
            Self::Search => 264,
            Self::History => 480,
            Self::Base64 => 620,
        }
    }
}
struct Animation {
    ticket: Ticket,
    start: Instant,
    from: u32,
    to: u32,
    native_done: bool,
}
struct Runtime {
    session: Session,
    snapshot: Snapshot,
    original: Option<Retained<NSRunningApplication>>,
    animation: Option<Animation>,
    pause_item: Option<CheckMenuItem<tauri::Wry>>,
}
impl Default for Runtime {
    fn default() -> Self {
        Self {
            session: Session::default(),
            original: None,
            animation: None,
            pause_item: None,
            snapshot: Snapshot {
                version: 0,
                session: None,
                view: View::Search,
                transition: None,
                settings_requested: false,
                shortcut_available: false,
                shortcut_failure: None,
                fault: None,
                reduced_motion: false,
                demo_paused: false,
            },
        }
    }
}
thread_local! { static STATE: RefCell<Runtime> = RefCell::new(Runtime::default()); }
static ANIMATING: AtomicBool = AtomicBool::new(false);
static WORKER: OnceLock<std::thread::Thread> = OnceLock::new();
type Result<T> = std::result::Result<T, LifecycleError>;
fn platform<T>(result: tauri::Result<T>) -> Result<T> {
    result.map_err(|_| LifecycleError::WindowOperationFailed)
}
fn window(app: &tauri::AppHandle, label: &str) -> Result<WebviewWindow> {
    app.get_webview_window(label)
        .ok_or(LifecycleError::WindowUnavailable)
}
pub fn snapshot() -> Snapshot {
    STATE.with(|s| s.borrow().snapshot.clone())
}
fn publish(app: &tauri::AppHandle) {
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        s.snapshot.version += 1;
    });
    // Notifications contain a version only. Clients subscribe before reading.
    for label in ["workbench", "quick-panel"] {
        let _ = app.emit_to(label, "lifecycle-version", snapshot().version);
    }
}
pub fn shortcut_status(app: &tauri::AppHandle, available: bool, failed: bool) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.snapshot.shortcut_available = available;
        state.snapshot.shortcut_failure =
            failed.then(|| "快捷键配置未完成，请在设置中查看恢复状态；仍可从托盘打开面板。".into());
        if available && state.snapshot.fault == Some(LifecycleError::ShortcutRegistrationFailed) {
            state.snapshot.fault = None;
        }
    });
    publish(app);
}
fn report(app: &tauri::AppHandle, result: Result<()>) {
    if let Err(error) = result {
        let changed = STATE.with(|s| {
            let mut s = s.borrow_mut();
            if s.snapshot.fault == Some(error) {
                return false;
            }
            s.snapshot.fault = Some(error);
            true
        });
        if changed {
            publish(app);
        }
    }
}
fn geometry(app: &tauri::AppHandle, opening: bool, height: u32) -> Result<()> {
    let panel = window(app, "quick-panel")?;
    let monitor = if opening {
        let cursor = platform(panel.cursor_position())?;
        platform(panel.monitor_from_point(cursor.x, cursor.y))?
    } else {
        platform(panel.current_monitor())?
    }
    .or(platform(panel.primary_monitor())?)
    .ok_or(LifecycleError::DisplayUnavailable)?;
    let area = monitor.work_area();
    let scale = monitor.scale_factor();
    let position = platform(panel.outer_position())?;
    let size = platform(panel.outer_size())?;
    let rect = fit(
        Rect {
            x: area.position.x,
            y: area.position.y,
            width: area.size.width,
            height: area.size.height,
        },
        (!opening).then_some(Rect {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        }),
        (650.0 * scale).round() as u32,
        (f64::from(height) * scale).round() as u32,
        (24.0 * scale).round() as i32,
    );
    apply_frame(&panel, rect)
}

/// Commit outer position and size together on AppKit's main thread.
fn apply_frame(panel: &WebviewWindow, rect: Rect) -> Result<()> {
    let position = platform(panel.outer_position())?;
    // Tauri/Tao queues set_size/set_position onto the main dispatch queue even
    // from the main thread. Commit one native frame synchronously before show
    // (and on each resize tick), rather than displaying the old centered frame.
    // SAFETY: geometry is called only on AppKit's main thread; this borrow is
    // limited to the live NSWindow's immediate frame read/write.
    let native = unsafe { &*platform(panel.ns_window())?.cast::<NSWindow>() };
    let mut frame = native.frame();
    let target = native_frame(
        (
            frame.origin.x,
            frame.origin.y,
            frame.size.width,
            frame.size.height,
        ),
        (position.x, position.y),
        rect,
        platform(panel.scale_factor())?,
    );
    frame.origin.x = target.0;
    frame.origin.y = target.1;
    frame.size.width = target.2;
    frame.size.height = target.3;
    native.setFrame_display(frame, true);
    Ok(())
}
fn size_workbench(app: &tauri::AppHandle) -> Result<()> {
    let workbench = window(app, "workbench")?;
    let monitor = platform(workbench.current_monitor())?
        .or(platform(workbench.primary_monitor())?)
        .ok_or(LifecycleError::DisplayUnavailable)?;
    let area = monitor.work_area();
    apply_frame(
        &workbench,
        workbench_frame(Rect {
            x: area.position.x,
            y: area.position.y,
            width: area.size.width,
            height: area.size.height,
        }),
    )
}

pub fn open(app: &tauri::AppHandle) -> Result<()> {
    if STATE.with(|s| s.borrow().session.current().is_some()) {
        return Ok(());
    }
    let original = NSWorkspace::sharedWorkspace().frontmostApplication();

    geometry(app, true, View::Search.height())?;

    STATE.with(|s| {
        let mut s = s.borrow_mut();
        s.snapshot.session = Some(s.session.open());
        s.snapshot.view = View::Search;
        s.snapshot.transition = None;
        s.snapshot.fault = if s.snapshot.shortcut_available {
            None
        } else {
            Some(LifecycleError::ShortcutRegistrationFailed)
        };
        s.snapshot.reduced_motion =
            NSWorkspace::sharedWorkspace().accessibilityDisplayShouldReduceMotion();
        s.original = original;
    });
    publish(app);
    let panel = window(app, "quick-panel")?;
    if platform(panel.show())
        .and_then(|_| platform(panel.set_focus()))
        .is_err()
    {
        let _ = close(app, false);
        return Err(LifecycleError::WindowOperationFailed);
    }

    Ok(())
}
pub fn close(app: &tauri::AppHandle, restore: bool) -> Result<()> {
    let suppress_fallback = restore
        && STATE.with(|s| {
            s.borrow()
                .original
                .as_ref()
                .is_some_and(|original| original.processIdentifier() != std::process::id() as i32)
        });
    let result = close_with_restore(
        || {
            STATE.with(|s| {
                let mut s = s.borrow_mut();
                let original = s.original.take();
                s.session.close();
                s.animation = None;
                ANIMATING.store(false, Ordering::Relaxed);
                s.snapshot.session = None;
                s.snapshot.transition = None;
                original
            })
        },
        || {
            let hide = || window(app, "quick-panel").and_then(|w| platform(w.hide()));
            let result = if suppress_fallback {
                match window(app, "workbench") {
                    Ok(workbench) => hide_without_fallback(
                        |enabled| platform(workbench.set_focusable(enabled)),
                        hide,
                    ),
                    Err(error) => hide().and(Err(error)),
                }
            } else {
                hide()
            };

            publish(app);
            result
        },
        |original| {
            if original.isTerminated() {
                return Ok(());
            }
            if original.processIdentifier() == std::process::id() as i32 {
                let workbench = window(app, "workbench")?;
                platform(workbench.show())?;
                platform(workbench.set_focus())
            } else {
                #[allow(deprecated)]
                let accepted = original
                    .activateWithOptions(NSApplicationActivationOptions::ActivateIgnoringOtherApps);

                if accepted {
                    Ok(())
                } else {
                    Err(LifecycleError::FocusRestoreRejected)
                }
            }
        },
        restore,
    );
    report(app, result);
    result
}
pub fn toggle(app: &tauri::AppHandle) {
    let result = if snapshot().session.is_some() {
        close(app, true)
    } else {
        open(app)
    };
    report(app, result);
}
pub fn workbench(app: &tauri::AppHandle, settings: bool) -> Result<()> {
    close(app, false)?;
    if settings {
        STATE.with(|s| s.borrow_mut().snapshot.settings_requested = true);
        publish(app);
    }
    let workbench = window(app, "workbench")?;

    platform(workbench.show())?;
    platform(workbench.unminimize())?;
    platform(workbench.set_focus())
}
pub fn acknowledge_settings(app: &tauri::AppHandle) {
    STATE.with(|s| s.borrow_mut().snapshot.settings_requested = false);
    publish(app);
}
pub fn transition(app: &tauri::AppHandle, session: u32, view: View) -> Result<Snapshot> {
    let panel = window(app, "quick-panel")?;
    let from = (f64::from(platform(panel.inner_size())?.height) / platform(panel.scale_factor())?)
        .round() as u32;
    let ticket = STATE
        .with(|s| s.borrow_mut().session.begin(session))
        .ok_or(LifecycleError::StaleOrBusy)?;
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        s.snapshot.view = view;
        s.snapshot.transition = Some(ticket.request);
        s.animation = Some(Animation {
            ticket,
            start: Instant::now(),
            from,
            to: view.height(),
            native_done: false,
        });
    });
    ANIMATING.store(true, Ordering::Relaxed);
    if let Some(worker) = WORKER.get() {
        worker.unpark();
    }
    publish(app);
    Ok(snapshot())
}
pub fn complete(app: &tauri::AppHandle, session: u32, request: u32) -> Result<()> {
    let ticket = Ticket { session, request };
    let done = STATE.with(|s| {
        s.borrow()
            .animation
            .as_ref()
            .is_some_and(|a| a.ticket == ticket && a.native_done)
    });
    // A UI completion before native sizing ends is harmless; watchdog finishes later.
    if done {
        finish(app, ticket);
    }
    Ok(())
}
fn finish(app: &tauri::AppHandle, ticket: Ticket) {
    let changed = STATE.with(|s| {
        let mut s = s.borrow_mut();
        if !s.session.finish(ticket) {
            return false;
        }
        s.animation = None;
        ANIMATING.store(false, Ordering::Relaxed);
        s.snapshot.transition = None;
        true
    });
    if changed {
        publish(app);
    }
}
fn tick(app: &tauri::AppHandle) -> Result<()> {
    let frame = STATE.with(|s| {
        let s = s.borrow();
        s.animation.as_ref().map(|a| {
            let elapsed = a.start.elapsed();
            let progress = if s.snapshot.reduced_motion {
                1.0
            } else {
                (elapsed.as_secs_f64() / 0.2).min(1.0)
            };
            let eased = 1.0 - (1.0 - progress).powi(3);
            (
                a.ticket,
                (f64::from(a.from) + (f64::from(a.to) - f64::from(a.from)) * eased).round() as u32,
                progress >= 1.0,
                elapsed >= Duration::from_millis(750),
            )
        })
    });
    if let Some((ticket, height, done, expired)) = frame {
        if let Err(error) = geometry(app, false, height) {
            finish(app, ticket);
            return Err(error);
        }
        if done {
            STATE.with(|s| {
                if let Some(a) = s.borrow_mut().animation.as_mut() {
                    a.native_done = true;
                }
            });
        }
        if expired {
            finish(app, ticket);
        }
    }
    Ok(())
}
fn recover_display(app: &tauri::AppHandle) -> Result<()> {
    if snapshot().session.is_none() {
        return Ok(());
    }
    let panel = window(app, "quick-panel")?;
    let pos = platform(panel.outer_position())?;
    let size = platform(panel.outer_size())?;
    let contained = platform(panel.available_monitors())?.iter().any(|m| {
        let a = m.work_area();
        pos.x >= a.position.x
            && pos.y >= a.position.y
            && i64::from(pos.x) + i64::from(size.width)
                <= i64::from(a.position.x) + i64::from(a.size.width)
            && i64::from(pos.y) + i64::from(size.height)
                <= i64::from(a.position.y) + i64::from(a.size.height)
    });
    if !contained {
        geometry(app, false, snapshot().view.height())?;
    }
    Ok(())
}
pub fn toggle_demo_pause(app: &tauri::AppHandle) -> Result<()> {
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        let paused = !s.snapshot.demo_paused;
        if let Some(item) = &s.pause_item {
            platform(item.set_checked(paused))?;
        }
        s.snapshot.demo_paused = paused;
        Ok::<_, LifecycleError>(())
    })?;
    publish(app);
    Ok(())
}
pub fn setup(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    report(app.handle(), size_workbench(app.handle()));
    let panel = window(app.handle(), "quick-panel")?;
    // SAFETY: setup is on the AppKit main thread; borrow the live NSWindow only
    // for this synchronous call. It is never retained or passed to another thread.
    let native = unsafe { &*panel.ns_window()?.cast::<NSWindow>() };

    native.setCollectionBehavior(
        NSWindowCollectionBehavior::MoveToActiveSpace
            | NSWindowCollectionBehavior::FullScreenAuxiliary,
    );
    panel.set_effects(
        EffectsBuilder::new()
            .effect(Effect::HudWindow)
            .state(EffectState::Active)
            .build(),
    )?;
    let home = MenuItem::with_id(app, "home", "打开 Workbench", true, None::<&str>)?;
    let quick = MenuItem::with_id(app, "quick", "打开 Quick Panel", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "设置…", true, Some("CmdOrCtrl+,"))?;
    let quit = MenuItem::with_id(app, "quit", "退出 Nibble", true, Some("CmdOrCtrl+Q"))?;
    let pause = CheckMenuItem::with_id(
        app,
        "demo-pause",
        "暂停演示（不控制真实捕获）",
        true,
        false,
        None::<&str>,
    )?;
    STATE.with(|s| s.borrow_mut().pause_item = Some(pause.clone()));
    let tray_menu = Menu::with_items(app, &[&home, &quick, &pause, &settings, &quit])?;
    TrayIconBuilder::with_id("nibble")
        .icon(
            app.default_window_icon()
                .ok_or("tray icon unavailable")?
                .clone(),
        )
        .tooltip("Nibble Local")
        .menu(&tray_menu)
        .build(app)?;
    let menu = Submenu::with_items(
        app,
        "Nibble",
        true,
        &[&settings, &PredefinedMenuItem::separator(app)?, &quit],
    )?;
    let edit = Submenu::with_items(
        app,
        "编辑",
        true,
        &[
            &PredefinedMenuItem::undo(app, None)?,
            &PredefinedMenuItem::redo(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::cut(app, None)?,
            &PredefinedMenuItem::copy(app, None)?,
            &PredefinedMenuItem::paste(app, None)?,
            &PredefinedMenuItem::select_all(app, None)?,
        ],
    )?;
    let windows = Submenu::with_items(
        app,
        "窗口",
        true,
        &[&PredefinedMenuItem::close_window(app, None)?],
    )?;
    app.set_menu(Menu::with_items(app, &[&menu, &edit, &windows])?)?;
    app.on_menu_event(|app, event| {
        let result = match event.id().as_ref() {
            "home" => workbench(app, false),
            "quick" => open(app),
            "settings" => workbench(app, true),
            "demo-pause" => toggle_demo_pause(app),
            "quit" => {
                app.exit(0);
                Ok(())
            }
            _ => Ok(()),
        };
        report(app, result);
    });
    // Install this optional service after the tray. Plugin-manager failure must
    // leave Workbench and the tray usable, just like a key registration failure.
    let plugin_ready = app
        .handle()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, key, event| {
                    if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        let handle = app.clone();
                        let key = *key;
                        let _ = app.run_on_main_thread(move || {
                            if crate::settings::handles(&key) {
                                toggle(&handle);
                            }
                        });
                    }
                })
                .build(),
        )
        .is_ok();
    crate::settings::initialize(app.handle(), plugin_ready);
    let handle = app.handle().clone();
    panel.on_window_event(move |event| {
        match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                report(&handle, close(&handle, true));
            }
            tauri::WindowEvent::Focused(false) => {
                // The native blur event is authoritative. A cached focus query
                // can lag behind it. Hidden-window events do not cancel a session.
                let active_blur = snapshot().session.is_some()
                    && window(&handle, "quick-panel")
                        .is_ok_and(|w| w.is_visible().unwrap_or(false));
                if active_blur {
                    report(&handle, close(&handle, false));
                }
            }
            _ => {}
        }
    });
    let handle = app.handle().clone();
    window(&handle, "workbench")?.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            report(
                &handle,
                window(&handle, "workbench").and_then(|w| platform(w.hide())),
            );
        }
    });
    let handle = app.handle().clone();
    std::thread::spawn(move || {
        let _ = WORKER.set(std::thread::current());
        let mut last_display_check = Instant::now();
        loop {
            std::thread::park_timeout(if ANIMATING.load(Ordering::Relaxed) {
                Duration::from_millis(16)
            } else {
                Duration::from_secs(1)
            });
            let app = handle.clone();
            let recover = last_display_check.elapsed() >= Duration::from_secs(1);
            if recover {
                last_display_check = Instant::now();
            }
            if handle
                .run_on_main_thread(move || {
                    report(&app, tick(&app));
                    if recover {
                        report(&app, recover_display(&app));
                    }
                })
                .is_err()
            {
                break;
            }
        }
    });
    Ok(())
}
