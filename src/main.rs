#[macro_use] extern crate rocket;

// add our routes and services modules
mod routes;
mod services;

// import our routes
use routes::index::index;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
