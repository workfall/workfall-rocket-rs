use rocket::serde::json::{Value, Json};
use elonaire_backend_rs::{models::models::{UserInputRole}};

// import services module
use crate::services;

#[get("/users")]
pub fn get_users() -> Value {
    services::users::get_users()
}

#[post("/users/add-role", format = "json", data = "<role_info>")]
pub fn create_role(role_info: Json<UserInputRole>) -> Value {
    let id = uuid::Uuid::new_v4().to_string();
    let role_name = &role_info.role_name;

    services::users::add_role(&id, &role_name)
}