use actix_http::http::{header, StatusCode};
use actix_web::{error, HttpResponse};
use actix_web::dev::HttpResponseBuilder;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AuthorizationError {
    #[display(fmt = "Access denied")]
    NotAdmin,
    #[display(fmt = "Not found")]
    NotFound,
}

#[derive(Debug, Display, Error)]
pub enum CustomDatabaseError {
    #[display(fmt = "Resource outdated")]
    Conflict,
}

#[derive(Debug, Display, Error)]
pub enum ApplicationError {
    CustomDbError(CustomDatabaseError),
    DbError(sqlx::Error),
    AuthorizeError(AuthorizationError),
}

impl error::ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApplicationError::CustomDbError(db_error) => match db_error {
                CustomDatabaseError::Conflict => StatusCode::CONFLICT,
            },
            ApplicationError::DbError(db_error) => match db_error {
                sqlx::Error::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::Tls(_) => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::Protocol(_) => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::RowNotFound => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::ColumnIndexOutOfBounds { .. } => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::ColumnNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::ColumnDecode { .. } => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::Decode(_) => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::PoolTimedOut => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::PoolClosed => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::WorkerCrashed => StatusCode::INTERNAL_SERVER_ERROR,
                sqlx::Error::Migrate(_) => StatusCode::INTERNAL_SERVER_ERROR,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            ApplicationError::AuthorizeError(auth_error) => match auth_error {
                AuthorizationError::NotAdmin => StatusCode::FORBIDDEN,
                AuthorizationError::NotFound => StatusCode::NOT_FOUND,
            },
        }
    }
}

impl From<AuthorizationError> for ApplicationError {
    fn from(e: AuthorizationError) -> Self {
        ApplicationError::AuthorizeError(e)
    }
}

impl From<sqlx::Error> for ApplicationError {
    fn from(e: sqlx::Error) -> Self {
        ApplicationError::DbError(e)
    }
}

impl From<CustomDatabaseError> for ApplicationError {
    fn from(e: CustomDatabaseError) -> Self {
        ApplicationError::CustomDbError(e)
    }
}
