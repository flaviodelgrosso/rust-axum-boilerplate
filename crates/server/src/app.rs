use anyhow::Context;
use axum::serve;
use database::Database;
use std::sync::Arc;
use tokio::signal;
use tracing::info;
use utils::AppConfig;

use crate::services::Services;
use crate::{logger::Logger, router::AppRouter};

pub struct ApplicationServer;
impl ApplicationServer {
    pub async fn serve(config: Arc<AppConfig>) -> anyhow::Result<()> {
        let _guard = Logger::init(config.cargo_env);

        let address = format!("{}:{}", config.app_host, config.app_port);
        let tcp_listener = tokio::net::TcpListener::bind(address)
            .await
            .context("Failed to bind TCP listener")?;

        let local_addr = tcp_listener
            .local_addr()
            .context("Failed to get local address")?;

        info!("server has launched on {local_addr} 🚀");

        let db = Database::new(config.clone()).await?;
        let services = Services::new(db);
        let router = AppRouter::init(services);

        serve(tcp_listener, router)
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .context("Failed to start server")?;

        Ok(())
    }

    async fn shutdown_signal() {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("Failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            () = ctrl_c => {},
            () = terminate => {},
        }

        tracing::warn!("❌ Signal received, starting graceful shutdown...");
    }
}
