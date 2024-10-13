use anyhow::Error;
use sea_orm::DbConn;
use uuid::Uuid;

use super::repo;

pub async fn get_users(db: &DbConn, request: GetUsersRequest) -> Result<GetUsersResponse, Error> {
    let users = repo::find_users_by_ids(db, request.user_ids).await?;

    Ok(GetUsersResponse { users })
}

pub struct GetUsersRequest {
    pub user_ids: Vec<Uuid>,
}

pub struct GetUsersResponse {
    pub users: Vec<repo::user::Model>,
}
