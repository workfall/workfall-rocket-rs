#[macro_use] extern crate rocket;
use rocket::{serde::{json::{json, Value}}};

// add routes and services modules here
mod routes;
mod services;

// import routes here
use routes::index::index;
use routes::users::{get_users, create_role};

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_users, create_role])
    .register("/", catchers![not_found])
}
