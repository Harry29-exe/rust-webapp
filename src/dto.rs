use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::Serialize;
use crate::dao::models::User;

#[derive(Serialize)]
pub struct UserDTO {
    pub name: String,
    pub surname: String
}

impl UserDTO {

    pub fn from_user(user: &User) -> UserDTO {
        return UserDTO{
            name: user.name.clone(),
            surname: user.surname.clone(),
        }
    }

}

impl Responder for UserDTO {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}