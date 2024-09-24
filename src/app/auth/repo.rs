use anyhow::Error;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, ModelTrait, QueryFilter,
};

pub mod user;
pub mod user_challenge;
pub mod user_credential;

pub async fn find_user_by_email_with_credentials<T: ConnectionTrait>(
    db: &T,
    email: &String,
) -> Result<Option<(user::Model, Vec<user_credential::Model>)>, Error> {
    match user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await?
    {
        Some(user) => {
            let user_credentials = user.find_related(user_credential::Entity).all(db).await?;

            Ok(Some((user, user_credentials)))
        }
        None => Ok(None),
    }
}

pub async fn create_user_challenge<T: ConnectionTrait>(
    db: &T,
    model: user_challenge::ActiveModel,
) -> Result<user_challenge::Model, Error> {
    let user_challenge = model.insert(db).await?;

    Ok(user_challenge)
}
