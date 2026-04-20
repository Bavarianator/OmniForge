//! #region agent log
//! NDJSON debug sink for Cursor debug mode (session-scoped path).

use serde_json::{json, Value};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

const LOG_PATH: &str = "/home/bayernator/.cursor/debug-6295aa.log";
const SESSION: &str = "6295aa";

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
