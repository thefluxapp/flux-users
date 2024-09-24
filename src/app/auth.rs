use flux_auth_api::auth_service_server::AuthServiceServer;
use grpc::GrpcAuthService;

mod grpc;

pub fn auth_service() -> AuthServiceServer<GrpcAuthService> {
    AuthServiceServer::new(GrpcAuthService::default())
}
