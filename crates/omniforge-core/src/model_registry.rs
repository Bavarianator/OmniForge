//! Tracks locally available base models, adapters, and export artifacts.

use omniforge_common::error::OmniForgeError;
use std::path::PathBuf;

/// Lists known GGUF or HF checkpoints under the XDG data directory.
pub fn list_local_models() -> Result<Vec<PathBuf>, OmniForgeError> {
    Ok(Vec::new())
}
