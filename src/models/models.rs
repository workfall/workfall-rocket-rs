use diesel::{prelude::*};
use rocket::serde::{Serialize, Deserialize};

use crate::schema::roles;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub role_id: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub role_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole<'a> {
    pub id: &'a str,
    pub role_name: &'a str,
}

#[derive(Deserialize)]
pub struct UserInputRole {
    pub role_name: String,
}
