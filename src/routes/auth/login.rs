use actix_web::{error, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::services::{hashing, jwt};

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
    keep_logged_in: bool,
}

pub async fn login(
    payload: web::Json<LoginPayload>,
    db_pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    let email: &str = &payload.email;
    let password: &str = &payload.password;
    let keep_logged_in: &bool = &payload.keep_logged_in;

    let result = sqlx::query!(
        r#"
        SELECT password_hash FROM users WHERE email = $1
        "#,
        email
    )
    .fetch_optional(db_pool.get_ref())
    .await
    .map_err(|e| error::ErrorInternalServerError(e))?;

    let password_hash: String = match result {
        Some(s) => s.password_hash,
        None => return Err(actix_web::error::ErrorNotFound("No matching user found")),
    };

    match hashing::verify_password(&password, &password_hash) {
        Ok(is_match) => {
            if is_match {
                // Issue JWT here
                let jwt: String = jwt::generate_jwt(&email, keep_logged_in)
                    .map_err(|e| error::ErrorInternalServerError(e))?;

                return Ok(HttpResponse::Ok().body(jwt));
            } else {
                return Err(error::ErrorUnauthorized("Invalid credentials"));
            }
        }
        Err(_) => {
            return Err(error::ErrorInternalServerError(
                "Error validating credentials",
            ))
        }
    }
}
