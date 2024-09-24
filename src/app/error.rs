use anyhow::Error;
use tonic::Status;

pub struct AppError(Error);

impl<E> From<E> for AppError
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl From<AppError> for Status {
    fn from(error: AppError) -> Self {
        Self::internal(error.0.to_string())
    }
}
