//! Throwaway macOS risk harness, compiled only with `native-probe`.
//! Native menu actions deliberately add no webview IPC authority.
use crate::probe_geometry::{fit, Rect};
use objc2::rc::Retained;
use objc2_app_kit::{
    NSApplicationActivationOptions, NSRunningApplication, NSWindow, NSWindowCollectionBehavior,
    NSWorkspace,
};
use std::{
    cell::RefCell,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    window::{Effect, EffectState, EffectsBuilder},
    Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
};

static OPEN_REQUEST: AtomicU64 = AtomicU64::new(0);

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
thread_local! {
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
    OPEN_REQUEST.fetch_add(1, Ordering::SeqCst);
    panel(app)?.hide()?;
    ORIGINAL.with(|slot| {
        if let Some(original) = slot.borrow_mut().take() {
            if restore && !original.isTerminated() {
                // One request only; deprecated API is intentionally being evaluated,
                // not treated as a guarantee that macOS grants activation.
                #[allow(deprecated)]
                let accepted = original.activateWithOptions(NSApplicationActivationOptions::ActivateIgnoringOtherApps);
                eprintln!("probe focus activation request accepted={accepted}; visually verify actual focus");
            }
        }
    });
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
    .effects(
        EffectsBuilder::new()
            .effect(Effect::HudWindow)
            .state(EffectState::Active)
            .build(),
    )
    .build()?;
    // SAFETY: setup runs on AppKit's main thread; ns_window returns this live
    // window's NSWindow pointer, borrowed only for this synchronous call.
    let native = unsafe { &*window.ns_window()?.cast::<NSWindow>() };
    native.setCollectionBehavior(
        NSWindowCollectionBehavior::MoveToActiveSpace
            | NSWindowCollectionBehavior::FullScreenAuxiliary,
    );
    let items = [
        (
            "open",
            "Open after 3 seconds (switch to target app)",
            "CmdOrCtrl+1",
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
                let request = OPEN_REQUEST.fetch_add(1, Ordering::SeqCst) + 1;
                let app = app.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_secs(3));
                    let handle = app.clone();
                    if let Err(error) = app.run_on_main_thread(move || {
                        if OPEN_REQUEST.load(Ordering::SeqCst) == request {
                            report(open(&handle));
                        }
                    }) {
                        eprintln!("PROBE FAILURE: {error}");
                    }
                });
            }
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
            report(close(&handle, false));
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
