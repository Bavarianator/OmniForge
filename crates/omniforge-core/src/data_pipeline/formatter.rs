//! Conversion into Alpaca, ChatML, or raw pretraining formats.

/// Target serialization format for supervised fine-tuning.
#[derive(Debug, Clone, Copy)]
pub enum SftFormat {
    /// Alpaca triples.
    Alpaca,
    /// ChatML style messages.
    ChatMl,
    /// Plain continuation corpus.
    RawText,
}
