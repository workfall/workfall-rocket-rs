use diesel::{prelude::*};
use rocket::serde::{Serialize, Deserialize};

use crate::schema::{roles, users};

/*
* User models begin from here
*/

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

#[derive(Insertable, Serialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub first_name: &'a str,
    pub middle_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub role_id: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize)]
pub struct UserInputUser {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub email: String,
    pub role_id: Option<String>,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInputUpdateUser {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub role_id: Option<String>,
    pub password: Option<String>,
}

/*
* Role models (no pun intended) begin from here
*/

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
