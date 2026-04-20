//! Coordinates long-running training jobs and bridges stdout metrics into `BackendEvent` values.

use crate::training::{hyperparams, monitor, python_bridge};
use omniforge_common::error::OmniForgeError;
use omniforge_common::events::BackendEvent;
use omniforge_common::paths;
use omniforge_common::types::TrainingConfig;
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{error, info};

/// Owns at most one training task; starting a new job aborts any previous handle after awaiting cleanup.
pub struct TrainingOrchestrator {
    handle: Option<JoinHandle<()>>,
}

impl Default for TrainingOrchestrator {
    fn default() -> Self {
        Self { handle: None }
    }
}

impl TrainingOrchestrator {
    /// Starts a training job, preempting any previous job handle.
    pub async fn start(
        &mut self,
        mut config: TrainingConfig,
        events: mpsc::Sender<BackendEvent>,
    ) -> Result<(), OmniForgeError> {
        if let Some(previous) = self.handle.take() {
            previous.abort();
            let _ignored = previous.await;
        }

        hyperparams::normalize(&mut config);

        let job_id = config.job_id.clone();
        let model_name = config.model_name.clone();

        events
            .send(BackendEvent::TrainingStarted {
                job_id: job_id.clone(),
                model_name,
            })
            .await
            .map_err(|_| OmniForgeError::Training {
                message: "GUI event channel closed".to_string(),
            })?;

        let events_task = events.clone();
        let task = tokio::spawn(async move {
            if let Err(err) = run_training_job(config, events_task).await {
                error!(?err, "training job failed");
            }
        });

        self.handle = Some(task);
        Ok(())
    }

    /// Aborts the running job, if any (idempotent).
    pub async fn stop(&mut self, _job_id: &str) -> Result<(), OmniForgeError> {
        if let Some(handle) = self.handle.take() {
            handle.abort();
            let _ignored = handle.await;
        }
        Ok(())
    }
}

async fn run_training_job(
    config: TrainingConfig,
    events: mpsc::Sender<BackendEvent>,
) -> Result<(), OmniForgeError> {
    let script = python_bridge::ensure_train_script_present()?;
    let python = python_bridge::resolve_python_executable();

    let adapter_dir: PathBuf = paths::data_dir()?.join("adapters").join(&config.job_id);
    std::fs::create_dir_all(&adapter_dir).map_err(OmniForgeError::Io)?;

    let mut cmd: Command = python_bridge::build_train_command(&python, &script, &config.workspace);
    cmd.arg("--job-id")
        .arg(&config.job_id)
        .arg("--dataset")
        .arg(&config.dataset_path)
        .arg("--output")
        .arg(&adapter_dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    info!(?python, ?script, "spawning Python trainer");
    let mut child = cmd.spawn().map_err(|e| OmniForgeError::Training {
        message: format!("failed to spawn trainer: {e}"),
    })?;

    let stdout = child.stdout.take().ok_or_else(|| OmniForgeError::Training {
        message: "missing stdout pipe".to_string(),
    })?;

    let mut reader = BufReader::new(stdout).lines();
    while let Ok(Some(line)) = reader.next_line().await {
        if let Some(metric) = monitor::parse_metric_line(&line) {
            monitor::emit_metric(&config.job_id, metric, &events).await?;
        }
    }

    let status = child.wait().await.map_err(|e| OmniForgeError::Training {
        message: format!("failed to wait for trainer: {e}"),
    })?;

    if status.success() {
        events
            .send(BackendEvent::TrainingCompleted {
                job_id: config.job_id.clone(),
                adapter_path: adapter_dir,
            })
            .await
            .map_err(|_| OmniForgeError::Training {
                message: "GUI event channel closed".to_string(),
            })?;
    } else {
        events
            .send(BackendEvent::TrainingFailed {
                job_id: config.job_id.clone(),
                error: format!("trainer exited with {status}"),
            })
            .await
            .map_err(|_| OmniForgeError::Training {
                message: "GUI event channel closed".to_string(),
            })?;
    }

    Ok(())
}
