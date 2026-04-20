//! Runtime sampling parameters for chat and batch inference.

/// User-tunable decoding configuration mapped to llama.cpp flags.
#[derive(Debug, Clone)]
pub struct InferenceParams {
    /// Softmax temperature.
    pub temperature: f32,
    /// Top-p nucleus sampling threshold.
    pub top_p: f32,
    /// Maximum new tokens per completion.
    pub max_tokens: u32,
}

impl Default for InferenceParams {
    fn default() -> Self {
        Self {
            temperature: 0.8,
            top_p: 0.95,
            max_tokens: 512,
        }
    }
}
