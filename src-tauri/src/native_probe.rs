//! Throwaway macOS risk harness, compiled only with `native-probe`.
//! Native menu actions deliberately add no webview IPC authority.
use crate::{
    probe_geometry::{fit, Rect},
    probe_schedule::{on_focus_lost, PendingOpen},
};
use objc2::rc::Retained;
use objc2_app_kit::{
    NSApplicationActivationOptions, NSRunningApplication, NSWindow, NSWindowCollectionBehavior,
    NSWorkspace,
};
use std::{cell::RefCell, time::Duration};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    window::{Effect, EffectState, EffectsBuilder},
    Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
thread_local! {
    static OPEN_REQUEST: RefCell<PendingOpen> = RefCell::new(PendingOpen::default());
    // Only accessed by the main-thread setup, menu and dispatched closures.
    static ORIGINAL: RefCell<Option<Retained<NSRunningApplication>>> = const { RefCell::new(None) };
}

fn panel(app: &tauri::AppHandle) -> Result<WebviewWindow> {
    app.get_webview_window("native-probe")
        .ok_or_else(|| "probe window unavailable".into())
}

fn geometry(window: &WebviewWindow, opening: bool, height: u32) -> Result<()> {
    let monitor = if opening {
        let cursor = window.cursor_position()?;
        window.monitor_from_point(cursor.x, cursor.y)?
    } else {
        window.current_monitor()?
    }
    .or(window.primary_monitor()?)
    .ok_or("no available display")?;
    let area = monitor.work_area();
    let scale = monitor.scale_factor();
    let position = window.outer_position()?;
    let size = window.outer_size()?;
    let target = fit(
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
    window.set_size(PhysicalSize::new(target.width, target.height))?;
    window.set_position(PhysicalPosition::new(target.x, target.y))?;
    eprintln!(
        "probe geometry: requested={target:?}; actual position={:?}; size={:?}; scale={scale}",
        window.outer_position()?,
        window.outer_size()?
    );
    Ok(())
}

fn open(app: &tauri::AppHandle) -> Result<()> {
    let window = panel(app)?;
    if window.is_visible()? {
        return Ok(());
    }
    ORIGINAL
        .with(|slot| *slot.borrow_mut() = NSWorkspace::sharedWorkspace().frontmostApplication());
    geometry(&window, true, 264)?;
    window.show()?;
    window.set_focus()?;
    window.eval("document.querySelectorAll('input').forEach(input => { input.value = ''; }); document.querySelector('input')?.focus()")?;
    eprintln!("probe opened; original application captured (identity not logged)");
    Ok(())
}

fn close(app: &tauri::AppHandle, restore: bool) -> Result<()> {
    OPEN_REQUEST.with(|pending| pending.borrow_mut().cancel());
    // Take the target before hiding: hide generates its own focus-loss event.
    let original = ORIGINAL.with(|slot| slot.borrow_mut().take());
    panel(app)?.hide()?;
    if restore {
        if let Some(original) = original.filter(|original| !original.isTerminated()) {
            if original.processIdentifier() == std::process::id() as i32 {
                // Activating an already-active application alone does not select
                // Workbench as the key window after the probe is hidden.
                app.get_webview_window("workbench")
                    .ok_or("Workbench unavailable for focus restoration")?
                    .set_focus()?;
                eprintln!("probe closed; Workbench focus requested");
            } else {
                #[allow(deprecated)]
                let accepted = original
                    .activateWithOptions(NSApplicationActivationOptions::ActivateIgnoringOtherApps);
                eprintln!("probe focus activation request accepted={accepted}; visually verify actual focus");
            }
        }
    }
    Ok(())
}

fn recover_display(app: &tauri::AppHandle) -> Result<()> {
    let window = panel(app)?;
    if !window.is_visible()? {
        return Ok(());
    }
    let position = window.outer_position()?;
    let size = window.outer_size()?;
    let contained = window.available_monitors()?.iter().any(|monitor| {
        let area = monitor.work_area();
        position.x >= area.position.x
            && position.y >= area.position.y
            && i64::from(position.x) + i64::from(size.width)
                <= i64::from(area.position.x) + i64::from(area.size.width)
            && i64::from(position.y) + i64::from(size.height)
                <= i64::from(area.position.y) + i64::from(area.size.height)
    });
    if !contained {
        let height = (f64::from(size.height) / window.scale_factor()?).round() as u32;
        geometry(&window, false, height)?;
        eprintln!("probe display recovery applied; verify controls remain reachable");
    }
    Ok(())
}

fn report(result: Result<()>) {
    if let Err(error) = result {
        eprintln!("PROBE FAILURE: {error}");
    }
}

fn probe_effects() -> tauri::utils::config::WindowEffectsConfig {
    EffectsBuilder::new()
        .effect(Effect::HudWindow)
        .state(EffectState::Active)
        .build()
}

#[derive(Debug)]
struct MaterialInspection {
    window_opaque: bool,
    opaque_content_views: usize,
    effect_count: usize,
    active_hud_behind_window: usize,
    reduce_transparency: bool,
}

fn inspect_material(app: &tauri::AppHandle) -> Result<MaterialInspection> {
    use objc2_app_kit::{
        NSView, NSVisualEffectBlendingMode, NSVisualEffectMaterial, NSVisualEffectState,
        NSVisualEffectView,
    };
    fn inspect(view: &NSView, result: &mut MaterialInspection) {
        let object: &objc2::runtime::AnyObject = view;
        if let Some(effect) = object.downcast_ref::<NSVisualEffectView>() {
            result.effect_count += 1;
            if effect.material() == NSVisualEffectMaterial::HUDWindow
                && effect.state() == NSVisualEffectState::Active
                && effect.blendingMode() == NSVisualEffectBlendingMode::BehindWindow
                && effect.frame().size.width > 0.0
                && effect.frame().size.height > 0.0
                && !effect.isHidden()
            {
                result.active_hud_behind_window += 1;
            }
        } else if view.isOpaque() {
            result.opaque_content_views += 1;
        }
        for child in view.subviews().iter() {
            inspect(&child, result);
        }
    }
    let window = panel(app)?;
    // SAFETY: called only on AppKit's main thread; the borrowed live NSWindow
    // and its view tree are inspected synchronously and never retained here.
    let native = unsafe { &*window.ns_window()?.cast::<NSWindow>() };
    let root = native.contentView().ok_or("no content view")?;
    let mut result = MaterialInspection {
        window_opaque: native.isOpaque(),
        opaque_content_views: 0,
        effect_count: 0,
        active_hud_behind_window: 0,
        reduce_transparency: NSWorkspace::sharedWorkspace()
            .accessibilityDisplayShouldReduceTransparency(),
    };
    inspect(&root, &mut result);
    eprintln!("probe material: {result:?}; native structure only, not visual blur acceptance");
    if result.reduce_transparency {
        eprintln!("probe material: system reduces transparency; structure does not imply visual translucency");
    }
    Ok(result)
}

pub fn setup(app: &mut tauri::App) -> Result<()> {
    let window = WebviewWindowBuilder::new(
        app,
        "native-probe",
        WebviewUrl::App("index.html?surface=native-probe".into()),
    )
    .title("Nibble — Native risk probe")
    .inner_size(650.0, 264.0)
    .visible(false)
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .resizable(false)
    .effects(probe_effects())
    .build()?;
    // SAFETY: setup runs on AppKit's main thread; ns_window returns this live
    // window's NSWindow pointer, borrowed only for this synchronous call.
    let native = unsafe { &*window.ns_window()?.cast::<NSWindow>() };
    native.setCollectionBehavior(
        NSWindowCollectionBehavior::MoveToActiveSpace
            | NSWindowCollectionBehavior::FullScreenAuxiliary,
    );
    let material_check = std::env::args()
        .find(|arg| arg == "--check-probe-material" || arg == "--check-probe-material-off");
    if let Some(mode) = material_check {
        let enabled = mode == "--check-probe-material";
        if !enabled {
            window.set_effects(None::<tauri::utils::config::WindowEffectsConfig>)?;
        }
        if let Some(workbench) = app.get_webview_window("workbench") {
            workbench.hide()?;
        }
        let app_handle = app.handle().clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(2));
            let handle = app_handle.clone();
            if let Err(error) = app_handle.run_on_main_thread(move || {
                let result = inspect_material(&handle);
                let expected = usize::from(enabled);
                let passed = result.as_ref().is_ok_and(|info| {
                    !info.window_opaque
                        && info.effect_count == expected
                        && info.active_hud_behind_window == expected
                });
                report(result.map(|_| ()));
                eprintln!("probe material structural check: enabled={enabled} passed={passed}");
                handle.exit(if passed { 0 } else { 2 });
            }) {
                eprintln!("PROBE FAILURE: {error}");
                app_handle.exit(2);
            }
        });
    }
    let items = [
        (
            "open",
            "Open after 3 seconds (switch to target app)",
            "CmdOrCtrl+1",
        ),
        (
            "material-on",
            "Material comparison: native HudWindow on",
            "CmdOrCtrl+5",
        ),
        (
            "material-off",
            "Material comparison: native effect off",
            "CmdOrCtrl+6",
        ),
        (
            "material-info",
            "Print native material diagnostics",
            "CmdOrCtrl+7",
        ),
        ("search", "Search size / focus input", "CmdOrCtrl+2"),
        ("history", "History size / focus input", "CmdOrCtrl+3"),
        ("base64", "Base64 size / focus input", "CmdOrCtrl+4"),
        ("restore", "Close and restore original app", "CmdOrCtrl+W"),
        ("escape", "Escape: close and restore original app", "Escape"),
        ("hide", "Close without restoring", "CmdOrCtrl+Shift+W"),
    ]
    .into_iter()
    .map(|(id, title, key)| MenuItem::with_id(app, id, title, true, Some(key)))
    .collect::<tauri::Result<Vec<_>>>()?;
    let submenu = Submenu::new(app, "Native probe", true)?;
    for item in &items {
        submenu.append(item)?;
    }
    submenu.append(&PredefinedMenuItem::quit(app, None)?)?;
    app.set_menu(Menu::with_items(app, &[&submenu])?)?;
    app.on_menu_event(|app, event| {
        match event.id().as_ref() {
            "open" => {
                let Some(request) = OPEN_REQUEST.with(|pending| pending.borrow_mut().request())
                else {
                    eprintln!("probe open already scheduled; keeping the first deadline");
                    return;
                };
                eprintln!("probe open scheduled in 3 seconds");
                let app = app.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_secs(3));
                    let handle = app.clone();
                    if let Err(error) = app.run_on_main_thread(move || {
                        if OPEN_REQUEST.with(|pending| pending.borrow_mut().complete(request)) {
                            report(open(&handle));
                        }
                    }) {
                        eprintln!("PROBE FAILURE: {error}");
                    }
                });
            }
            "material-on" => report((|| {
                panel(app)?.set_effects(probe_effects())?;
                eprintln!("probe material comparison: native HudWindow on; CSS unchanged");
                Ok(())
            })()),
            "material-off" => report((|| {
                panel(app)?.set_effects(None::<tauri::utils::config::WindowEffectsConfig>)?;
                eprintln!("probe material comparison: native effect off; CSS unchanged");
                Ok(())
            })()),
            "material-info" => report(inspect_material(app).map(|_| ())),
            "search" | "history" | "base64" => report((|| {
                let window = panel(app)?;
                let height = match event.id().as_ref() {
                    "history" => 480,
                    "base64" => 620,
                    _ => 264,
                };
                geometry(&window, false, height)?;
                // Constant script; no user content or additional IPC permissions.
                window.eval("document.querySelector('input')?.focus()")?;
                Ok(())
            })()),
            "restore" | "escape" => report(close(app, true)),
            "hide" => report(close(app, false)),
            _ => {}
        }
    });
    let handle = app.handle().clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::Focused(false) = event {
            report(on_focus_lost(
                || {
                    let window = panel(&handle)?;
                    Ok(window.is_visible()?)
                },
                || close(&handle, false),
            ));
        }
    });
    // Re-check live work areas while visible, including unplug/reconfiguration.
    // Main-thread dispatch keeps AppKit work and session state serialized.
    let app_handle = app.handle().clone();
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(1));
        let handle = app_handle.clone();
        if app_handle
            .run_on_main_thread(move || report(recover_display(&handle)))
            .is_err()
        {
            break;
        }
    });
    eprintln!("NATIVE RISK PROBE enabled; no clipboard, accessibility or recording access; menu Cmd+1 arms opening");
    Ok(())
}
