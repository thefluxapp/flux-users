use anyhow::Error;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use passkey::types::{
    rand::random_vec,
    webauthn::{
        AttestationConveyancePreference, CredentialCreationOptions,
        PublicKeyCredentialCreationOptions, PublicKeyCredentialRpEntity,
        PublicKeyCredentialUserEntity,
    },
    Bytes,
};
use sea_orm::{sqlx::types::chrono::Utc, DbConn, Set};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use super::repo;

pub async fn join(db: &DbConn, request: JoinRequest) -> Result<JoinResponse, Error> {
    let email = request.email.to_lowercase();
    let user_id = Uuid::now_v7();
    let challenge: Bytes = random_vec(64).into();

    let response = match repo::find_user_by_email_with_credentials(db, &email).await? {
        Some(_) => todo!(),
        None => {
            let cco = PublicKeyCredentialCreationOptions {
                rp: PublicKeyCredentialRpEntity {
                    id: Some("QQ".into()),
                    name: "QQ".into(),
                },
                user: PublicKeyCredentialUserEntity {
                    id: user_id.as_bytes().as_slice().into(),
                    name: email.clone(),
                    display_name: email.clone(),
                },
                challenge: challenge.clone(),
                pub_key_cred_params: vec![],
                timeout: None,
                exclude_credentials: None,
                authenticator_selection: None,
                hints: None,
                attestation: AttestationConveyancePreference::None,
                attestation_formats: None,
                extensions: None,
            };

            repo::create_user_challenge(db, {
                repo::user_challenge::ActiveModel {
                    id: Set(URL_SAFE_NO_PAD.encode(challenge.as_slice())),
                    user_id: Set(user_id),
                    user_name: Set(Some(email)),
                    created_at: Set(Utc::now().naive_utc()),
                }
            })
            .await?;

            JoinResponse::Creation(CredentialCreationOptions { public_key: cco })
        }
    };

    Ok(response)
}

#[derive(Deserialize, Validate)]
pub struct JoinRequest {
    #[validate(email)]
    pub email: String,
}

pub enum JoinResponse {
    Creation(CredentialCreationOptions),
    // Request(CredentialRequestOptions),
}
