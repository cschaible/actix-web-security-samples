use actix_web_security::user_details::UserDetails;
use uuid::Uuid;

#[derive(sqlx::FromRow, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct User {
    pub id: i64,
    pub version: i64,
    pub identifier: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub user_id: String,
    pub admin: bool,
}

impl UserDetails for User {}

#[derive(Clone, Debug)]
pub struct NewUser {
    pub identifier: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub user_id: String,
}
