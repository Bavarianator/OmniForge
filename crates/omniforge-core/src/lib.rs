//! OmniForge core: hardware detection, data pipelines, training orchestration, RAG, and export.

#![warn(missing_docs)]

#[allow(missing_docs)]
mod agent_debug;
pub mod data_pipeline;
pub mod downloader;
pub mod export;
pub mod hal;
pub mod inference;
pub mod model_registry;
pub mod rag;
pub mod runtime;
pub mod training;

pub use runtime::CoreRuntime;
