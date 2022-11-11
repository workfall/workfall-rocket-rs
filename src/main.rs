#[macro_use] extern crate rocket;
use rocket::{serde::{json::{json, Value}}};

// add routes and services modules here
mod routes;
mod services;

// import routes here
use routes::index::index;
use routes::users::{get_users, create_role, create_user, update_user};

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(500)]
fn server_error() -> Value {
    json!({
        "status": "Server error",
        "reason": "Something went wrong. Please try again later"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_users, create_role, create_user, update_user])
    .register("/", catchers![not_found, server_error])
}
