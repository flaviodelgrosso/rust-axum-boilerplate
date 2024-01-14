#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum CargoEnv {
    Development,
    Production,
}

#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env, value_enum)]
    pub cargo_env: CargoEnv,

    #[clap(long, env, default_value = "127.0.0.1")]
    pub app_host: String,

    #[clap(long, env, default_value = "5000")]
    pub app_port: u16,

    #[clap(long, env, default_value = "mongodb://localhost:27017")]
    pub mongo_uri: String,

    #[clap(long, env)]
    pub mongo_db: String,
}
