use actix_web::{HttpResponse, Responder};

pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login route is reachable")
}
