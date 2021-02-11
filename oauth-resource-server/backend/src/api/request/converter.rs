use uuid::Uuid;

use crate::api::request::resource::{
    CreateUserResource, UpdateAdminUserResource, UpdateUserResource,
};
use crate::model::user::{NewUser, User};

pub fn convert_to_new_user(create_user_resource: &CreateUserResource, user_id: String) -> NewUser {
    NewUser {
        identifier: Uuid::new_v4(),
        first_name: create_user_resource.first_name.clone(),
        last_name: create_user_resource.last_name.clone(),
        user_id,
    }
}

pub fn convert_to_user(update_user_resource: &UpdateUserResource, existing_user: &User) -> User {
    User {
        id: existing_user.id,
        version: update_user_resource.version,
        identifier: existing_user.identifier,
        first_name: update_user_resource.first_name.clone(),
        last_name: update_user_resource.last_name.clone(),
        user_id: existing_user.user_id.clone(),
        admin: existing_user.admin,
    }
}

pub fn convert_to_user_as_admin(
    update_user_resource: &UpdateAdminUserResource,
    existing_user: &User,
) -> User {
    User {
        id: existing_user.id,
        version: update_user_resource.version,
        identifier: existing_user.identifier,
        first_name: update_user_resource.first_name.clone(),
        last_name: update_user_resource.last_name.clone(),
        user_id: existing_user.user_id.clone(),
        admin: update_user_resource.admin,
    }
}
