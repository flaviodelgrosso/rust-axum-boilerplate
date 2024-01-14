use axum::{
    routing::{get, post},
    Extension, Json, Router,
};

use database::user::model::User;
use mongodb::results::InsertOneResult;
use utils::AppResult;

use crate::{
    dtos::user_dto::SignUpUserDto, extractors::validation_extractor::ValidationExtractor,
    services::Services,
};

pub struct UserController;
impl UserController {
    pub fn app() -> Router {
        Router::new()
            .route("/", get(Self::all))
            .route("/signup", post(Self::signup))
    }

    pub async fn all(Extension(services): Extension<Services>) -> AppResult<Json<Vec<User>>> {
        let users = services.user.get_all_users().await?;
        Ok(Json(users))
    }

    pub async fn signup(
        Extension(services): Extension<Services>,
        ValidationExtractor(req): ValidationExtractor<SignUpUserDto>,
    ) -> AppResult<Json<InsertOneResult>> {
        let created_user = services.user.signup_user(req).await?;

        Ok(Json(created_user))
    }
}
