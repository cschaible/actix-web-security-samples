use std::sync::Arc;

use actix_web::{HttpResponse, Responder};
use actix_web::delete;
use actix_web::get;
use actix_web::post;
use actix_web::put;
use actix_web::web::{Json, Path};
use actix_web::web::Data;
use actix_web_security::user_details::UserDetails;

use crate::api::Identifier;
use crate::api::request::converter::{
    convert_to_new_user, convert_to_user, convert_to_user_as_admin,
};
use crate::api::request::resource::{
    CreateUserResource, UpdateAdminUserResource, UpdateUserResource,
};
use crate::api::response::resources::{UserListResource, UserResource};
use crate::api::response::resources::UnregisteredUserResource;
use crate::error::{ApplicationError, AuthorizationError, CustomDatabaseError};
use crate::error::ApplicationError::AuthorizeError;
use crate::model::unregistered_user::UnregisteredUser;
use crate::model::user::User;
use crate::repository::user_repository::UserRepository;
use crate::security::user_authorization_component::assert_is_admin;

type UserRepo = Data<Arc<Box<dyn UserRepository>>>;
type Response = Result<HttpResponse, ApplicationError>;

#[post("/users/current")]
pub async fn register_current_user(
    create_user_resource: Json<CreateUserResource>,
    user: UnregisteredUser,
    user_repository: UserRepo,
) -> Response {
    Ok(user_repository
        .create(&convert_to_new_user(&create_user_resource, user.user_id))
        .await
        .map(|created_user| HttpResponse::Created().json(UserResource::from(&created_user)))?)
}

#[put("/users/current")]
pub async fn update_current_user(
    update_user_resource: Json<UpdateUserResource>,
    user: User,
    user_repository: UserRepo,
) -> Response {
    Ok(user_repository
        .update(&convert_to_user(&update_user_resource, &user))
        .await
        .map(|updated_user| HttpResponse::Created().json(UserResource::from(&updated_user)))?)
}

#[get("/users/current")]
pub async fn get_current_user(user: Box<dyn UserDetails>) -> impl Responder {
    if user.is::<User>() {
        match user.downcast_ref::<User>() {
            Some(user) => HttpResponse::Ok().json(UserResource::from(user)),
            None => HttpResponse::InternalServerError().finish(),
        }
    } else {
        match user.downcast_ref::<UnregisteredUser>() {
            Some(user) => HttpResponse::Ok().json(UnregisteredUserResource::from(user)),
            None => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[delete("/users/current")]
pub async fn delete_current_user(user: User, user_repository: UserRepo) -> Response {
    Ok(user_repository
        .delete_by_identifier(user.identifier)
        .await
        .map(|_| HttpResponse::NoContent().finish())?)
}

#[get("/users")]
pub async fn find_all_users(user: User, user_repository: UserRepo) -> Response {
    assert_is_admin(user)?;
    Ok(user_repository
        .find_all()
        .await
        .map(|users| HttpResponse::Ok().json(UserListResource::from(users)))?)
}

#[put("/users/{id}")]
pub async fn update_user(
    id: Path<Identifier>,
    update_user_resource: Json<UpdateAdminUserResource>,
    user: User,
    user_repository: UserRepo,
) -> Response {
    assert_is_admin(user)?;
    match user_repository.find_by_identifier(id.id).await? {
        Some(existing_user) => Ok(user_repository
            .update(&convert_to_user_as_admin(
                &update_user_resource,
                &existing_user,
            ))
            .await
            .map(|updated_user| HttpResponse::Ok().json(UserResource::from(&updated_user)))
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    ApplicationError::CustomDbError(CustomDatabaseError::Conflict)
                }
                _ => ApplicationError::DbError(e),
            })?),
        None => Err(AuthorizeError(AuthorizationError::NotFound)),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(id: Path<Identifier>, user: User, user_repository: UserRepo) -> Response {
    assert_is_admin(user)?;
    Ok(user_repository
        .delete_by_identifier(id.id)
        .await
        .map(|_| HttpResponse::NoContent().finish())?)
}
