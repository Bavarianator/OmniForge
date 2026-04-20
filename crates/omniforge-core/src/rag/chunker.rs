//! Token-budget aware chunking strategies.

/// Default chunk size target (tokens) for RAG corpora.
pub const DEFAULT_CHUNK_TOKENS: usize = 512;

/// Default overlap between consecutive chunks (tokens).
pub const DEFAULT_CHUNK_OVERLAP: usize = 50;
