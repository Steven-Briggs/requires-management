use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not authenticated")]
    Unauthenticated,

    #[error("Forbidden: insufficient permissions")]
    Forbidden,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("External API error: {0}")]
    ExternalApi(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        use crate::response::ApiResponse;

        let (status, message) = match self {
            AppError::Unauthenticated => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                self.to_string(),
            ),
            AppError::Forbidden => (
                actix_web::http::StatusCode::FORBIDDEN,
                self.to_string(),
            ),
            AppError::NotFound(msg) => (
                actix_web::http::StatusCode::NOT_FOUND,
                msg.clone(),
            ),
            AppError::BadRequest(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                msg.clone(),
            ),
            AppError::Database(e) => {
                tracing::error!("Database error: {e}");
                (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred".to_string(),
                )
            }
            AppError::ExternalApi(msg) => (
                actix_web::http::StatusCode::BAD_GATEWAY,
                msg.clone(),
            ),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {msg}");
                (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal error occurred".to_string(),
                )
            }
        };

        HttpResponse::build(status).json(ApiResponse::<()>::error(message))
    }
}
