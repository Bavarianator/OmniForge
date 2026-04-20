//! GGUF inference engine facade.

use crate::inference::config::InferenceParams;
use omniforge_common::error::OmniForgeError;

/// Abstracts llama.cpp-backed streaming generation.
pub struct InferenceEngine;

impl InferenceEngine {
    /// Runs a single user turn; streaming hooks will be added with GTK integration.
    pub async fn complete(
        _prompt: &str,
        _params: &InferenceParams,
    ) -> Result<String, OmniForgeError> {
        Err(OmniForgeError::Model {
            message: "inference engine not yet wired to llama.cpp".to_string(),
        })
    }
}
