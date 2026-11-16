fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "bootstrap_info",
            "lifecycle_snapshot",
            "panel_close",
            "panel_transition",
            "panel_transition_complete",
            "panel_settings",
            "panel_toggle_demo_pause",
            "settings_acknowledged",
        ]),
    ))
    .expect("Tauri build configuration failed");
}
