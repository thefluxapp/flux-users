use std::sync::Arc;

use anyhow::Error;
use sea_orm::{ConnectOptions, Database, DbConn};

use super::settings::AppSettings;

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub db: Arc<DbConn>,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let opt = ConnectOptions::new(&settings.db.endpoint);
        // opt.sqlx_logging(true)
        //     .sqlx_logging_level(log::LevelFilter::Info);
        let db = Arc::new(Database::connect(opt).await?);

        Ok(Self { settings, db })
    }
}
