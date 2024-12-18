use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("RP_ID_MISSMATCH")]
    RpIdMissmatch,
    #[error("INVALID_CLIENT_DATA_TYPE")]
    InvalidClientDataType,
    #[error("UNPARSED_RP_ID")]
    UnparsedRpId(#[from] url::ParseError),
    #[error("INVALID_RP_ID")]
    InvalidRpId,
    #[error("USER_CHALLENGE_NOT_FOUND")]
    UserChallengeNotFound,
}
