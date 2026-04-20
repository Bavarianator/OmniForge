//! OmniForge desktop entry: Adwaita shell, Relm4 root component, structured logging.

use adw::prelude::*;
use omniforge_common::events::BackendEvent;
use omniforge_core::CoreRuntime;
use relm4::RelmApp;
use serde_json::json;
use tokio::runtime::Builder as TokioBuilder;

mod agent_debug;
mod app;
mod components;
mod config;
mod widgets;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("omniforge=info")),
        )
        .init();

    tracing::info!("OmniForge startet");

    let (async_tx, async_rx) = async_channel::unbounded::<BackendEvent>();

    let (core, mut event_rx) = CoreRuntime::new();

    std::thread::Builder::new()
        .name("omniforge-event-bridge".into())
        .spawn(move || {
            let rt = TokioBuilder::new_current_thread()
                .enable_all()
                .build()
                .expect("Tokio bridge runtime");
            rt.block_on(async move {
                // #region agent log
                agent_debug::log(
                    "H3",
                    "main.rs:bridge",
                    "bridge_runtime_ready",
                    json!({}),
                );
                // #endregion
                while let Some(ev) = event_rx.recv().await {
                    // #region agent log
                    agent_debug::log(
                        "H3",
                        "main.rs:bridge",
                        "tokio_event_rx",
                        json!({"event": agent_debug::backend_event_tag(&ev)}),
                    );
                    // #endregion
                    if async_tx.send(ev).await.is_err() {
                        // #region agent log
                        agent_debug::log(
                            "H3",
                            "main.rs:bridge",
                            "async_channel_send_failed",
                            json!({}),
                        );
                        // #endregion
                        tracing::debug!("GUI event consumer ended; stopping Tokio bridge");
                        break;
                    }
                    // #region agent log
                    agent_debug::log(
                        "H3",
                        "main.rs:bridge",
                        "async_channel_forward_ok",
                        json!({}),
                    );
                    // #endregion
                }
            });
        })
        .expect("spawn event bridge thread");

    let gtk_app = adw::Application::builder()
        .application_id("com.omniforge.App")
        .build();

    let relm = RelmApp::from_app(gtk_app);
    relm.set_global_css(include_str!("../../../assets/style.css"));
    relm.run::<app::AppModel>(app::AppInit {
        runtime: core,
        backend_events: async_rx,
    });
}
