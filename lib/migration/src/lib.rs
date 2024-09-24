pub use sea_orm_migration::prelude::*;

mod m20240924_105951_create_users;
mod m20240924_110240_create_user_credentials;
mod m20240924_110302_create_user_challenges;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240924_105951_create_users::Migration),
            Box::new(m20240924_110240_create_user_credentials::Migration),
            Box::new(m20240924_110302_create_user_challenges::Migration),
        ]
    }
}
