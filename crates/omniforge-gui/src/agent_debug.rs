//! #region agent log
//! NDJSON debug sink for Cursor debug mode (session-scoped path).

use omniforge_common::events::BackendEvent;
use serde_json::{json, Value};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

const LOG_PATH: &str = "/home/bayernator/.cursor/debug-6295aa.log";
const SESSION: &str = "6295aa";

/// Stable event name for logs (no payload).
pub(crate) fn backend_event_tag(ev: &BackendEvent) -> &'static str {
    match ev {
        BackendEvent::HardwareDetected { .. } => "HardwareDetected",
        BackendEvent::FileImportStarted { .. } => "FileImportStarted",
        BackendEvent::FileImportProgress { .. } => "FileImportProgress",
        BackendEvent::FileImportCompleted { .. } => "FileImportCompleted",
        BackendEvent::FileImportFailed { .. } => "FileImportFailed",
        BackendEvent::TrainingStarted { .. } => "TrainingStarted",
        BackendEvent::TrainingMetrics { .. } => "TrainingMetrics",
        BackendEvent::TrainingCompleted { .. } => "TrainingCompleted",
        BackendEvent::TrainingFailed { .. } => "TrainingFailed",
        BackendEvent::InferenceTokenGenerated { .. } => "InferenceTokenGenerated",
        BackendEvent::InferenceCompleted { .. } => "InferenceCompleted",
        BackendEvent::RagIndexingProgress { .. } => "RagIndexingProgress",
        BackendEvent::RagSearchResults { .. } => "RagSearchResults",
        BackendEvent::ModelDownloadProgress { .. } => "ModelDownloadProgress",
        BackendEvent::ModelDownloadCompleted { .. } => "ModelDownloadCompleted",
        BackendEvent::ExportCompleted { .. } => "ExportCompleted",
    }
}

/// Appends one NDJSON line; never panics.
pub(crate) fn log(hypothesis_id: &str, location: &str, message: &str, data: Value) {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let line = json!({
        "sessionId": SESSION,
        "hypothesisId": hypothesis_id,
        "location": location,
        "message": message,
        "data": data,
        "timestamp": ts,
    });
    if let Ok(s) = serde_json::to_string(&line) {
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(LOG_PATH)
        {
            let _ = writeln!(f, "{s}");
        }
    }
}
// #endregion
