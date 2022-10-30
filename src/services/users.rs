use rocket::{serde::{json::{json, Value}}};
use elonaire_backend_rs::{*, models::models::{User, NewRole, Role}};
use diesel::prelude::*;

pub fn get_users() -> Value {
    use elonaire_backend_rs::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let results = users
        .load::<User>(connection)
        .expect("Error loading posts");

    json!(results)
}

pub fn add_role(id: &str, role_name: &str) -> Value {
    use elonaire_backend_rs::schema::roles;

    let connection = &mut establish_connection();

    let new_role = NewRole {id, role_name};

    let created_role = diesel::insert_into(roles::table)
        .values(&new_role)
        .get_result::<Role>(connection)
        .expect("Error saving new role");

    json!(created_role)
}