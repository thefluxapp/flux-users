use anyhow::Error;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sea_orm::{DbConn, Set, TransactionTrait as _};
use url::Url;

use crate::app::auth::passkey::ClientDataType;

use super::{
    error::AuthError,
    passkey::{PublicKeyCredentialCreationOptions, PublicKeyCredentialRequestOptions},
    repo,
    settings::AuthSettings,
    Claims,
};

#[mry::mry]
pub async fn join(
    db: &DbConn,
    settings: &AuthSettings,
    req: join::Request,
) -> Result<join::Response, Error> {
    let res = match repo::find_user_by_email_with_credentials(db, &req.email).await? {
        Some(user_with_credentials) => {
            let public_key: PublicKeyCredentialRequestOptions =
                (user_with_credentials, settings).into();

            public_key.into()
        }
        None => {
            let public_key: PublicKeyCredentialCreationOptions = (req, settings).into();

            repo::create_user_challenge(db, {
                repo::user_challenge::ActiveModel {
                    id: Set(URL_SAFE_NO_PAD.encode(public_key.challenge.clone())),
                    user_id: Set(public_key.user.id),
                    user_name: Set(public_key.user.name.clone()),
                    created_at: Set(Utc::now().naive_utc()),
                }
            })
            .await?;

            public_key.into()
        }
    };

    Ok(res)
}

pub mod join {
    use coset::iana::{self, EnumI64};
    use rand::RngCore as _;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    use validator::Validate;

    use crate::app::auth::{
        passkey::{
            CredentialCreationOptions, CredentialRequestOptions,
            PublicKeyCredentialCreationOptions, PublicKeyCredentialDescriptor,
            PublicKeyCredentialParameters, PublicKeyCredentialRequestOptions,
            PublicKeyCredentialRpEntity, PublicKeyCredentialType, PublicKeyCredentialUserEntity,
        },
        repo,
        settings::AuthSettings,
    };

    #[derive(Deserialize, Validate, Clone, PartialEq)]
    pub struct Request {
        #[validate(email)]
        pub email: String,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Response {
        Creation(CredentialCreationOptions),
        Request(CredentialRequestOptions),
    }

    impl
        From<(
            (repo::user::Model, Vec<repo::user_credential::Model>),
            &AuthSettings,
        )> for PublicKeyCredentialRequestOptions
    {
        fn from(
            ((_, user_credentials), settings): (
                (repo::user::Model, Vec<repo::user_credential::Model>),
                &AuthSettings,
            ),
        ) -> Self {
            let mut challenge = vec![0u8; 128];
            rand::thread_rng().fill_bytes(&mut challenge);

            Self {
                challenge,
                rp_id: Some(settings.rp.id.clone()),
                allow_credentials: user_credentials
                    .into_iter()
                    .map(|it| PublicKeyCredentialDescriptor {
                        id: it.id,
                        tp: PublicKeyCredentialType::PublicKey,
                        transports: vec!["internal".to_string(), "hybrid".to_string()],
                    })
                    .collect(),
                user_verification: "preferred".into(),
            }
        }
    }

    impl From<(Request, &AuthSettings)> for PublicKeyCredentialCreationOptions {
        fn from((req, settings): (Request, &AuthSettings)) -> Self {
            let user_id = Uuid::now_v7();
            let mut challenge = vec![0u8; 128];
            rand::thread_rng().fill_bytes(&mut challenge);

            Self {
                challenge,
                pub_key_cred_params: vec![
                    PublicKeyCredentialParameters {
                        alg: iana::Algorithm::RS256.to_i64(),
                        tp: PublicKeyCredentialType::PublicKey,
                    },
                    PublicKeyCredentialParameters {
                        alg: iana::Algorithm::ES256.to_i64(),
                        tp: PublicKeyCredentialType::PublicKey,
                    },
                    PublicKeyCredentialParameters {
                        alg: iana::Algorithm::EdDSA.to_i64(),
                        tp: PublicKeyCredentialType::PublicKey,
                    },
                ],
                rp: PublicKeyCredentialRpEntity {
                    id: Some(settings.rp.id.clone()),
                    name: settings.rp.name.clone(),
                },
                user: PublicKeyCredentialUserEntity {
                    id: user_id,
                    name: req.email.clone(),
                    display_name: req.email.clone(),
                },
            }
        }
    }

    impl From<PublicKeyCredentialCreationOptions> for Response {
        fn from(public_key: PublicKeyCredentialCreationOptions) -> Self {
            Self::Creation(CredentialCreationOptions { public_key })
        }
    }

    impl From<PublicKeyCredentialRequestOptions> for Response {
        fn from(public_key: PublicKeyCredentialRequestOptions) -> Self {
            Self::Request(CredentialRequestOptions { public_key })
        }
    }

    #[cfg(test)]
    mod tests {
        use uuid::Uuid;

        use crate::app::auth::passkey::{
            CredentialCreationOptions, PublicKeyCredentialCreationOptions,
            PublicKeyCredentialRpEntity, PublicKeyCredentialUserEntity,
        };

        use super::Response;

        impl Response {
            pub fn default() -> Self {
                Self::Creation(CredentialCreationOptions {
                    public_key: PublicKeyCredentialCreationOptions {
                        challenge: vec![],
                        pub_key_cred_params: vec![],
                        rp: PublicKeyCredentialRpEntity {
                            id: Some(String::default()),
                            name: String::default(),
                        },
                        user: PublicKeyCredentialUserEntity {
                            id: Uuid::now_v7(),
                            name: String::default(),
                            display_name: String::default(),
                        },
                    },
                })
            }
        }
    }
}

pub mod login {
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Deserialize, Validate, Clone, PartialEq)]
    pub struct Request {}

    #[derive(Debug, Serialize)]
    pub struct Response {
        pub jwt: String,
    }
}

pub async fn complete(
    db: &DbConn,
    settings: &AuthSettings,
    private_key: &Vec<u8>,
    req: complete::Request,
) -> Result<complete::Response, Error> {
    dbg!(&req);

    let client_data = req.credential.response.client_data;

    validate_origin(&client_data.origin, &settings.rp.id)?;
    validate_tp(client_data.tp, ClientDataType::Create)?;

    let txn = db.begin().await?;

    let user_challenge = repo::find_user_challengle(&txn, &client_data.challenge)
        .await?
        .ok_or(AuthError::UserChallengeNotFound)?;

    let user = repo::create_user(
        &txn,
        repo::user::Model {
            id: user_challenge.user_id.clone(),
            email: user_challenge.user_name.clone(),
            first_name: req.first_name.clone(),
            last_name: req.last_name.clone(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        },
    )
    .await?;

    repo::create_user_credential(
        &txn,
        repo::user_credential::Model {
            id: req.credential.id,
            user_id: user.id,
            public_key: req.credential.response.public_key,
            public_key_algorithm: req.credential.response.public_key_algorithm,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        },
    )
    .await?;

    repo::delete_user_challengle(&txn, user_challenge).await?;

    txn.commit().await?;

    Ok(complete::Response {
        jwt: create_jwt(&private_key, &user)?,
    })
}

pub mod complete {
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    use crate::app::auth::passkey::PublicKeyCredentialWithAttestation;

    #[derive(Debug, Deserialize, Validate)]
    pub struct Request {
        pub first_name: String,
        pub last_name: String,
        pub credential: PublicKeyCredentialWithAttestation,
    }

    #[derive(Serialize, Debug)]
    pub struct Response {
        pub jwt: String,
    }

    #[cfg(test)]
    mod tests {
        use anyhow::Error;
        use sea_orm::DatabaseConnection;

        use crate::app::auth::{
            passkey::{
                AuthenticatorAttestationResponse, ClientData, ClientDataType,
                PublicKeyCredentialWithAttestation,
            },
            service::{self, complete::Request},
            settings::{AuthSettings, RPSettings},
        };

        #[tokio::test]
        async fn example() -> Result<(), Error> {
            let db = DatabaseConnection::default();
            let req = Request {
                first_name: "FIRST_NAME".into(),
                last_name: "LAST_NAME".into(),
                credential: PublicKeyCredentialWithAttestation {
                    response: AuthenticatorAttestationResponse {
                        client_data: ClientData {
                            tp: ClientDataType::Create,
                            challenge: String::default(),
                            origin: String::default(),
                        },
                        public_key: vec![],
                        public_key_algorithm: 0,
                    },
                    id: "ID".into(),
                },
            };
            let settings = AuthSettings {
                rp: RPSettings {
                    id: String::default(),
                    name: String::default(),
                },
                private_key_file: String::default(),
            };

            let res = service::complete(&db, &settings, &vec![], req).await;

            dbg!(&res);

            Ok(())
        }
    }
}

pub async fn me(db: &DbConn, request: me::Request) -> Result<me::Response, Error> {
    let user = repo::find_user_by_id(db, request.user_id).await?;

    Ok(me::Response { user })
}

pub mod me {
    use serde::Deserialize;
    use uuid::Uuid;
    use validator::Validate;

    use crate::app::auth::repo::user;

    #[derive(Deserialize, Validate)]
    pub struct Request {
        pub user_id: Uuid,
    }

    #[derive(Deserialize, Validate)]
    pub struct Response {
        pub user: Option<user::Model>,
    }
}

pub fn create_jwt(private_key: &Vec<u8>, user: &repo::user::Model) -> Result<String, Error> {
    let claims = Claims {
        sub: user.id,
        exp: (Utc::now() + Duration::days(300)).timestamp().try_into()?,
    };

    let jwt = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(private_key)?,
    )?;

    Ok(jwt)
}

fn validate_origin(origin: &str, expected: &str) -> Result<(), AuthError> {
    let url = Url::parse(origin).map_err(AuthError::UnparsedRpId)?;
    let host = url.host().ok_or(AuthError::InvalidRpId)?;

    if host.to_string() != expected {
        return Err(AuthError::RpIdMissmatch);
    };

    Ok(())
}

fn validate_tp(tp: ClientDataType, expected: ClientDataType) -> Result<(), AuthError> {
    if tp != expected {
        return Err(AuthError::InvalidClientDataType);
    }

    Ok(())
}
