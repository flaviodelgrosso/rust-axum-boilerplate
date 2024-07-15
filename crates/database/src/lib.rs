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
    /// Creates a new `Database` instance.
    ///
    /// # Arguments
    ///
    /// * `config` - An `Arc` containing the application configuration.
    ///
    /// # Returns
    ///
    /// * `AppResult<Self>` - A result containing the `Database` instance or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `MongoDB` client cannot be initialized
    /// or if the specified database or collection cannot be accessed.
    pub async fn new(config: Arc<AppConfig>) -> AppResult<Self> {
        let client = Client::with_uri_str(&config.mongo_uri).await?;
        let db = client.database(&config.mongo_db);
        let user_col: Collection<User> = db.collection("User");

        info!("initializing database connection...");

        Ok(Database { user_col })
    }
}
