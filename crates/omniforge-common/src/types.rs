//! Cross-cutting domain types shared between GUI and core.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Describes a detected or configured GPU/accelerator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    /// Vendor string, e.g. "NVIDIA", "AMD".
    pub vendor: String,
    /// Short model label when known.
    pub model: Option<String>,
    /// Dedicated video memory in megabytes, if applicable.
    pub vram_mb: Option<u32>,
    /// Driver or stack version string when discoverable.
    pub driver_version: Option<String>,
}

/// User-selected export target for a packaged model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    /// Portable GGUF weights for llama.cpp-compatible runtimes.
    Gguf,
    /// Ollama `Modelfile` plus local path hints.
    Ollama,
    /// Local OpenAI-compatible HTTP API (localhost).
    LocalApi,
}

/// Parameters for a LoRA / QLoRA training job initiated from the GUI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Stable identifier used for logs and cancellation.
    pub job_id: String,
    /// Display name of the base model artifact.
    pub model_name: String,
    /// Absolute path to the workspace project root.
    pub workspace: PathBuf,
    /// Path to dataset manifest or `jsonl` produced by Data Forge.
    pub dataset_path: PathBuf,
    /// Learning rate (e.g. 2e-4).
    pub learning_rate: f64,
    /// LoRA rank.
    pub lora_rank: u32,
    /// LoRA alpha.
    pub lora_alpha: u32,
    /// Number of training epochs.
    pub epochs: u32,
    /// Micro batch size; may be auto-tuned by HAL before start.
    pub batch_size: u32,
    /// Quantization mode for the base weights during tuning.
    pub quantization: QuantizationMode,
}

/// Quantization strategy for QLoRA-style training.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuantizationMode {
    /// 4-bit NF4 (typical QLoRA).
    FourBit,
    /// 8-bit linear quantization.
    EightBit,
    /// No quantization (requires sufficient VRAM).
    None,
}

/// Options for merging adapters and publishing artifacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Target export format.
    pub format: ExportFormat,
    /// Path to the base model directory or file.
    pub base_model: PathBuf,
    /// Path to the trained adapter directory.
    pub adapter_path: PathBuf,
    /// Output directory for generated artifacts.
    pub output_dir: PathBuf,
    /// Optional GGUF quantization preset label (e.g. "Q4_K_M").
    pub gguf_preset: Option<String>,
}
