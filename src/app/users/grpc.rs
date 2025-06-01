use anyhow::Error;
use flux_users_api::{
    get_users_response::User, users_service_server::UsersService, GetUsersRequest, GetUsersResponse,
};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::app::{error::AppError, state::AppState};

use super::service;

pub struct GrpcUsersService {
    pub state: AppState,
}

impl GrpcUsersService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl UsersService for GrpcUsersService {
    async fn get_users(
        &self,
        request: Request<GetUsersRequest>,
    ) -> Result<Response<GetUsersResponse>, Status> {
        let response = get_users(&self.state, request.into_inner()).await?;

        Ok(Response::new(response))
    }
}

async fn get_users(
    state: &AppState,
    request: GetUsersRequest,
) -> Result<GetUsersResponse, AppError> {
    let AppState { db, .. } = state;
    let response = service::get_users(&db, request.try_into()?).await?;

    Ok(response.into())
}

impl TryFrom<GetUsersRequest> for service::GetUsersRequest {
    type Error = AppError;

    fn try_from(request: GetUsersRequest) -> Result<Self, Self::Error> {
        let data = Self {
            user_ids: request
                .user_ids
                .iter()
                .map(|user_id| -> Result<Uuid, Error> { Ok(Uuid::parse_str(user_id)?) })
                .collect::<Result<Vec<Uuid>, Error>>()?,
        };

        Ok(data)
    }
}

impl Into<GetUsersResponse> for service::GetUsersResponse {
    fn into(self) -> GetUsersResponse {
        GetUsersResponse {
            users: self
                .users
                .iter()
                .map(|user| User {
                    user_id: Some(user.id.into()),
                    first_name: Some(user.first_name.clone()),
                    last_name: Some(user.last_name.clone()),
                    locale: user.locale.clone(),
                    name: Some(user.name()),
                    abbr: Some(user.abbr()),
                    color: Some(user.color()),
                })
                .collect(),
        }
    }
}
