// @generated automatically by Diesel CLI.

diesel::table! {
    roles (id) {
        id -> Varchar,
        role_name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        email -> Varchar,
        role_id -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    users,
);
