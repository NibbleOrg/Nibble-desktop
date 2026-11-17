//! ACL plus runtime caller validation: a webview cannot supply its own identity.
use crate::lifecycle::{self, LifecycleError, Snapshot, View};
use tauri::{Manager, WebviewWindow};
fn authorize(label: &str, panel_only: bool) -> Result<(), LifecycleError> {
    if label == "quick-panel" || (!panel_only && label == "workbench") {
        Ok(())
    } else {
        Err(LifecycleError::ForbiddenWindow)
    }
}
fn current(session: u32) -> Result<(), LifecycleError> {
    if lifecycle::snapshot().session == Some(session) {
        Ok(())
    } else {
        Err(LifecycleError::StaleSession)
    }
}
fn authorize_workbench(label: &str) -> Result<(), LifecycleError> {
    if label == "workbench" {
        Ok(())
    } else {
        Err(LifecycleError::ForbiddenWindow)
    }
}
#[tauri::command]
#[specta::specta]
pub fn calendar_preferences(
    window: WebviewWindow,
) -> Result<crate::calendar_preferences::CalendarPreferences, LifecycleError> {
    authorize_workbench(window.label())?;
    Ok(crate::calendar_preferences::read())
}
#[tauri::command]
#[specta::specta]
pub fn lifecycle_snapshot(window: WebviewWindow) -> Result<Snapshot, LifecycleError> {
    authorize(window.label(), false)?;
    Ok(lifecycle::snapshot())
}
#[tauri::command]
#[specta::specta]
pub fn panel_close(window: WebviewWindow, session: u32) -> Result<(), LifecycleError> {
    authorize(window.label(), true)?;
    current(session)?;
    lifecycle::close(window.app_handle(), true)
}
#[tauri::command]
#[specta::specta]
pub fn panel_transition(
    window: WebviewWindow,
    session: u32,
    view: View,
) -> Result<Snapshot, LifecycleError> {
    authorize(window.label(), true)?;
    current(session)?;
    let previous = lifecycle::snapshot().view;
    if previous != View::Search && view != View::Search {
        return Err(LifecycleError::ReturnToSearchFirst);
    }
    lifecycle::transition(window.app_handle(), session, view)
}
#[tauri::command]
#[specta::specta]
pub fn panel_transition_complete(
    window: WebviewWindow,
    session: u32,
    request: u32,
) -> Result<(), LifecycleError> {
    authorize(window.label(), true)?;
    current(session)?;
    lifecycle::complete(window.app_handle(), session, request)
}
#[tauri::command]
#[specta::specta]
pub fn panel_settings(window: WebviewWindow, session: u32) -> Result<(), LifecycleError> {
    authorize(window.label(), true)?;
    current(session)?;
    lifecycle::workbench(window.app_handle(), true)
}
#[tauri::command]
#[specta::specta]
pub fn settings_acknowledged(window: WebviewWindow) -> Result<(), LifecycleError> {
    authorize_workbench(window.label())?;
    lifecycle::acknowledge_settings(window.app_handle());
    Ok(())
}
#[tauri::command]
#[specta::specta]
pub fn panel_toggle_demo_pause(window: WebviewWindow, session: u32) -> Result<(), LifecycleError> {
    authorize(window.label(), true)?;
    current(session)?;
    lifecycle::toggle_demo_pause(window.app_handle())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn caller_identity_matrix() {
        assert!(authorize_workbench("workbench").is_ok());
        for label in ["quick-panel", "native-probe", "", "remote"] {
            assert!(authorize_workbench(label).is_err());
        }
        assert!(authorize("quick-panel", true).is_ok());
        assert!(authorize("workbench", false).is_ok());
        assert!(authorize("workbench", true).is_err());
        for label in ["native-probe", "", "remote", "quick-panel-2"] {
            assert!(authorize(label, false).is_err());
        }
    }
}
