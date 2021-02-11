use crate::api::response::resources::{UnregisteredUserResource, UserListResource, UserResource};
use crate::model::unregistered_user::UnregisteredUser;
use crate::model::user::User;

impl From<&User> for UserResource {
    fn from(user: &User) -> Self {
        UserResource {
            id: user.identifier,
            version: user.version,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            admin: Some(user.admin),
        }
    }
}

impl From<Vec<User>> for UserListResource {
    fn from(users: Vec<User>) -> Self {
        let mut user_resources: Vec<UserResource> = Vec::with_capacity(users.len());
        for user in users {
            user_resources.push(UserResource::from(&user))
        }
        UserListResource {
            items: user_resources,
        }
    }
}

impl From<&UnregisteredUser> for UnregisteredUserResource {
    fn from(_: &UnregisteredUser) -> Self {
        UnregisteredUserResource {}
    }
}
