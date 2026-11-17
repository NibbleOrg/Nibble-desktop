//! Main-thread settings adapter; all changes and startup are serialized here.
use crate::{
    settings_store::{normalize, Store},
    shortcut_settings::{Backend, Config, Service, SettingsError},
};
use std::cell::RefCell;
use tauri::{Manager, WebviewWindow};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
thread_local! { static STATE: RefCell<Option<Service>> = const { RefCell::new(None) }; }
#[derive(Clone, Debug, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SettingsSnapshot {
    pub version: u32,
    pub shortcut: String,
    pub available: bool,
    pub error: Option<SettingsError>,
    pub startup_recovered: bool,
    pub app_version: String,
}
thread_local! { static RECOVERED: RefCell<bool> = const { RefCell::new(false) }; }
struct Adapter<'a>(&'a tauri::AppHandle);
impl Backend for Adapter<'_> {
    fn register(&mut self, key: &str) -> Result<(), ()> {
        self.0.global_shortcut().register(key).map_err(|_| ())
    }
    fn unregister(&mut self, key: &str) -> Result<(), ()> {
        self.0.global_shortcut().unregister(key).map_err(|_| ())
    }
    fn save(&mut self, config: &Config) -> Result<(), ()> {
        store(self.0)?.save(config)
    }
}
fn store(app: &tauri::AppHandle) -> Result<Store, ()> {
    Ok(Store(
        app.path()
            .app_config_dir()
            .map_err(|_| ())?
            .join("settings.json"),
    ))
}
pub fn initialize(app: &tauri::AppHandle, plugin_ready: bool) {
    let loaded = store(app).and_then(|store| store.load());
    let (service, recovered) = Service::start(loaded, plugin_ready, &mut Adapter(app));
    RECOVERED.with(|state| *state.borrow_mut() = recovered);
    STATE.with(|state| *state.borrow_mut() = Some(service));
    sync(app);
}
fn sync(app: &tauri::AppHandle) {
    STATE.with(|state| {
        if let Some(service) = state.borrow().as_ref() {
            crate::lifecycle::shortcut_status(app, service.available, service.error.is_some());
        }
    });
}
pub fn handles(key: &Shortcut) -> bool {
    STATE.with(|state| {
        state.borrow().as_ref().is_some_and(|service| {
            service.available
                && service
                    .config
                    .shortcut
                    .parse::<Shortcut>()
                    .is_ok_and(|active| active == *key)
        })
    })
}
fn authorize(label: &str) -> Result<(), SettingsError> {
    if label == "workbench" {
        Ok(())
    } else {
        Err(SettingsError::ForbiddenWindow)
    }
}
#[tauri::command]
#[specta::specta]
pub fn settings_snapshot(window: WebviewWindow) -> Result<SettingsSnapshot, SettingsError> {
    authorize(window.label())?;
    snapshot()
}
fn snapshot() -> Result<SettingsSnapshot, SettingsError> {
    STATE.with(|state| {
        let state = state.borrow();
        let service = state.as_ref().ok_or(SettingsError::ServiceUnavailable)?;
        Ok(SettingsSnapshot {
            version: crate::lifecycle::snapshot().version,
            shortcut: service.config.shortcut.clone(),
            available: service.available,
            error: service.error,
            startup_recovered: RECOVERED.with(|state| *state.borrow()),
            app_version: env!("CARGO_PKG_VERSION").into(),
        })
    })
}
#[tauri::command]
#[specta::specta]
pub fn settings_save(
    window: WebviewWindow,
    shortcut: String,
) -> Result<SettingsSnapshot, SettingsError> {
    authorize(window.label())?;
    let key = normalize(&shortcut).map_err(|()| SettingsError::InvalidShortcut)?;
    let result = STATE.with(|state| {
        let mut state = state.borrow_mut();
        let service = state.as_mut().ok_or(SettingsError::ServiceUnavailable)?;
        if service.error == Some(SettingsError::ServiceUnavailable) {
            return Err(SettingsError::ServiceUnavailable);
        }
        let result = service.change(key, &mut Adapter(window.app_handle()));
        let result = if result.is_err()
            && result != Err(SettingsError::RestartRequired)
            && service.error == Some(SettingsError::RecoveryFailed)
        {
            Err(SettingsError::RecoveryFailed)
        } else {
            result
        };
        service.error = result.err();
        result
    });
    sync(window.app_handle());
    result?;
    RECOVERED.with(|state| *state.borrow_mut() = false);
    snapshot()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn settings_are_workbench_only() {
        assert!(authorize("workbench").is_ok());
        for label in ["quick-panel", "native-probe", "", "remote"] {
            assert_eq!(authorize(label), Err(SettingsError::ForbiddenWindow));
        }
    }
    #[test]
    fn capability_grants_settings_only_to_workbench() {
        let workbench: serde_json::Value =
            serde_json::from_str(include_str!("../capabilities/lifecycle-workbench.json")).unwrap();
        let panel: serde_json::Value =
            serde_json::from_str(include_str!("../capabilities/lifecycle-panel.json")).unwrap();
        for permission in ["allow-settings-save", "allow-settings-snapshot"] {
            assert!(workbench["permissions"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!(permission)));
            assert!(!panel["permissions"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!(permission)));
        }
        assert_eq!(workbench["windows"], serde_json::json!(["workbench"]));
    }
}
