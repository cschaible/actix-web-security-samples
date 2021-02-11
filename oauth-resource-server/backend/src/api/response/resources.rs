use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct UserResource {
    pub id: Uuid,
    pub version: i64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub admin: Option<bool>,
}

#[derive(Clone, Debug, Serialize)]
pub struct UserListResource {
    pub items: Vec<UserResource>,
}

#[derive(Clone, Debug, Serialize)]
pub struct UnregisteredUserResource {}
