use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CreateUserResource {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateUserResource {
    pub version: i64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateAdminUserResource {
    pub version: i64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub admin: bool,
}
