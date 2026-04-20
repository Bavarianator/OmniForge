//! Python integration layer. Heavy training runs in a subprocess; optional PyO3 hooks stay isolated here.

use omniforge_common::error::OmniForgeError;
use std::path::{Path, PathBuf};
use tokio::process::Command;

/// Locates the bundled training script next to the development tree or install prefix.
pub fn train_script_path() -> PathBuf {
    if let Ok(root) = std::env::var("OMNIFORGE_ROOT") {
        return PathBuf::from(root).join("python").join("train_lora.py");
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("python")
        .join("train_lora.py")
}

/// Builds the `tokio::process::Command` used to launch LoRA training.
///
/// PyO3-based in-process execution remains intentionally out of scope for the subprocess
/// path to keep CUDA aborts isolated from the GTK main loop.
pub fn build_train_command(
    python_executable: &Path,
    script: &Path,
    workspace: &Path,
) -> Command {
    let mut cmd = Command::new(python_executable);
    cmd.arg(script)
        .current_dir(workspace)
        .kill_on_drop(true);
    cmd
}

/// Resolves the interpreter inside the embedded venv when packaged, else falls back to `python3`.
pub fn resolve_python_executable() -> PathBuf {
    if let Ok(p) = std::env::var("OMNIFORGE_PYTHON") {
        return PathBuf::from(p);
    }
    PathBuf::from("python3")
}

/// Ensures the training entrypoint exists on disk before spawning.
pub fn ensure_train_script_present() -> Result<PathBuf, OmniForgeError> {
    let script = train_script_path();
    if script.exists() {
        Ok(script)
    } else {
        Err(OmniForgeError::Training {
            message: format!("train_lora.py not found at {}", script.display()),
        })
    }
}
