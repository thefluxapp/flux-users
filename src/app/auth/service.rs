use anyhow::{bail, Error};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::RngCore as _;

use sea_orm::{DbConn, Set, TransactionTrait as _};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;
use validator::Validate;

use crate::app::settings::AppSettings;

use super::{
    passkey::{
        CredentialCreationOptions, PublicKeyCredentialCreationOptions,
        PublicKeyCredentialParameters, PublicKeyCredentialRpEntity, PublicKeyCredentialUserEntity,
        PublicKeyCredentialWithAttestation,
    },
    repo,
    settings::AuthSettings,
    Claims,
};

pub async fn join(
    db: &DbConn,
    settings: &AuthSettings,
    request: JoinRequest,
) -> Result<JoinResponse, Error> {
    let email = request.email.to_lowercase();
    let user_id = Uuid::now_v7();
    let mut challenge = vec![0u8; 128];
    rand::thread_rng().fill_bytes(&mut challenge);

    let response = match repo::find_user_by_email_with_credentials(db, &email).await? {
        Some(_) => todo!(),
        None => {
            let public_key = PublicKeyCredentialCreationOptions {
                rp: PublicKeyCredentialRpEntity {
                    id: Some(settings.rp.id.clone()),
                    name: settings.rp.name.clone(),
                },
                user: PublicKeyCredentialUserEntity {
                    id: user_id,
                    name: email.clone(),
                    display_name: email.clone(),
                },
                challenge,
                // TODO: remove hardcode
                pub_key_cred_params: vec![
                    PublicKeyCredentialParameters {
                        alg: -7,
                        tp: "public-key".to_string(),
                    },
                    PublicKeyCredentialParameters {
                        alg: -257,
                        tp: "public-key".to_string(),
                    },
                ],
            };

            repo::create_user_challenge(db, {
                repo::user_challenge::ActiveModel {
                    id: Set(URL_SAFE_NO_PAD.encode(public_key.challenge.clone())),
                    user_id: Set(public_key.user.id),
                    user_name: Set(Some(public_key.user.name.clone())),
                    created_at: Set(Utc::now().naive_utc()),
                }
            })
            .await?;

            JoinResponse::Creation(CredentialCreationOptions { public_key })
        }
    };

    Ok(response)
}

#[derive(Deserialize, Validate)]
pub struct JoinRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum JoinResponse {
    Creation(CredentialCreationOptions),
    // Request(CredentialRequestOptions),
}

pub async fn complete(
    db: &DbConn,
    settings: &AppSettings,
    private_key: &Vec<u8>,
    request: CompleteRequest,
) -> Result<CompleteResponse, Error> {
    let client_data = request.credential.response.client_data;
    dbg!(&client_data);

    validate_origin(&client_data.origin, &settings.auth.rp.id)?;
    validate_tp(&client_data.tp, "webauthn.create")?;

    let txn = db.begin().await?;

    let user_challenge = match repo::find_user_challengle(&txn, &client_data.challenge).await? {
        Some(user_challenge) => user_challenge,
        None => bail!("user_challenge not found"),
    };

    let email = match user_challenge.user_name.clone() {
        Some(email) => email,
        None => bail!("user_challenge without email"),
    };

    let user = repo::create_user(
        &txn,
        repo::user::Model {
            id: user_challenge.user_id,
            email,
            first_name: Some(request.first_name.clone()),
            last_name: Some(request.last_name.clone()),
            // passkeys: todo!(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        },
    )
    .await?;

    repo::create_user_credential(
        &txn,
        repo::user_credential::Model {
            id: request.credential.id,
            user_id: user.id,
            public_key: request.credential.response.public_key,
            public_key_algorithm: request.credential.response.public_key_algorithm,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        },
    )
    .await?;

    repo::delete_user_challengle(&txn, user_challenge).await?;

    txn.commit().await?;

    println!("QQQ: {}", &request.first_name);
    let jwt = create_jwt(&private_key, &user)?;

    Ok(CompleteResponse { jwt })
}

#[derive(Deserialize, Validate)]
pub struct CompleteRequest {
    pub first_name: String,
    pub last_name: String,
    pub credential: PublicKeyCredentialWithAttestation,
}

#[derive(Serialize)]
pub struct CompleteResponse {
    pub jwt: String,
}

pub async fn me(db: &DbConn, request: MeRequest) -> Result<MeResponse, Error> {
    println!("QQQ: {}", request.user_id);

    Ok(MeResponse {
        name: "QQQ".to_string(),
    })
}

#[derive(Deserialize, Validate)]
pub struct MeRequest {
    pub user_id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct MeResponse {
    pub name: String,
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

fn validate_origin(origin: &str, expected: &str) -> Result<(), Error> {
    // TODO: rewrite it!

    match Url::parse(&origin) {
        Ok(url) => match url.host() {
            Some(host) => {
                if host.to_string() == expected {
                    Ok(())
                } else {
                    bail!("ORIGIN")
                }
            }
            None => bail!("ORIGIN"),
        },
        Err(_) => bail!("ORIGIN"),
    }
}

fn validate_tp(tp: &str, expected: &str) -> Result<(), Error> {
    if tp != expected {
        bail!("TP")
    } else {
        Ok(())
    }
}
