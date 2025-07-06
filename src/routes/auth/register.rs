use actix_web::{HttpResponse, Responder};

pub async fn register() -> impl Responder {
    HttpResponse::Ok().body("Register route is reachable")
}
