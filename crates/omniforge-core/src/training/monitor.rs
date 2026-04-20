//! Parsing JSON-lines metrics from the Python trainer stdout stream.

use omniforge_common::events::BackendEvent;
use serde::Deserialize;
use tokio::sync::mpsc;

/// One training metric line emitted by `python/train_lora.py`.
#[derive(Debug, Deserialize)]
pub struct TrainingMetricLine {
    /// Optimizer step.
    pub step: u32,
    /// Epoch index (trainer-defined; surfaced as-is to the GUI).
    pub epoch: u32,
    /// Scalar loss.
    pub loss: f64,
    /// Current learning rate.
    pub lr: f64,
}

/// Parses a single line; returns `None` when the line is not a metric record.
pub fn parse_metric_line(line: &str) -> Option<TrainingMetricLine> {
    serde_json::from_str::<TrainingMetricLine>(line.trim()).ok()
}

/// Forwards a parsed metric to the GUI channel.
pub async fn emit_metric(
    job_id: &str,
    metric: TrainingMetricLine,
    tx: &mpsc::Sender<BackendEvent>,
) -> Result<(), omniforge_common::error::OmniForgeError> {
    tx.send(BackendEvent::TrainingMetrics {
        job_id: job_id.to_string(),
        epoch: metric.epoch,
        step: metric.step,
        loss: metric.loss,
        lr: metric.lr,
    })
    .await
    .map_err(|_| omniforge_common::error::OmniForgeError::Training {
        message: "GUI event channel closed".to_string(),
    })
}
