#![allow(dead_code)]
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;

use axum::extract::rejection::JsonRejection;
use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tracing::debug;
use tracing::log::error;
use validator::{ValidationErrors, ValidationErrorsKind};

pub type AppResult<T> = Result<T, AppError>;

pub type ErrorMap = HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpError {
    pub error: String,
}

impl HttpError {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("authentication is required to access this resource")]
    Unauthorized,
    #[error("user does not have privilege to access this resource")]
    Forbidden,
    #[error("unexpected error has occurred")]
    InternalServerError,
    #[error("{0}")]
    InternalServerErrorWithContext(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    PreconditionFailed(String),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),
    #[error("unprocessable request has occurred")]
    UnprocessableEntity { errors: ErrorMap },
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error("{0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("{0}")]
    MongoErrorKind(mongodb::error::ErrorKind),
    #[error("error serializing BSON")]
    MongoSerializeBsonError(#[from] mongodb::bson::ser::Error),
    #[error("error deserializing BSON")]
    MongoDeserializeBsonError(#[from] mongodb::bson::de::Error),
    #[error("document validation error")]
    MongoDataError(#[from] mongodb::bson::document::ValueAccessError),
    #[error("error converting object id")]
    MongoObjectIdError(#[from] mongodb::bson::oid::Error),
}

impl AppError {
    /// Maps `validator`'s `ValidationrErrors` to a simple map of property name/error messages structure.
    pub fn unprocessable_entity(errors: ValidationErrors) -> Response {
        let mut validation_errors = ErrorMap::new();

        for (field_property, error_kind) in errors.into_errors() {
            if let ValidationErrorsKind::Field(field_meta) = error_kind.clone() {
                for error in field_meta.into_iter() {
                    validation_errors
                        .entry(Cow::from(field_property))
                        .or_insert_with(Vec::new)
                        .push(error.message.unwrap_or_else(|| {
                            let params: Vec<Cow<'static, str>> = error
                                .params
                                .iter()
                                .filter(|(key, _value)| key.to_owned() != "value")
                                .map(|(key, value)| {
                                    Cow::from(format!("{} value is {}", key, value.to_string()))
                                })
                                .collect();

                            if params.len() >= 1 {
                                Cow::from(params.join(", "))
                            } else {
                                Cow::from(format!("{} is required", field_property))
                            }
                        }))
                }
            }

            if let ValidationErrorsKind::Struct(meta) = error_kind.clone() {
                for (struct_property, struct_error_kind) in meta.into_errors() {
                    if let ValidationErrorsKind::Field(field_meta) = struct_error_kind {
                        for error in field_meta.into_iter() {
                            validation_errors
                                .entry(Cow::from(struct_property))
                                .or_insert_with(Vec::new)
                                .push(error.message.unwrap_or_else(|| {
                                    Cow::from(format!("{} is required", struct_property))
                                }));
                        }
                    }
                }
            }
        }

        let body = Json(json!({
            "errors": validation_errors,
        }));

        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        debug!("{:#?}", self);
        if let Self::ValidationError(e) = self {
            return Self::unprocessable_entity(e);
        }

        let (status, error_message) = match self {
            Self::InternalServerErrorWithContext(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            Self::NotFound(err) => (StatusCode::NOT_FOUND, err),
            Self::Conflict(err) => (StatusCode::CONFLICT, err),
            Self::PreconditionFailed(err) => (StatusCode::PRECONDITION_FAILED, err),
            Self::BadRequest(err) => (StatusCode::BAD_REQUEST, err),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, Self::Unauthorized.to_string()),
            Self::Forbidden => (StatusCode::FORBIDDEN, Self::Forbidden.to_string()),
            Self::AxumJsonRejection(err) => (StatusCode::BAD_REQUEST, err.body_text()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Self::InternalServerError.to_string(),
            ),
        };

        let body = Json(HttpError::new(error_message));

        (status, body).into_response()
    }
}
