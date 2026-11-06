use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::error::ErrorKind;

pub type Result<T> = std::result::Result<T, AppError>;

pub enum AppError {
    NotFound,
    Unauthorized,
    Forbidden,
    BadRequest(String),
    Conflict(String),
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "forbidden".to_string()),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Conflict(m) => (StatusCode::CONFLICT, m),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),
        };
        (status, Json(ErrorBody { error: message })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            sqlx::Error::Database(db) => match db.kind() {
                ErrorKind::UniqueViolation => AppError::Conflict("already exists".to_string()),
                ErrorKind::ForeignKeyViolation => {
                    AppError::Conflict("related record is missing or still in use".to_string())
                }
                ErrorKind::CheckViolation | ErrorKind::NotNullViolation => {
                    AppError::BadRequest("invalid values".to_string())
                }
                _ => {
                    eprintln!("database error: {db}");
                    AppError::Internal
                }
            },
            other => {
                eprintln!("sqlx error: {other}");
                AppError::Internal
            }
        }
    }
}
