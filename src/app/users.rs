use flux_users_api::users_service_server::UsersServiceServer;
use grpc::GrpcUsersService;

use super::state::AppState;

mod grpc;
mod repo;
mod service;

pub fn users_service(state: AppState) -> UsersServiceServer<GrpcUsersService> {
    UsersServiceServer::new(GrpcUsersService::new(state))
}
