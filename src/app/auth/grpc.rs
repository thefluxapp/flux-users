use flux_auth_api::{
    auth_service_server::AuthService, CompleteRequest, CompleteResponse, JoinRequest, JoinResponse,
    MeRequest, MeResponse,
};
use tonic::{Request, Response, Status};

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

    async fn me(&self, request: Request<MeRequest>) -> Result<Response<MeResponse>, Status> {
        let response = me(&self.state, request.into_inner()).await?;

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

mod join {
    use flux_auth_api::{JoinRequest, JoinResponse};
    use serde_json::json;
    use validator::Validate as _;

    use crate::app::{
        auth::service::join::{Request, Response},
        error::AppError,
    };

    impl TryFrom<JoinRequest> for Request {
        type Error = AppError;

        fn try_from(request: JoinRequest) -> Result<Self, Self::Error> {
            let data = Self {
                email: request.email().into(),
            };
            data.validate()?;

            Ok(data)
        }
    }

    impl From<Response> for JoinResponse {
        fn from(res: Response) -> Self {
            JoinResponse {
                response: Some(json!(res).to_string()),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use anyhow::Error;
        use mry::ArgMatcher::Any;

        use crate::app::{
            auth::{grpc::join, service},
            state::AppState,
        };

        use super::*;

        #[test]
        fn should_validate_email() -> Result<(), Error> {
            let email = "email@theflux.app";

            let req: Request = JoinRequest {
                email: Some(email.into()),
            }
            .try_into()?;

            assert_eq!(req.email, email);

            Ok(())
        }

        #[tokio::test]
        #[mry::lock(service::join)]
        async fn test() -> Result<(), Error> {
            let join_request = JoinRequest {
                email: Some("email@theflux.app".into()),
            };
            let req: Request = join_request.clone().try_into()?;
            service::mock_join(Any, Any, req).returns_once(Ok(Response::default()));

            let res = join(&AppState::default(), join_request).await?;

            assert!(res.response.is_some());

            Ok(())
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

mod complete {
    use flux_auth_api::{CompleteRequest, CompleteResponse};
    use validator::Validate as _;

    use crate::app::{
        auth::service::complete::{Request, Response},
        error::AppError,
    };

    impl TryFrom<CompleteRequest> for Request {
        type Error = AppError;

        fn try_from(request: CompleteRequest) -> Result<Self, Self::Error> {
            let data: Self = serde_json::from_str(request.request())?;
            data.validate()?;

            Ok(data)
        }
    }

    impl From<Response> for CompleteResponse {
        fn from(res: Response) -> Self {
            CompleteResponse { jwt: Some(res.jwt) }
        }
    }
}

async fn me(AppState { db, .. }: &AppState, request: MeRequest) -> Result<MeResponse, AppError> {
    let response = service::me(db, request.try_into()?).await?;

    Ok(response.try_into()?)
}

mod me {
    use flux_auth_api::{me_response::User, MeRequest, MeResponse};
    use uuid::Uuid;
    use validator::{Validate as _, ValidationErrors};

    use crate::app::{auth::service, error::AppError};

    impl TryFrom<MeRequest> for service::me::Request {
        type Error = AppError;

        fn try_from(request: MeRequest) -> Result<Self, Self::Error> {
            let data = Self {
                user_id: Uuid::parse_str(request.user_id())
                    .map_err(|_| AppError::Validation(ValidationErrors::new()))?,
            };
            data.validate()?;

            Ok(data)
        }
    }

    impl TryInto<MeResponse> for service::me::Response {
        type Error = AppError;

        fn try_into(self) -> Result<MeResponse, Self::Error> {
            let user = self.user.ok_or(AppError::NotFound)?;

            Ok(MeResponse {
                user: Some(User {
                    user_id: Some(user.id.into()),
                    first_name: Some(user.first_name.clone()),
                    last_name: Some(user.last_name.clone()),
                    name: Some(user.name()),
                    abbr: Some(user.abbr()),
                    color: Some(user.color()),
                }),
            })
        }
    }
}
