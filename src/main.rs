mod dao;
mod dto;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use crate::dao::schema::users::dsl::*;
use crate::dao::models::*;
use crate::dto::UserDTO;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/users")]
async fn get_users() -> impl Responder {
    let connection = &mut dao::connect();
    let results: Vec<User> = users
        .limit(10)
        .load::<User>(connection)
        .expect("Error loading users");

    let dto = match results.get(0) {
        Some(user) => UserDTO::from_user(user),
        None => panic!("not users in vec")
    };

    dto
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_users)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
