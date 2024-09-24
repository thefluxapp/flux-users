use serde::Serialize;
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;
use uuid::Uuid;


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialRequestOptions {
    pub public_key: PublicKeyCredentialRequestOptions,
}

#[serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialRequestOptions {
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub challenge: Vec<u8>,
    pub rp_id: Option<String>,
    pub allow_credentials: Vec<PublicKeyCredentialDescriptor>,
}

#[serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialDescriptor {
    pub id: String,
    #[serde(rename = "type")]
    pub tp: String,
    pub transports: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialCreationOptions {
    pub public_key: PublicKeyCredentialCreationOptions,
}

#[serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialCreationOptions {
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub challenge: Vec<u8>,
    pub pub_key_cred_params: Vec<PublicKeyCredentialParameters>,
    pub rp: PublicKeyCredentialRpEntity,
    pub user: PublicKeyCredentialUserEntity,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialParameters {
    pub alg: i16,
    #[serde(rename = "type")]
    pub tp: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialRpEntity {
    pub id: Option<String>,
    pub name: String,
}

#[serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialUserEntity {
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
}

// #[serde_as]
// #[derive(Deserialize, Debug)]
// pub struct PublicKeyCredentialWithAttestation {
//     pub response: AuthenticatorAttestationResponse,
//     pub id: String,
// }
