pub mod login;
pub mod register;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // web::resource represents a specific HTTP endpoint
    // .route() takes a Route instance and adds it to the Resource on which it is called
    // web::post() creates and returns a Route instance
    // .to() registers a  handler fn for a Route, mutating the Route instance
    cfg.service(web::resource("/register").route(web::post().to(register::register)));
    cfg.service(web::resource("/login").route(web::post().to(login::login)));
}
