use rocket::serde::json::{Value};

// import services module
use crate::services;

#[get("/")]
pub fn index() -> Value {
    services::index::home()
}