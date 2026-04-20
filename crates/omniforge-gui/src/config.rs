//! Loads and persists user-facing preferences under `XDG_CONFIG_HOME`.

use omniforge_common::error::OmniForgeError;
use std::path::PathBuf;

/// Runtime configuration snapshot for the GUI shell.
#[derive(Debug, Clone, Default)]
pub struct GuiConfig {
    /// Last opened workspace directory.
    pub last_workspace: Option<PathBuf>,
}

impl GuiConfig {
    /// Loads TOML configuration from disk when present.
    pub fn load() -> Result<Self, OmniForgeError> {
        Ok(Self::default())
    }
}
