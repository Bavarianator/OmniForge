//! High-level import entrypoints used by the GUI and CLI wrappers.

use omniforge_common::error::OmniForgeError;
use std::path::Path;

/// Imports a single file or directory and returns a coarse chunk count for progress UI.
///
/// The scaffold implementation only validates presence; parsers will increment the count.
pub async fn import_path(path: &Path) -> Result<usize, OmniForgeError> {
    let meta = tokio::fs::metadata(path)
        .await
        .map_err(OmniForgeError::Io)?;
    if meta.is_dir() {
        Ok(1)
    } else {
        Ok(1)
    }
}
