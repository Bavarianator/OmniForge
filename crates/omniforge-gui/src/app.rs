//! Root Relm4 component hosting the primary navigation layout (dashboard placeholder).

use adw::prelude::*;
use async_channel::Receiver;
use gtk::glib::clone;
use gtk::glib::MainContext;
use omniforge_common::events::{BackendEvent, GuiCommand};
use omniforge_core::CoreRuntime;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};
use serde_json::json;
use tracing::warn;

use crate::agent_debug;
use crate::components::dashboard::DashboardWidgets;

/// Initialization payload assembled in `main` before GTK spins up.
pub struct AppInit {
    /// Keeps the background core thread and its `cmd_tx` endpoint alive.
    pub runtime: CoreRuntime,
    /// Async channel fed by the Tokio bridge; consumed on the GTK main loop via [`MainContext::spawn_local`].
    pub backend_events: Receiver<BackendEvent>,
}

/// Top-level application state (expanded as views are wired to `GuiCommand`).
pub struct AppModel {
    /// Owns the background runtime so the command channel stays open for the app lifetime.
    runtime: CoreRuntime,
    /// Status / telemetry line for operators.
    pub status: String,
}

/// User actions and backend notifications routed through the root component.
#[derive(Debug)]
pub enum AppMsg {
    /// Ask the core HAL to rescan GPUs and memory.
    DetectHardware,
    /// Emitted on the GTK thread when the Tokio core publishes a [`BackendEvent`].
    Backend(BackendEvent),
}

/// Widgets that must be updated when `AppModel` changes.
pub struct AppWidgets {
    dashboard: DashboardWidgets,
    status_label: gtk::Label,
}

impl SimpleComponent for AppModel {
    type Init = AppInit;
    type Input = AppMsg;
    type Output = ();
    type Root = adw::ApplicationWindow;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        adw::ApplicationWindow::builder()
            .default_width(1280)
            .default_height(800)
            .build()
    }

    fn init(
        init: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let ui = sender.clone();
        let backend_rx = init.backend_events;
        MainContext::default().spawn_local(async move {
            // #region agent log
            agent_debug::log(
                "H4",
                "app.rs:spawn_local",
                "consumer_started",
                json!({}),
            );
            // #endregion
            while let Ok(event) = backend_rx.recv().await {
                // #region agent log
                agent_debug::log(
                    "H4",
                    "app.rs:spawn_local",
                    "async_channel_recv",
                    json!({"event": agent_debug::backend_event_tag(&event)}),
                );
                // #endregion
                ui.input(AppMsg::Backend(event));
            }
        });

        let model = AppModel {
            runtime: init.runtime,
            status: "Bereit — verbunden mit lokalem OmniForge-Core.".to_string(),
        };

        window.set_title(Some("OmniForge"));

        let root = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(0)
            .build();

        let header = adw::HeaderBar::new();
        let title = gtk::Label::new(Some("OmniForge"));
        header.set_title_widget(Some(&title));

        let detect_btn = gtk::Button::with_label("Hardware erkennen");
        detect_btn.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::DetectHardware);
            }
        ));
        header.pack_end(&detect_btn);

        root.append(&header);

        let content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .margin_start(18)
            .margin_end(18)
            .margin_top(18)
            .margin_bottom(18)
            .build();

        let dashboard = DashboardWidgets::new();
        content.append(dashboard.root_widget());

        let status_label = gtk::Label::builder()
            .label(&model.status)
            .xalign(0.0)
            .wrap(true)
            .build();
        content.append(&status_label);

        root.append(&content);
        window.set_content(Some(&root));

        let widgets = AppWidgets {
            dashboard,
            status_label,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::DetectHardware => {
                // #region agent log
                agent_debug::log("H1", "app.rs:DetectHardware", "enter", json!({}));
                // #endregion
                self.status = "Hardware wird ermittelt …".to_string();
                match self
                    .runtime
                    .command_sender()
                    .blocking_send(GuiCommand::DetectHardware)
                {
                    Ok(()) => {
                        // #region agent log
                        agent_debug::log(
                            "H1",
                            "app.rs:DetectHardware",
                            "blocking_send_ok",
                            json!({}),
                        );
                        // #endregion
                    }
                    Err(err) => {
                        // #region agent log
                        agent_debug::log(
                            "H1",
                            "app.rs:DetectHardware",
                            "blocking_send_err",
                            json!({"error": err.to_string()}),
                        );
                        // #endregion
                        self.status = format!("Backend nicht erreichbar: {err}");
                        warn!(?err, "failed to send DetectHardware");
                    }
                }
            }
            AppMsg::Backend(event) => match event {
                BackendEvent::HardwareDetected {
                    gpu,
                    ram_gb,
                    vram_mb,
                } => {
                    // #region agent log
                    agent_debug::log(
                        "H4",
                        "app.rs:Backend",
                        "HardwareDetected_ui",
                        json!({"vendor": &gpu.vendor, "ram_gb": ram_gb}),
                    );
                    // #endregion
                    let model = gpu.model.as_deref().unwrap_or("Unbekanntes Modell");
                    let vram = vram_mb
                        .map(|m| format!("{m} MiB VRAM"))
                        .unwrap_or_else(|| "VRAM unbekannt".to_string());
                    self.status = format!(
                        "{} — {} — System-RAM ca. {ram_gb} GB — {vram}",
                        gpu.vendor, model
                    );
                }
                BackendEvent::TrainingMetrics {
                    job_id,
                    epoch,
                    step,
                    loss,
                    lr,
                } => {
                    self.status = format!(
                        "Training {job_id}: Epoche {epoch}, Schritt {step}, Loss {loss:.4}, LR {lr:.2e}"
                    );
                }
                BackendEvent::TrainingCompleted { job_id, adapter_path } => {
                    self.status = format!(
                        "Training {job_id} abgeschlossen — Adapter: {}",
                        adapter_path.display()
                    );
                }
                BackendEvent::TrainingFailed { job_id, error } => {
                    self.status = format!("Training {job_id} fehlgeschlagen: {error}");
                }
                BackendEvent::FileImportStarted { filename } => {
                    self.status = format!("Import startet: {filename}");
                }
                BackendEvent::FileImportProgress { filename, percent } => {
                    self.status = format!("Import {filename}: {:.0} %", percent * 100.0);
                }
                BackendEvent::FileImportCompleted { filename, chunks } => {
                    self.status = format!("Import fertig: {filename} ({chunks} Chunks)");
                }
                BackendEvent::FileImportFailed { filename, error } => {
                    self.status = format!("Import fehlgeschlagen ({filename}): {error}");
                }
                other => {
                    self.status = format!("{other:?}");
                }
            },
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        widgets.status_label.set_label(&self.status);
    }
}
