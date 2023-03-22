use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub surname: String,
    pub password_hash: String,
}
