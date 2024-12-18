use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel as _,
    ModelTrait, QueryFilter, QuerySelect as _,
};
use uuid::Uuid;

pub mod user;
pub mod user_challenge;
pub mod user_credential;

pub async fn find_user_by_id<T: ConnectionTrait>(
    db: &T,
    id: Uuid,
) -> Result<Option<user::Model>, DbErr> {
    Ok(user::Entity::find_by_id(id).one(db).await?)
}

pub async fn find_user_by_email_with_credentials<T: ConnectionTrait>(
    db: &T,
    email: &String,
) -> Result<Option<(user::Model, Vec<user_credential::Model>)>, DbErr> {
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
) -> Result<user_challenge::Model, DbErr> {
    let user_challenge = model.insert(db).await?;

    Ok(user_challenge)
}

pub async fn find_user_challengle<T: ConnectionTrait>(
    db: &T,
    id: &String,
) -> Result<Option<user_challenge::Model>, DbErr> {
    Ok(user_challenge::Entity::find_by_id(id)
        .lock_exclusive()
        .one(db)
        .await?)
}

pub async fn create_user<T: ConnectionTrait>(
    db: &T,
    model: user::Model,
) -> Result<user::Model, DbErr> {
    let user = model.into_active_model().insert(db).await?;

    Ok(user)
}

pub async fn create_user_credential<T: ConnectionTrait>(
    db: &T,
    model: user_credential::Model,
) -> Result<user_credential::Model, DbErr> {
    let user = model.into_active_model().insert(db).await?;

    Ok(user)
}

pub async fn delete_user_challengle<T: ConnectionTrait>(
    db: &T,
    model: user_challenge::Model,
) -> Result<(), DbErr> {
    model.delete(db).await?;

    Ok(())
}
