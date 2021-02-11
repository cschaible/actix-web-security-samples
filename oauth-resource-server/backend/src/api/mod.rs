use serde::Deserialize;
use uuid::Uuid;

pub mod controller;
pub mod extractors;
pub mod request;
pub mod response;

#[derive(Deserialize)]
pub struct Identifier {
    pub id: Uuid,
}
