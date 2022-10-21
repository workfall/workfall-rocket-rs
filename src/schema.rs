// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}
