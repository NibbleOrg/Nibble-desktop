#[cfg(target_os = "macos")]
mod calendar_preferences;
#[cfg(target_os = "macos")]
mod lifecycle;
#[cfg(target_os = "macos")]
mod lifecycle_commands;
#[cfg(all(feature = "native-probe", target_os = "macos"))]
mod native_probe;
mod panel_session;
#[cfg(any(test, all(feature = "native-probe", target_os = "macos")))]
mod probe_schedule;
mod settings;
mod settings_store;
mod shortcut_settings;
#[cfg(any(test, target_os = "macos"))]
mod window_geometry;
#[derive(Debug, PartialEq, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapInfo {
    pub version: String,
    pub stage: String,
}

#[tauri::command]
#[specta::specta]
fn bootstrap_info() -> BootstrapInfo {
    BootstrapInfo {
        version: env!("CARGO_PKG_VERSION").into(),
        stage: "bootstrap".into(),
    }
}

pub fn command_builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::new().commands(tauri_specta::collect_commands![
        bootstrap_info,
        settings::settings_snapshot,
        settings::settings_save,
        lifecycle_commands::calendar_preferences,
        lifecycle_commands::lifecycle_snapshot,
        lifecycle_commands::panel_close,
        lifecycle_commands::panel_transition,
        lifecycle_commands::panel_transition_complete,
        lifecycle_commands::panel_settings,
        lifecycle_commands::panel_toggle_demo_pause,
        lifecycle_commands::settings_acknowledged
    ])
}

pub fn run() {
    let builder = tauri::Builder::default()
        // Single-instance must be the first plugin, before any service/window setup.
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let handle = app.clone();
            let _ = app.run_on_main_thread(move || {
                let _ = lifecycle::workbench(&handle, false);
            });
        }))
        .setup(|app| {
            #[cfg(not(feature = "native-probe"))]
            lifecycle::setup(app)?;
            #[cfg(all(feature = "native-probe", target_os = "macos"))]
            native_probe::setup(app)?;
            Ok(())
        });
    builder
        .invoke_handler(command_builder().invoke_handler())
        .build(tauri::generate_context!())
        .expect("application startup failed")
        .run(|app, event| {
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Reopen { .. } = event {
                let _ = lifecycle::workbench(app, false);
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_exposes_only_bootstrap_identity() {
        let info = bootstrap_info();
        assert_eq!(info.stage, "bootstrap");
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
    }
}
