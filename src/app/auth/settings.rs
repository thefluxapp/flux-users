use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AuthSettings {
    pub rp: RPSettings,
    pub private_key_file: String,
}

#[derive(Deserialize, Clone)]
pub struct RPSettings {
    pub id: String,
    pub name: String,
}
