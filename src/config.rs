use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneralConfig {
    #[serde(default = "default_vivado_version")]
    pub vivado_version: String,
}

fn default_vivado_version() -> String { "2026.1".into() }

impl Default for Config {
    fn default() -> Self {
        Config {
            general: GeneralConfig::default(),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Read { path: PathBuf, source: std::io::Error },
    Parse(toml::de::Error),
    AlreadyInitialized,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Read { path, source } => {
                write!(f, "failed to read config file at {}: {}", path.display(), source)
            }
            ConfigError::Parse(e) => write!(f, "failed to parse config TOML: {e}"),
            ConfigError::AlreadyInitialized => write!(f, "config already initialized"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Read { source, .. } => Some(source),
            ConfigError::Parse(e) => Some(e),
            ConfigError::AlreadyInitialized => None,
        }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::Parse(e)
    }
}

// --- Global singleton ---

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    /// Call once in `main`. Loads from `path`, falling back to defaults
    /// if the file doesn't exist, and stores the result in the global.
    /// Errors if the file exists but is malformed, or if called twice.
    pub fn init(path: impl AsRef<Path>) -> Result<(), ConfigError> {
        let path = path.as_ref();

        let config = if path.exists() {
            let contents = std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
                path: path.to_path_buf(),
                source,
            })?;
            toml::from_str(&contents)?
        } else {
            Config::default()
        };

        CONFIG.set(config).map_err(|_| ConfigError::AlreadyInitialized)
    }

    /// Access from anywhere — panels, widgets, async tasks.
    pub fn get() -> &'static Config {
        CONFIG.get().expect("Config accessed before Config::init()")
    }

    pub fn try_get() -> Option<&'static Config> {
        CONFIG.get()
    }
}
