use axum::body::Bytes;
use thiserror::Error;
use tonic::{metadata::MetadataMap, Code, Status};

impl From<AppError> for Status {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Validation(validation_errors) => Self::with_details_and_metadata(
                Code::InvalidArgument,
                validation_errors.to_string(),
                Bytes::new(),
                MetadataMap::new(),
            ),
            AppError::Json(error) => Self::internal(error.to_string()),
            AppError::Other(error) => Self::internal(error.to_string()),
            AppError::NotFound => Self::not_found("entity not found"),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("entity not found")]
    NotFound,
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
