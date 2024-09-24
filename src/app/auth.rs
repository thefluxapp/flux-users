use flux_auth_api::auth_service_server::AuthServiceServer;
use grpc::GrpcAuthService;

use super::state::AppState;

mod grpc;
mod repo;
mod service;

pub fn auth_service(state: AppState) -> AuthServiceServer<GrpcAuthService> {
    AuthServiceServer::new(GrpcAuthService::new(state))
}
