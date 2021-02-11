use actix_web_security::user_details::UserDetails;

#[derive(Clone, Debug)]
pub struct UnregisteredUser {
    pub user_id: String,
}

impl UserDetails for UnregisteredUser {}
