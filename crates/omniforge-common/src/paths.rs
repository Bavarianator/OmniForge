//! XDG-based directory layout for OmniForge (config, data, cache).

use crate::error::OmniForgeError;
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

const QUALIFIER: &str = "";
const ORG: &str = "";
const APP: &str = "omniforge";

fn project_dirs() -> Result<ProjectDirs, OmniForgeError> {
    ProjectDirs::from(QUALIFIER, ORG, APP).ok_or_else(|| OmniForgeError::Config {
        message: "unable to resolve XDG project directories".to_string(),
    })
}

/// Returns the configuration directory (`XDG_CONFIG_HOME/omniforge`).
pub fn config_dir() -> Result<PathBuf, OmniForgeError> {
    Ok(project_dirs()?.config_dir().to_path_buf())
}

/// Returns the data directory (`XDG_DATA_HOME/omniforge`).
pub fn data_dir() -> Result<PathBuf, OmniForgeError> {
    Ok(project_dirs()?.data_dir().to_path_buf())
}

/// Returns the cache directory (`XDG_CACHE_HOME/omniforge`).
pub fn cache_dir() -> Result<PathBuf, OmniForgeError> {
    Ok(project_dirs()?.cache_dir().to_path_buf())
}

/// Ensures a directory exists, creating parents as needed.
#[allow(dead_code)]
pub fn ensure_dir(path: &Path) -> Result<(), OmniForgeError> {
    std::fs::create_dir_all(path)?;
    Ok(())
}
