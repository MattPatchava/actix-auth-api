pub mod auth;
pub mod user;

use actix_web::web;

// web::ServiceConfig is an actix-web-specific config object that registers routes, services, and
// scope onto the main App. The App instance has an internal web::ServiceConfig which it passes to
// this function to be configured
pub fn configure(cfg: &mut web::ServiceConfig) {
    // web::scope() creates and returns a Scope instance. A Scope is a grouping of routes under a
    // shared prefix
    cfg.service(web::scope("/auth").configure(auth::configure));
}
