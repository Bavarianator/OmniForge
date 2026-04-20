//! Central error type for all OmniForge crates.

use thiserror::Error;

/// Top-level error returned by fallible OmniForge operations.
#[derive(Debug, Error)]
pub enum OmniForgeError {
    /// Filesystem or OS-level failures.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration load/save problems.
    #[error("configuration error: {message}")]
    Config {
        /// Human-readable explanation for operators and UI copy.
        message: String,
    },

    /// Model download, format, or path issues.
    #[error("model error: {message}")]
    Model {
        /// Human-readable explanation for operators and UI copy.
        message: String,
    },

    /// Training orchestration or Python bridge failures.
    #[error("training error: {message}")]
    Training {
        /// Human-readable explanation for operators and UI copy.
        message: String,
    },

    /// RAG indexing, embedding, or retrieval failures.
    #[error("RAG error: {message}")]
    Rag {
        /// Human-readable explanation for operators and UI copy.
        message: String,
    },

    /// GPU / driver / capability detection issues.
    #[error("hardware error: {message}")]
    Hardware {
        /// Human-readable explanation for operators and UI copy.
        message: String,
    },
}
