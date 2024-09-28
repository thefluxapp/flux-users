use anyhow::Error;
use sea_orm::{
    ColumnTrait as _, ConnectionTrait, EntityTrait as _, Order, QueryFilter as _, QueryOrder as _,
};
use uuid::Uuid;

pub mod user;

pub async fn find_users_by_ids<T: ConnectionTrait>(
    db: &T,
    user_ids: Vec<Uuid>,
) -> Result<Vec<user::Model>, Error> {
    let users = user::Entity::find()
        .filter(user::Column::Id.is_in(user_ids))
        .order_by(user::Column::Id, Order::Asc)
        .all(db)
        .await?;

    Ok(users)
}
