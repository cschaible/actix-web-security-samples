use std::sync::Arc;

use actix_web_security::authentication::scheme::bearer::jwt::default_jwt::DefaultJwt;
use actix_web_security::authentication::scheme::bearer::jwt::token::Claims;
use actix_web_security::authentication::scheme::bearer::jwt::user_details_service::JwtUserDetailsService;
use actix_web_security::user_details::UserDetails;
use async_trait::async_trait;
use sqlx::error::Error::RowNotFound;

use crate::model::unregistered_user::UnregisteredUser;
use crate::repository::user_repository::UserRepository;

#[derive(Clone)]
pub struct JwtUserDetailsServiceImpl {
    pub(crate) user_repository: Arc<Box<dyn UserRepository>>,
}

#[async_trait]
impl JwtUserDetailsService for JwtUserDetailsServiceImpl {
    #[allow(clippy::borrowed_box)]
    async fn find_user(&self, token: &Box<dyn Claims>) -> Option<Box<dyn UserDetails>> {
        match token.downcast_ref::<DefaultJwt>() {
            Some(claims) => {
                let sub = claims.sub.clone().expect("sub expected");
                let found_user = self.user_repository.find_by_user_id(sub.clone()).await;
                match found_user {
                    Ok(user) => match user {
                        Some(u) => Some(Box::new(u)),
                        None => Some(Box::new(UnregisteredUser {
                            user_id: sub.clone(),
                        })),
                    },
                    Err(e) => match e {
                        RowNotFound => Some(Box::new(UnregisteredUser {
                            user_id: sub.clone(),
                        })),
                        _ => None,
                    },
                }
            }
            None => None,
        }
    }
}
