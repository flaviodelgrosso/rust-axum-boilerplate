use std::sync::Arc;

use async_trait::async_trait;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};
use tokio_stream::StreamExt;

use crate::{user::model::User, Database};
use utils::AppResult;

#[allow(clippy::module_name_repetitions)]
pub type DynUserRepository = Arc<dyn UserRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn create_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> AppResult<InsertOneResult>;

    async fn get_user_by_id(&self, id: &str) -> AppResult<Option<User>>;

    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>>;

    async fn update_user(
        &self,
        id: &str,
        name: &str,
        email: &str,
        password: &str,
    ) -> AppResult<UpdateResult>;

    async fn delete_user(&self, id: &str) -> AppResult<DeleteResult>;

    async fn get_all_users(&self) -> AppResult<Vec<User>>;
}

#[async_trait]
impl UserRepositoryTrait for Database {
    async fn create_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> AppResult<InsertOneResult> {
        let new_doc = User {
            id: None,
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };

        let user = self.user_col.insert_one(new_doc).await?;

        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let filter = doc! {"email": email};
        let user_detail = self.user_col.find_one(filter).await?;

        Ok(user_detail)
    }

    async fn get_user_by_id(&self, id: &str) -> AppResult<Option<User>> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let user_detail = self.user_col.find_one(filter).await?;

        Ok(user_detail)
    }

    async fn update_user(
        &self,
        id: &str,
        name: &str,
        email: &str,
        password: &str,
    ) -> AppResult<UpdateResult> {
        let id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": id};
        let new_doc = doc! {
            "$set":
                {
                    "name": name,
                    "email": email,
                    "password": password,
                },
        };

        let updated_doc = self.user_col.update_one(filter, new_doc).await?;

        Ok(updated_doc)
    }

    async fn delete_user(&self, id: &str) -> AppResult<DeleteResult> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let user_detail = self.user_col.delete_one(filter).await?;

        Ok(user_detail)
    }

    async fn get_all_users(&self) -> AppResult<Vec<User>> {
        let filter = doc! {};
        let mut cursor = self.user_col.find(filter).await?;

        let mut users: Vec<User> = Vec::new();
        while let Some(doc) = cursor.next().await {
            users.push(doc?);
        }

        Ok(users)
    }
}
