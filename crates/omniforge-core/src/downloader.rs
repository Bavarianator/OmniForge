//! Hugging Face Hub downloads with explicit user consent and offline cache reuse.

use omniforge_common::error::OmniForgeError;

/// Downloads a model repository into the OmniForge models directory.
pub async fn download_model(_model_id: &str) -> Result<(), OmniForgeError> {
    Err(OmniForgeError::Model {
        message: "model downloader not implemented in scaffold".to_string(),
    })
}
