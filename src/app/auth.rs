use flux_users_api::auth_service_server::AuthServiceServer;
use grpc::GrpcAuthService;
use serde::Serialize;
use uuid::Uuid;

use super::state::AppState;

pub(super) mod error;
mod grpc;
mod passkey;
mod repo;
mod service;
pub(super) mod settings;

pub fn auth_service(state: AppState) -> AuthServiceServer<GrpcAuthService> {
    AuthServiceServer::new(GrpcAuthService::new(state))
}

#[derive(Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}
