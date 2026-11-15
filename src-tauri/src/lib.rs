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
    tauri_specta::Builder::new().commands(tauri_specta::collect_commands![bootstrap_info])
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(command_builder().invoke_handler())
        .run(tauri::generate_context!())
        .expect("application startup failed");
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
