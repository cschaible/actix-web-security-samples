use std::future::Future;
use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::dev::PayloadStream;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web_security::authentication::error::error_type::AuthenticationError;
use actix_web_security::user_details::UserDetails;

use crate::model::unregistered_user::UnregisteredUser;
use crate::model::user::User;

impl FromRequest for User {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload<PayloadStream>) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let extensions = req.extensions();
            let user_details = extensions.get::<Box<dyn UserDetails>>();

            let user = match user_details.cloned() {
                Some(boxed_user_details) => {
                    let cloned_user_details_box = boxed_user_details.clone_box();
                    cloned_user_details_box.downcast_ref::<User>().cloned()
                }
                None => None,
            };
            match user {
                Some(u) => Ok(u),
                None => Err(AuthenticationError::UsernameNotFound.into()),
            }
        })
    }
}

impl FromRequest for UnregisteredUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload<PayloadStream>) -> Self::Future {
        let req = req.clone();
        let found_user = req
            .extensions()
            .get::<Box<dyn UserDetails>>()
            .map(
                |found_user| match found_user.clone_box().downcast_ref::<UnregisteredUser>() {
                    Some(user) => Ok(user.clone()),
                    None => Err(AuthenticationError::UsernameNotFound.into()),
                },
            );

        Box::pin(async move {
            match found_user {
                Some(user) => user,
                None => Err(AuthenticationError::UsernameNotFound.into()),
            }
        })
    }
}
