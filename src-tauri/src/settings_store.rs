//! App-owned, versioned JSON. No frontend filesystem permissions.
use crate::shortcut_settings::Config;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};
pub fn normalize(input: &str) -> Result<String, ()> {
    if input.len() > 128 {
        return Err(());
    }
    let key: Shortcut = input.parse().map_err(|_| ())?;
    // macOS media keys use a shared event tap, not exclusive Carbon registration.
    if matches!(
        key.key,
        Code::MediaPlayPause
            | Code::MediaTrackNext
            | Code::MediaTrackPrevious
            | Code::MediaFastForward
            | Code::MediaRewind
    ) {
        return Err(());
    }
    if !key
        .mods
        .intersects(Modifiers::ALT | Modifiers::CONTROL | Modifiers::SUPER)
    {
        return Err(());
    }
    Ok(key.to_string())
}
pub struct Store(pub PathBuf);
impl Store {
    pub fn load(&self) -> Result<Config, ()> {
        let mut file = match File::open(&self.0) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Config::default())
            }
            Err(_) => return Err(()),
        };
        let mut bytes = Vec::new();
        Read::by_ref(&mut file)
            .take(4097)
            .read_to_end(&mut bytes)
            .map_err(|_| ())?;
        if bytes.len() > 4096 {
            return Err(());
        }
        let mut config: Config = serde_json::from_slice(&bytes).map_err(|_| ())?;
        if config.version != 1 {
            return Err(());
        }
        config.shortcut = normalize(&config.shortcut)?;
        Ok(config)
    }
    pub fn save(&self, config: &Config) -> Result<(), ()> {
        let parent = self.0.parent().ok_or(())?;
        fs::create_dir_all(parent).map_err(|_| ())?;
        let temporary = self.0.with_extension("json.tmp");
        // Writes are serialized on the main thread. A stale temporary file from
        // a crash can be removed, but never follow a pre-existing symlink.
        match fs::remove_file(&temporary) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => return Err(()),
        }
        let result = (|| {
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options.open(&temporary).map_err(|_| ())?;
            let bytes = serde_json::to_vec(config).map_err(|_| ())?;
            file.write_all(&bytes).map_err(|_| ())?;
            file.sync_all().map_err(|_| ())?;
            fs::rename(&temporary, &self.0).map_err(|_| ())?;
            File::open(parent)
                .and_then(|directory| directory.sync_all())
                .map_err(|_| ())
        })();
        let _ = fs::remove_file(temporary);
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn store() -> Store {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        Store(
            std::env::temp_dir()
                .join(format!(
                    "nibble-settings-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                ))
                .join("settings.json"),
        )
    }
    #[test]
    fn restart_reads_saved_shortcut_and_rejects_bad_formats() {
        let store = store();
        let config = Config {
            version: 1,
            shortcut: normalize("Control+Space").unwrap(),
        };
        assert_eq!(store.load(), Ok(Config::default()));
        store.save(&config).unwrap();
        assert_eq!(Store(store.0.clone()).load(), Ok(config));
        for bytes in [
            "{",
            "{\"version\":2,\"shortcut\":\"Alt+Space\"}",
            "{\"version\":1,\"shortcut\":\"Space\"}",
            "{\"version\":1,\"shortcut\":\"Alt+Space\",\"capture\":true}",
        ] {
            fs::write(&store.0, bytes).unwrap();
            assert!(store.load().is_err());
        }
        fs::remove_dir_all(store.0.parent().unwrap()).unwrap();
    }
    #[test]
    fn write_failure_preserves_existing_file() {
        let store = store();
        store.save(&Config::default()).unwrap();
        fs::create_dir(store.0.with_extension("json.tmp")).unwrap();
        assert!(store
            .save(&Config {
                version: 1,
                shortcut: "Control+Space".into()
            })
            .is_err());
        assert_eq!(
            store.load().unwrap().shortcut,
            normalize("Alt+Space").unwrap()
        );
        fs::remove_dir_all(store.0.parent().unwrap()).unwrap();
    }
    #[test]
    fn validation_requires_non_shift_modifier_and_valid_key() {
        for key in [
            "",
            "Space",
            "Shift+Space",
            "Alt+Bogus",
            "Alt+Space+KeyA",
            "Alt+MediaPlayPause",
            "Control+MediaTrackNext",
        ] {
            assert!(normalize(key).is_err(), "{key}");
        }
        assert_eq!(normalize("Alt+Space"), normalize("Option+Space"));
    }
}
