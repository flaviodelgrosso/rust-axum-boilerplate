use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use utils::CargoEnv;

pub struct Logger;
impl Logger {
    pub fn init(cargo_env: CargoEnv) -> WorkerGuard {
        let (non_blocking, guard) = match cargo_env {
            CargoEnv::Development => {
                let console_logger = std::io::stdout();
                tracing_appender::non_blocking(console_logger)
            }
            CargoEnv::Production => {
                let file_logger = tracing_appender::rolling::daily("logs", "log");
                tracing_appender::non_blocking(file_logger)
            }
        };

        // Set the default verbosity level for the root of the dependency graph.
        // env var: `RUST_LOG`
        let env_filter =
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_PKG_NAME")).into()
            });

        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
            .init();

        guard
    }
}
