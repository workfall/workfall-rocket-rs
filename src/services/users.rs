use diesel::prelude::*;
use elonaire_backend_rs::{
    models::models::{NewRole, NewUser, Role, User, UserInputUser},
    *,
};
use rocket::serde::json::{json, Value};
extern crate bcrypt;
use bcrypt::{hash, DEFAULT_COST};

pub fn get_users() -> Value {
    use elonaire_backend_rs::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let results: Vec<User> = users.load::<User>(connection).expect("Error loading posts");

    json!(results)
}

pub fn add_role(role_name: &str) -> Value {
    use elonaire_backend_rs::schema::roles;

    let id = uuid::Uuid::new_v4().to_string();

    let connection = &mut establish_connection();

    let new_role: NewRole = NewRole { id: &id, role_name };

    let created_role: Role = diesel::insert_into(roles::table)
        .values(&new_role)
        .get_result::<Role>(connection)
        .expect("Error saving new role");

    json!(created_role)
}

pub fn create_user(user_details: &UserInputUser) -> Value {
    use elonaire_backend_rs::schema::roles::dsl::*;
    use elonaire_backend_rs::schema::users;

    let connection = &mut establish_connection();

    let mut role: Vec<Role>;

    if user_details.role_id.is_none() {
        role = roles
            .filter(role_name.eq("USER"))
            .limit(1)
            .load::<Role>(connection)
            .expect("Error loading role");
    } else {
        role = roles
            .filter(id.eq(&user_details.role_id.clone().unwrap()))
            .limit(1)
            .load::<Role>(connection)
            .expect("Invalid role");
    }

    let user_id = uuid::Uuid::new_v4().to_string();
    let hashed = hash(&user_details.password, DEFAULT_COST).expect("Failed to encrypt");

    let new_user: NewUser = NewUser {
        id: &user_id,
        first_name: &user_details.first_name,
        middle_name: &user_details.middle_name,
        last_name: &user_details.last_name,
        email: &user_details.email,
        role_id: &mut role[0].id,
        password: &hashed,
    };

    let created_user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(connection)
        .expect("Error saving new role");

    json!(created_user)
}

pub fn update_user(user_details: &UserInputUser) -> Value {
    // use elonaire_backend_rs::schema::users;
    use elonaire_backend_rs::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let hashed = hash(&user_details.password, DEFAULT_COST).expect("Failed to encrypt");
    
    let updated_user: User = diesel::update(users.filter(email.eq(user_details.email.clone())))
    .set(password.eq(hashed))
    .get_result::<User>(connection)
    .expect("Error updating user");

    json!(updated_user)
}
