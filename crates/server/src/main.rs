pub(crate) mod api;
pub(crate) mod app;
pub(crate) mod dtos;
pub(crate) mod extractors;
pub(crate) mod logger;
pub(crate) mod router;
pub(crate) mod services;

use std::sync::Arc;

use anyhow::{Context, Result};
use app::ApplicationServer;
use clap::Parser;
use dotenvy::dotenv;
use utils::AppConfig;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let config = Arc::new(AppConfig::parse());

    ApplicationServer::serve(config)
        .await
        .context("Failed to start server")?;

    Ok(())
}
