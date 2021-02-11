use crate::error::AuthorizationError;
use crate::model::user::User;

pub fn assert_is_admin(user: User) -> Result<(), AuthorizationError> {
    match user.admin {
        true => Ok(()),
        false => Err(AuthorizationError::NotAdmin),
    }
}
