pub use sea_orm_migration::prelude::*;

mod m20240924_105951_create_users;
mod m20240924_110240_create_user_credentials;
mod m20240924_110302_create_user_challenges;
mod m20240928_165536_create_indexes;
mod m20250601_184245_add_locale_to_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240924_105951_create_users::Migration),
            Box::new(m20240924_110240_create_user_credentials::Migration),
            Box::new(m20240924_110302_create_user_challenges::Migration),
            Box::new(m20240928_165536_create_indexes::Migration),
            Box::new(m20250601_184245_add_locale_to_users::Migration),
        ]
    }
}
