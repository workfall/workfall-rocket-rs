use rocket::serde::json::{Json};

use crate::services::index::T;

// import services module
use crate::services;

#[get("/")]
pub fn index() -> Json<T> {
    services::index::home()
}