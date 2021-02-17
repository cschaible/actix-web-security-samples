use std::env;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web_security::authentication::endpoint_matcher::AllEndpointsMatcher;
use actix_web_security::authentication::middleware::HttpAuthenticationMiddleware;
use actix_web_security::authentication::scheme::bearer::jwk::default_jwk_loader::load_default_rsa_jwks;
use actix_web_security::authentication::scheme::bearer::jwt::authentication_provider::JwtAuthenticationProvider;
use actix_web_security::authentication::scheme::bearer::jwt::header_extractor::BearerAuthenticationExtractor;
use actix_web_security::authentication::ProviderManager;
use jsonwebtoken::Algorithm;
use log::LevelFilter;
use sqlx::postgres::PgPoolOptions;
use sqlx::{migrate, migrate::Migrator, Pool, Postgres};

use api::controller;
use security::user_details_service::JwtUserDetailsServiceImpl;

use crate::error::ApplicationError;
use crate::repository::user_repository::{UserRepository, UserRepositoryImpl};

mod api;
mod error;
mod model;
mod repository;
pub mod security;

static MIGRATOR: Migrator = migrate!("./migrations");

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "warn")
    };

    env_logger::builder()
        .filter(Some("actix_oauth_example"), LevelFilter::Info)
        .filter(Some("actix_server::builder"), LevelFilter::Info)
        .init();

    let db_pool = init_db_and_migrate(&MIGRATOR).await?;
    let user_repository: Box<dyn UserRepository> =
        Box::new(UserRepositoryImpl::new(Arc::new(db_pool)));

    let jwks_url = env::var("JWKS_URL").expect("JWKS_URL env variable not set");
    let jwks =
        load_default_rsa_jwks(jwks_url, Algorithm::RS256).map_err(ApplicationError::JwkError)?;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(
                &env::var("ALLOWED_ORIGIN_URL").expect("ALLOWED_ORIGIN_URL env variable not set"),
            )
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let user_details_service = JwtUserDetailsServiceImpl {
            user_repository: Arc::new(user_repository.clone()),
        };

        let authentication_provider =
            JwtAuthenticationProvider::new(Box::new(user_details_service));

        let provider_manager = ProviderManager::new(vec![Box::new(authentication_provider)]);

        let authentication_extractor = BearerAuthenticationExtractor::new(jwks.clone());

        let authentication_middleware = HttpAuthenticationMiddleware::new(
            provider_manager,
            Box::new(authentication_extractor),
            Box::new(AllEndpointsMatcher::new()),
        );

        App::new()
            .data(Arc::new(user_repository.clone()))
            .wrap(authentication_middleware)
            .wrap(cors)
            .service(controller::update_current_user)
            .service(controller::delete_current_user)
            .service(controller::update_user)
            .service(controller::delete_user)
            .service(controller::register_current_user)
            .service(controller::get_current_user)
            .service(controller::find_all_users)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await?;

    Ok(())
}

async fn init_db_and_migrate(migrator: &Migrator) -> Result<Pool<Postgres>, anyhow::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable not set");

    let pool = PgPoolOptions::new()
        .max_connections(15)
        .min_connections(5)
        .connect(&database_url)
        .await?;

    migrator.run(&pool).await?;

    Ok(pool)
}
