//! Run manually in a macOS GUI session; no synthetic keys or user config writes.
#[allow(dead_code)]
#[path = "../src/shortcut_settings.rs"]
mod shortcut_settings;
use global_hotkey::{hotkey::HotKey, GlobalHotKeyManager};
use shortcut_settings::{Backend, Config, Service, SettingsError};
use std::io::{BufRead, Write};
struct Native {
    manager: GlobalHotKeyManager,
    disk: Config,
}
impl Backend for Native {
    fn register(&mut self, key: &str) -> Result<(), ()> {
        self.manager
            .register(key.parse().map_err(|_| ())?)
            .map_err(|_| ())
    }
    fn unregister(&mut self, key: &str) -> Result<(), ()> {
        self.manager
            .unregister(key.parse().map_err(|_| ())?)
            .map_err(|_| ())
    }
    fn save(&mut self, config: &Config) -> Result<(), ()> {
        self.disk = config.clone();
        Ok(())
    }
}
fn main() {
    let key: HotKey = "Control+Shift+F18".parse().unwrap();
    if std::env::args().any(|arg| arg == "--hold") {
        let manager = GlobalHotKeyManager::new().expect("GUI session required");
        manager.register(key).expect("test key already occupied");
        println!("ready");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        manager.unregister(key).unwrap();
        return;
    }
    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--hold")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut ready = String::new();
        std::io::BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut ready)
            .unwrap();
        assert_eq!(ready.trim(), "ready", "holder failed");
        let old = Config {
            version: 1,
            shortcut: "Control+Shift+F19".into(),
        };
        let mut native = Native {
            manager: GlobalHotKeyManager::new().unwrap(),
            disk: old.clone(),
        };
        let (mut service, _) = Service::start(Ok(old.clone()), true, &mut native);
        assert!(service.available, "old test key already occupied");
        let result = service.change("Control+Shift+F18".into(), &mut native);
        assert_eq!(
            result,
            Err(SettingsError::RegistrationFailed),
            "occupied key must be rejected"
        );
        assert!(service.available, "old registration must be restored");
        assert_eq!(service.config, old);
        assert_eq!(native.disk, old, "failed registration must not save");
        native.unregister(&old.shortcut).unwrap();
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(b"\n").unwrap();
        drop(stdin);
        assert!(child.wait().unwrap().success());
        native
            .manager
            .register(key)
            .expect("released candidate must become available");
        native.manager.unregister(key).unwrap();
        println!("PASS: independent owner rejected; old registration/config restored; released key reusable");
    }));
    if result.is_err() {
        let _ = child.kill();
        let _ = child.wait();
        std::process::exit(1);
    }
}
