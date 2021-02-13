use std::env;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::error::BlockingError;
use actix_web::{web, App, HttpServer};
use actix_web_security::authentication::endpoint_matcher::AllEndpointsMatcher;
use actix_web_security::authentication::middleware::HttpAuthenticationMiddleware;
use actix_web_security::authentication::scheme::bearer::jwk::default_jwk_loader::load_default_rsa_jwks;
use actix_web_security::authentication::scheme::bearer::jwt::authentication_provider::JwtAuthenticationProvider;
use actix_web_security::authentication::scheme::bearer::jwt::header_extractor::BearerAuthenticationExtractor;
use actix_web_security::authentication::ProviderManager;
use jsonwebtoken::Algorithm;
use log::info;
use log::LevelFilter;
use refinery::config::{Config, ConfigDbType};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use url::Url;

use api::controller;
use security::user_details_service::JwtUserDetailsServiceImpl;

use crate::error::ApplicationError;
use crate::repository::user_repository::{UserRepository, UserRepositoryImpl};

mod api;
mod error;
mod model;
mod repository;
pub mod security;

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!();
}

async fn migrate(url: String) -> Result<(), BlockingError<anyhow::Error>> {
    web::block(move || {
        let parsed_url = Url::parse(url.as_str())?;

        let mut connection = Config::new(ConfigDbType::Postgres)
            .set_db_host(parsed_url.host_str().unwrap())
            .set_db_name(&parsed_url.path().to_string()[1..])
            .set_db_pass(parsed_url.password().unwrap())
            .set_db_port(&format!("{}", parsed_url.port().unwrap()))
            .set_db_user(parsed_url.username());

        info!("Migrate Database");
        embedded::migrations::runner().run(&mut connection)?;
        Ok(())
    })
    .await
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "warn")
    };

    env_logger::builder()
        .filter(Some("actix_oauth_example"), LevelFilter::Info)
        .filter(Some("actix_server::builder"), LevelFilter::Info)
        .filter(Some("refinery_core::traits"), LevelFilter::Info)
        .filter(Some("refinery_core::traits::sync"), LevelFilter::Info)
        .init();

    let db_pool = init_db_and_migrate().await?;
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

async fn init_db_and_migrate() -> Result<Pool<Postgres>, anyhow::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable not set");

    migrate(database_url.clone()).await?;

    Ok(PgPoolOptions::new()
        .max_connections(15)
        .min_connections(5)
        .connect(&database_url)
        .await?)
}
