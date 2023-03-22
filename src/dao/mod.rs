pub mod models;
pub mod schema;

use std::env;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

pub fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_RUL mut be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}