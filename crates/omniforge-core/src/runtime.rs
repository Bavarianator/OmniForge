//! Tokio-backed orchestration on a dedicated thread: `GuiCommand` in, `BackendEvent` out.

use crate::hal;
use crate::training::orchestrator::TrainingOrchestrator;
use omniforge_common::error::OmniForgeError;
use omniforge_common::events::{BackendEvent, GuiCommand};
use serde_json::json;
use std::sync::Arc;
use std::thread;
use tokio::runtime::Builder;
use tokio::sync::{mpsc, Mutex};
use tracing::{error, info};

/// Handle for dispatching commands to the background core thread.
pub struct CoreRuntime {
    cmd_tx: mpsc::Sender<GuiCommand>,
}

impl CoreRuntime {
    /// Spawns a dedicated OS thread running a multi-thread Tokio runtime that executes [`handle_command`].
    ///
    /// The returned receiver must be drained (or bridged to the GUI main loop); otherwise completed work
    /// will block once the channel buffer fills.
    pub fn new() -> (Self, mpsc::Receiver<BackendEvent>) {
        let (event_tx, event_rx) = mpsc::channel::<BackendEvent>(256);
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<GuiCommand>(32);
        let training = Arc::new(Mutex::new(TrainingOrchestrator::default()));

        thread::Builder::new()
            .name("omniforge-core".into())
            .spawn(move || {
                let runtime = match Builder::new_multi_thread().enable_all().build() {
                    Ok(rt) => rt,
                    Err(err) => {
                        error!(?err, "failed to build Tokio runtime for OmniForge core");
                        return;
                    }
                };

                runtime.block_on(async move {
                    while let Some(cmd) = cmd_rx.recv().await {
                        // #region agent log
                        crate::agent_debug::log(
                            "H2",
                            "runtime.rs:cmd_rx",
                            "received",
                            json!({"cmd": gui_command_tag(&cmd)}),
                        );
                        // #endregion
                        if let Err(err) = handle_command(&cmd, &event_tx, &training).await {
                            // #region agent log
                            crate::agent_debug::log(
                                "H2",
                                "runtime.rs:handle_command",
                                "error",
                                json!({"error": err.to_string()}),
                            );
                            // #endregion
                            error!(?err, "command failed");
                        }
                    }
                });
            })
            .expect("spawn omniforge-core thread");

        (Self { cmd_tx }, event_rx)
    }

    /// Cloneable handle used from GTK signal handlers (typically via [`tokio::sync::mpsc::Sender::blocking_send`]).
    pub fn command_sender(&self) -> mpsc::Sender<GuiCommand> {
        self.cmd_tx.clone()
    }
}

async fn handle_command(
    cmd: &GuiCommand,
    events: &mpsc::Sender<BackendEvent>,
    training: &Arc<Mutex<TrainingOrchestrator>>,
) -> Result<(), OmniForgeError> {
    match cmd {
        GuiCommand::DetectHardware => {
            // #region agent log
            crate::agent_debug::log("H2", "runtime.rs:DetectHardware", "enter", json!({}));
            // #endregion
            let profile = match hal::detect_hardware().await {
                Ok(p) => {
                    // #region agent log
                    crate::agent_debug::log(
                        "H5",
                        "runtime.rs:DetectHardware",
                        "hal_ok",
                        json!({"vendor": &p.gpu.vendor, "ram_gb": p.ram_gb}),
                    );
                    // #endregion
                    p
                }
                Err(e) => {
                    // #region agent log
                    crate::agent_debug::log(
                        "H5",
                        "runtime.rs:DetectHardware",
                        "hal_err",
                        json!({"error": e.to_string()}),
                    );
                    // #endregion
                    return Err(e);
                }
            };
            events
                .send(BackendEvent::HardwareDetected {
                    gpu: profile.gpu.clone(),
                    ram_gb: profile.ram_gb,
                    vram_mb: profile.gpu.vram_mb,
                })
                .await
                .map_err(|_| {
                    // #region agent log
                    crate::agent_debug::log(
                        "H3",
                        "runtime.rs:DetectHardware",
                        "event_send_failed",
                        json!({}),
                    );
                    // #endregion
                    OmniForgeError::Hardware {
                        message: "GUI event channel closed".to_string(),
                    }
                })?;
            // #region agent log
            crate::agent_debug::log(
                "H3",
                "runtime.rs:DetectHardware",
                "event_sent_ok",
                json!({}),
            );
            // #endregion
        }
        GuiCommand::StartTraining { config } => {
            let mut guard = training.lock().await;
            guard.start(config.clone(), events.clone()).await?;
        }
        GuiCommand::StopTraining { job_id } => {
            let mut guard = training.lock().await;
            guard.stop(job_id.as_str()).await?;
        }
        GuiCommand::ImportFiles { paths: file_paths } => {
            for path in file_paths {
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("<unknown>")
                    .to_string();
                events
                    .send(BackendEvent::FileImportStarted {
                        filename: filename.clone(),
                    })
                    .await
                    .map_err(|_| OmniForgeError::Rag {
                        message: "GUI event channel closed".to_string(),
                    })?;
                match crate::data_pipeline::ingest::import_path(path).await {
                    Ok(chunks) => {
                        events
                            .send(BackendEvent::FileImportCompleted {
                                filename,
                                chunks,
                            })
                            .await
                            .map_err(|_| OmniForgeError::Rag {
                                message: "GUI event channel closed".to_string(),
                            })?;
                    }
                    Err(err) => {
                        events
                            .send(BackendEvent::FileImportFailed {
                                filename,
                                error: err.to_string(),
                            })
                            .await
                            .map_err(|_| OmniForgeError::Rag {
                                message: "GUI event channel closed".to_string(),
                            })?;
                    }
                }
            }
        }
        other => {
            info!(?other, "command not implemented in scaffold runtime");
        }
    }
    Ok(())
}

fn gui_command_tag(cmd: &GuiCommand) -> &'static str {
    match cmd {
        GuiCommand::DetectHardware => "DetectHardware",
        GuiCommand::StartTraining { .. } => "StartTraining",
        GuiCommand::StopTraining { .. } => "StopTraining",
        GuiCommand::ImportFiles { .. } => "ImportFiles",
        GuiCommand::SendChatMessage { .. } => "SendChatMessage",
        GuiCommand::ExportModel { .. } => "ExportModel",
        GuiCommand::DownloadModel { .. } => "DownloadModel",
    }
}
