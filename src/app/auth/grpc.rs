use anyhow::Error;
use flux_auth_api::{
    auth_service_server::AuthService, CompleteRequest, CompleteResponse, JoinRequest, JoinResponse,
};
use serde_json::json;
use tonic::{Request, Response, Status};
use validator::Validate;

use super::service;
use crate::app::{error::AppError, state::AppState};

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
        let response = join(&self.state, request.into_inner()).await?;

        Ok(Response::new(response))
    }

    async fn complete(
        &self,
        request: Request<CompleteRequest>,
    ) -> Result<Response<CompleteResponse>, Status> {
        let response = complete(&self.state, request.into_inner()).await?;

        Ok(Response::new(response))
    }
}

async fn join(
    AppState { settings, db, .. }: &AppState,
    request: JoinRequest,
) -> Result<JoinResponse, AppError> {
    let response = service::join(db, &settings.auth, request.try_into()?).await?;

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

async fn complete(
    AppState {
        settings,
        db,
        private_key,
    }: &AppState,
    request: CompleteRequest,
) -> Result<CompleteResponse, AppError> {
    let response = service::complete(db, settings, private_key, request.try_into()?).await?;

    Ok(response.into())
}

impl TryFrom<CompleteRequest> for service::CompleteRequest {
    type Error = Error;

    fn try_from(request: CompleteRequest) -> Result<Self, Self::Error> {
        let data: Self = serde_json::from_str(request.request())?;
        data.validate()?;

        Ok(data)
    }
}

impl Into<CompleteResponse> for service::CompleteResponse {
    fn into(self) -> CompleteResponse {
        CompleteResponse {
            jwt: Some(self.jwt),
        }
    }
}
