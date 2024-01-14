pub mod user;

use std::sync::Arc;

use mongodb::{Client, Collection};
use tracing::info;

use user::model::User;
use utils::{AppConfig, AppResult};

#[derive(Clone, Debug)]
pub struct Database {
    pub user_col: Collection<User>,
}

impl Database {
    pub async fn new(config: Arc<AppConfig>) -> AppResult<Self> {
        let client = Client::with_uri_str(&config.mongo_uri).await?;
        let db = client.database(&config.mongo_db);
        let user_col: Collection<User> = db.collection("User");

        info!("initializing database connection...");

        Ok(Database { user_col })
    }
}
