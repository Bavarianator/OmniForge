//! IPC-style messages between the GTK/Relm4 frontend and the async core orchestrator.

use crate::types::{ExportConfig, ExportFormat, GpuInfo, TrainingConfig};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Events emitted from the backend toward the GUI (training metrics, imports, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackendEvent {
    /// Hardware discovery finished; drives recommendation cards in the dashboard.
    HardwareDetected {
        /// Primary GPU info when available.
        gpu: GpuInfo,
        /// System RAM in gigabytes (rounded).
        ram_gb: u32,
        /// VRAM in megabytes when a discrete GPU is present.
        vram_mb: Option<u32>,
    },

    /// A file import pass has started.
    FileImportStarted {
        /// File name for UI lists.
        filename: String,
    },
    /// Per-file progress for Data Forge.
    FileImportProgress {
        /// File name for UI lists.
        filename: String,
        /// Normalized progress in `[0.0, 1.0]`.
        percent: f32,
    },
    /// File successfully ingested into chunks or training rows.
    FileImportCompleted {
        /// File name for UI lists.
        filename: String,
        /// Number of chunks or rows produced.
        chunks: usize,
    },
    /// File import failed; surface to toast + log.
    FileImportFailed {
        /// File name for UI lists.
        filename: String,
        /// Human-readable failure reason.
        error: String,
    },

    /// Training job accepted by the orchestrator.
    TrainingStarted {
        /// Correlates metrics with UI state.
        job_id: String,
        /// Model display name.
        model_name: String,
    },
    /// Streaming training metrics for charts and logs.
    TrainingMetrics {
        /// Correlates metrics with UI state.
        job_id: String,
        /// Current epoch (1-based UI convention).
        epoch: u32,
        /// Optimizer step counter.
        step: u32,
        /// Scalar loss from the trainer.
        loss: f64,
        /// Current learning rate.
        lr: f64,
    },
    /// Training produced an adapter bundle.
    TrainingCompleted {
        /// Correlates metrics with UI state.
        job_id: String,
        /// Adapter directory on disk.
        adapter_path: PathBuf,
    },
    /// Training stopped with an error.
    TrainingFailed {
        /// Correlates metrics with UI state.
        job_id: String,
        /// Human-readable failure reason.
        error: String,
    },

    /// Single decoded token for streaming chat UIs.
    InferenceTokenGenerated {
        /// UTF-8 token text fragment.
        token: String,
    },
    /// Final inference statistics after a completion.
    InferenceCompleted {
        /// Full assistant response text.
        full_response: String,
        /// Throughput estimate in tokens per second.
        tokens_per_second: f32,
    },

    /// RAG indexer progress.
    RagIndexingProgress {
        /// Documents processed so far.
        documents_processed: usize,
        /// Total documents scheduled.
        total: usize,
    },
    /// Retrieved chunks for optional inspector UI.
    RagSearchResults {
        /// Context snippets injected into the prompt.
        context_chunks: Vec<String>,
        /// Similarity scores aligned with `context_chunks`.
        scores: Vec<f32>,
    },

    /// Hugging Face Hub or mirror download progress.
    ModelDownloadProgress {
        /// Logical model identifier.
        model_id: String,
        /// Normalized progress in `[0.0, 1.0]`.
        percent: f32,
        /// Throughput estimate in megabytes per second.
        speed_mbps: f32,
    },
    /// Model artifact available locally.
    ModelDownloadCompleted {
        /// Logical model identifier.
        model_id: String,
        /// On-disk location of weights/config.
        path: PathBuf,
    },
    /// Export pipeline finished successfully.
    ExportCompleted {
        /// Format that was produced.
        format: ExportFormat,
        /// Primary output path (file or directory).
        path: PathBuf,
    },
}

/// Commands issued by the GUI to the backend task graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuiCommand {
    /// Import and normalize user files into the active workspace.
    ImportFiles {
        /// Absolute paths chosen via dialog or drag-and-drop.
        paths: Vec<PathBuf>,
    },
    /// Start a training job with the given hyperparameters.
    StartTraining {
        /// Training configuration validated in the UI shell.
        config: TrainingConfig,
    },
    /// Request cooperative cancellation of a running job.
    StopTraining {
        /// Target job identifier.
        job_id: String,
    },
    /// Send a user chat turn to the inference engine.
    SendChatMessage {
        /// User message text.
        message: String,
        /// When true, run retrieval before generation.
        use_rag: bool,
    },
    /// Merge, quantize, or bundle artifacts for distribution.
    ExportModel {
        /// Export parameters from the packager UI.
        config: ExportConfig,
    },
    /// Download or refresh a model from the Hub (explicit user action).
    DownloadModel {
        /// Hub model id, e.g. `meta-llama/Meta-Llama-3-8B-Instruct`.
        model_id: String,
    },
    /// Trigger a fresh hardware capability scan.
    DetectHardware,
}
