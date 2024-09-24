use flux_auth_api::{auth_service_server::AuthService, JoinRequest, JoinResponse};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct GrpcAuthService {}

#[tonic::async_trait]
impl AuthService for GrpcAuthService {
    async fn join(&self, request: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
        let request = request.into_inner();
        let response = JoinResponse { bar: request.foo };

        Ok(Response::new(response))
    }
}
