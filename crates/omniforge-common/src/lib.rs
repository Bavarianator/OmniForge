//! Shared building blocks for the OmniForge workspace: errors, domain types, IPC events, and XDG paths.

#![warn(missing_docs)]

pub mod error;
pub mod events;
pub mod paths;
pub mod types;

pub use error::OmniForgeError;
pub use events::{BackendEvent, GuiCommand};
