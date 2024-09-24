use anyhow::Error;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::RngCore as _;

use sea_orm::{sqlx::types::chrono::Utc, DbConn, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::{
    passkey::{
        CredentialCreationOptions, PublicKeyCredentialCreationOptions,
        PublicKeyCredentialParameters, PublicKeyCredentialRpEntity, PublicKeyCredentialUserEntity,
    },
    repo,
};

pub async fn join(db: &DbConn, request: JoinRequest) -> Result<JoinResponse, Error> {
    let email = request.email.to_lowercase();
    let user_id = Uuid::now_v7();
    let mut challenge = vec![0u8; 128];
    rand::thread_rng().fill_bytes(&mut challenge);

    let response = match repo::find_user_by_email_with_credentials(db, &email).await? {
        Some(_) => todo!(),
        None => {
            let public_key = PublicKeyCredentialCreationOptions {
                rp: PublicKeyCredentialRpEntity {
                    id: Some("QQ".into()),
                    name: "QQ".into(),
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
