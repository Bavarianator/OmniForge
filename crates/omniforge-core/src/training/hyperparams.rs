//! Defaults and clamping for LoRA hyperparameters.

use omniforge_common::types::{QuantizationMode, TrainingConfig};

/// Applies conservative clamps and fills missing auto-fields before spawning the trainer.
pub fn normalize(config: &mut TrainingConfig) {
    config.lora_rank = config.lora_rank.clamp(4, 128);
    config.lora_alpha = config.lora_alpha.max(1);
    config.epochs = config.epochs.clamp(1, 100);
    if config.batch_size == 0 {
        config.batch_size = 1;
    }
}

/// Maps HAL recommendation strings to a concrete quantization mode.
pub fn quantization_from_hint(hint: &str) -> QuantizationMode {
    match hint {
        "fp16" => QuantizationMode::None,
        "8bit_or_fp16_lora" | "5bit_or_8bit_lora" => QuantizationMode::EightBit,
        _ => QuantizationMode::FourBit,
    }
}
