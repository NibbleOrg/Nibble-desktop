//! Shortcut transaction rules, independent of windows and the filesystem.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    pub version: u32,
    pub shortcut: String,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            shortcut: "Alt+Space".into(),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum SettingsError {
    InvalidShortcut,
    RestartRequired,
    RegistrationFailed,
    SaveFailed,
    RecoveryFailed,
    ServiceUnavailable,
    ForbiddenWindow,
}
pub trait Backend {
    fn register(&mut self, key: &str) -> Result<(), ()>;
    fn unregister(&mut self, key: &str) -> Result<(), ()>;
    fn save(&mut self, config: &Config) -> Result<(), ()>;
}
pub struct Service {
    pub config: Config,
    pub available: bool,
    pub error: Option<SettingsError>,
    pub restart_required: bool,
}
impl Service {
    pub fn start(
        loaded: Result<Config, ()>,
        plugin_ready: bool,
        backend: &mut impl Backend,
    ) -> (Self, bool) {
        let recovered = loaded.is_err();
        let config = loaded.unwrap_or_default();
        let available = plugin_ready && backend.register(&config.shortcut).is_ok();
        (
            Self {
                config,
                available,
                restart_required: false,
                error: if !plugin_ready {
                    Some(SettingsError::ServiceUnavailable)
                } else if !available {
                    Some(SettingsError::RegistrationFailed)
                } else {
                    None
                },
            },
            recovered,
        )
    }

    pub fn change(&mut self, key: String, backend: &mut impl Backend) -> Result<(), SettingsError> {
        if self.restart_required {
            return Err(SettingsError::RestartRequired);
        }
        if self.available && key == self.config.shortcut {
            // Retry saving after an earlier recovery failure; never report a false success.
            return backend
                .save(&self.config)
                .map_err(|()| SettingsError::SaveFailed);
        }
        let old = self.config.clone();
        let was_available = self.available;
        if was_available && backend.unregister(&old.shortcut).is_err() {
            self.restart_required = true;
            self.available = false;
            return Err(SettingsError::RestartRequired);
        }
        self.available = false;
        if backend.register(&key).is_err() {
            self.available = was_available && backend.register(&old.shortcut).is_ok();
            return Err(if was_available && !self.available {
                SettingsError::RecoveryFailed
            } else {
                SettingsError::RegistrationFailed
            });
        }
        let next = Config {
            version: 1,
            shortcut: key,
        };
        if backend.save(&next).is_err() {
            let removed = backend.unregister(&next.shortcut).is_ok();
            if !removed {
                self.restart_required = true;
            }
            self.available = was_available && backend.register(&old.shortcut).is_ok();
            let restored = backend.save(&old).is_ok();
            return Err(if !removed {
                SettingsError::RestartRequired
            } else if restored && (!was_available || self.available) {
                SettingsError::SaveFailed
            } else {
                SettingsError::RecoveryFailed
            });
        }
        self.config = next;
        self.available = true;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    #[derive(Default)]
    struct Fake {
        failures: VecDeque<&'static str>,
        calls: Vec<String>,
        disk: Config,
    }
    impl Fake {
        fn op(&mut self, op: &str, key: &str) -> Result<(), ()> {
            self.calls.push(format!("{op}:{key}"));
            if self.failures.front() == Some(&op) {
                self.failures.pop_front();
                Err(())
            } else {
                Ok(())
            }
        }
    }
    impl Backend for Fake {
        fn register(&mut self, key: &str) -> Result<(), ()> {
            self.op("register", key)
        }
        fn unregister(&mut self, key: &str) -> Result<(), ()> {
            self.op("unregister", key)
        }
        fn save(&mut self, config: &Config) -> Result<(), ()> {
            self.op("save", &config.shortcut)?;
            self.disk = config.clone();
            Ok(())
        }
    }
    fn service() -> Service {
        Service {
            config: Config::default(),
            available: true,
            error: None,
            restart_required: false,
        }
    }
    #[test]
    fn success_commits_only_after_registration_and_save() {
        let mut service = service();
        let mut backend = Fake::default();
        service
            .change("Control+Space".into(), &mut backend)
            .unwrap();
        assert_eq!(service.config, backend.disk);
        assert_eq!(
            backend.calls,
            [
                "unregister:Alt+Space",
                "register:Control+Space",
                "save:Control+Space"
            ]
        );
    }
    #[test]
    fn registration_failure_keeps_old_configuration() {
        let mut service = service();
        let mut backend = Fake {
            failures: ["register"].into(),
            ..Default::default()
        };
        assert_eq!(
            service.change("Control+Space".into(), &mut backend),
            Err(SettingsError::RegistrationFailed)
        );
        assert!(service.available);
        assert_eq!(service.config, Config::default());
    }
    #[test]
    fn save_failure_restores_registration_and_disk() {
        let mut service = service();
        let mut backend = Fake {
            failures: ["save"].into(),
            ..Default::default()
        };
        assert_eq!(
            service.change("Control+Space".into(), &mut backend),
            Err(SettingsError::SaveFailed)
        );
        assert!(service.available);
        assert_eq!(backend.disk, Config::default());
        assert_eq!(backend.calls.len(), 6);
    }
    #[test]
    fn failed_restore_is_explicit() {
        let mut service = service();
        let mut backend = Fake {
            failures: ["register", "register"].into(),
            ..Default::default()
        };
        assert_eq!(
            service.change("Control+Space".into(), &mut backend),
            Err(SettingsError::RecoveryFailed)
        );
        assert!(!service.available);
    }
    #[test]
    fn unregister_failure_disables_dispatch_and_requires_restart() {
        let mut service = service();
        let mut backend = Fake {
            failures: ["unregister"].into(),
            ..Default::default()
        };
        assert_eq!(
            service.change("Control+Space".into(), &mut backend),
            Err(SettingsError::RestartRequired)
        );
        assert!(!service.available);
        assert!(service.restart_required);
        assert_eq!(service.config, Config::default());
        assert_eq!(backend.calls.len(), 1);
    }
    #[test]
    fn failed_new_key_removal_cannot_claim_retry_success() {
        let mut service = service();
        let mut backend = Fake {
            failures: ["save", "unregister"].into(),
            ..Default::default()
        };
        assert_eq!(
            service.change("Control+Space".into(), &mut backend),
            Err(SettingsError::RestartRequired)
        );
        assert!(service.restart_required);
        assert!(service.available);
        let count = backend.calls.len();
        assert_eq!(
            service.change("Super+Space".into(), &mut backend),
            Err(SettingsError::RestartRequired)
        );
        assert_eq!(backend.calls.len(), count);
    }
    #[test]
    fn failed_disk_restore_never_reports_success() {
        let mut service = service();
        let mut backend = Fake {
            failures: ["save", "save"].into(),
            ..Default::default()
        };
        assert_eq!(
            service.change("Control+Space".into(), &mut backend),
            Err(SettingsError::RecoveryFailed)
        );
        assert!(service.available);
        service.change("Alt+Space".into(), &mut backend).unwrap();
        assert_eq!(backend.disk, service.config);
    }
    #[test]
    fn unavailable_old_key_can_be_recovered_with_new_key() {
        let mut service = service();
        service.available = false;
        let mut backend = Fake::default();
        service
            .change("Control+Space".into(), &mut backend)
            .unwrap();
        assert!(service.available);
        assert_eq!(
            backend.calls,
            ["register:Control+Space", "save:Control+Space"]
        );
    }
    #[test]
    fn corrupt_startup_tries_default_without_overwriting_disk() {
        let mut backend = Fake::default();
        let (service, recovered) = Service::start(Err(()), true, &mut backend);
        assert!(recovered);
        assert!(service.available);
        assert_eq!(backend.calls, ["register:Alt+Space"]);
    }
    #[test]
    fn startup_conflict_preserves_config_and_reports_unavailable() {
        let mut backend = Fake {
            failures: ["register"].into(),
            ..Default::default()
        };
        let config = Config {
            version: 1,
            shortcut: "Control+Space".into(),
        };
        let (service, recovered) = Service::start(Ok(config.clone()), true, &mut backend);
        assert!(!recovered);
        assert!(!service.available);
        assert_eq!(service.config, config);
        assert_eq!(service.error, Some(SettingsError::RegistrationFailed));
        assert_eq!(backend.calls, ["register:Control+Space"]);
    }
    #[test]
    fn registration_restore_can_fail_after_new_key_was_saved_unsuccessfully() {
        let mut service = service();
        let mut backend = Fake {
            failures: ["save", "register"].into(),
            ..Default::default()
        };
        assert_eq!(
            service.change("Control+Space".into(), &mut backend),
            Err(SettingsError::RecoveryFailed)
        );
        assert!(!service.available);
        assert_eq!(backend.disk, Config::default());
        service
            .change("Control+Space".into(), &mut backend)
            .unwrap();
        assert!(service.available);
        assert_eq!(backend.disk, service.config);
    }
}
