use rocket::serde::json::{Json};
use rocket::serde::{Serialize, /*Deserialize*/ };

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct T { message: String }

pub fn home() -> Json<T> {
    Json(T { message: String::from("Welcome to my web API") })
}