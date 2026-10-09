use serde::{Deserialize, Serialize};
use std::{io, path::PathBuf};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub memory_saver: bool,
    pub save_passwords: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            memory_saver: false,
            save_passwords: true,
        }
    }
}

pub fn data_dir() -> PathBuf {
    std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("Kiniwolf")
}

impl Settings {
    pub fn load() -> Self {
        std::fs::read(data_dir().join("settings.json"))
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> io::Result<()> {
        std::fs::create_dir_all(data_dir())?;
        std::fs::write(
            data_dir().join("settings.json"),
            serde_json::to_vec_pretty(self)?,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Settings;

    #[test]
    fn older_settings_preserve_defaults_for_new_options() {
        let settings: Settings = serde_json::from_str(r#"{"memory_saver":true}"#).unwrap();
        assert!(settings.memory_saver);
        assert!(settings.save_passwords);
    }

    #[test]
    fn preferences_survive_serialization() {
        let original = Settings {
            memory_saver: true,
            save_passwords: false,
        };
        let restored: Settings =
            serde_json::from_slice(&serde_json::to_vec(&original).unwrap()).unwrap();
        assert!(restored.memory_saver);
        assert!(!restored.save_passwords);
    }
}
