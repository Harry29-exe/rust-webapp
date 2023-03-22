// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        surname -> Varchar,
        password_hash -> Varchar,
    }
}
