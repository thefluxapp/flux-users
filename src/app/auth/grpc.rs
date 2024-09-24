use anyhow::Error;
use flux_auth_api::{auth_service_server::AuthService, JoinRequest, JoinResponse};
use sea_orm::DbConn;
use serde_json::json;
use tonic::{Request, Response, Status};
use validator::Validate;

use crate::app::{error::AppError, state::AppState};

use super::service;

pub struct GrpcAuthService {
    pub state: AppState,
}

impl GrpcAuthService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl AuthService for GrpcAuthService {
    async fn join(&self, request: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
        let response = join(&self.state.db, request.into_inner()).await?;

        Ok(Response::new(response))
    }
}

async fn join(db: &DbConn, request: JoinRequest) -> Result<JoinResponse, AppError> {
    let response = service::join(db, request.try_into()?).await?;

    Ok(response.into())
}

impl TryFrom<JoinRequest> for service::JoinRequest {
    type Error = Error;

    fn try_from(request: JoinRequest) -> Result<Self, Self::Error> {
        let data = Self {
            email: request.email().into(),
        };
        data.validate()?;

        Ok(data)
    }
}

impl Into<JoinResponse> for service::JoinResponse {
    fn into(self) -> JoinResponse {
        JoinResponse {
            response: Some(json!(self).to_string()),
        }
    }
}
