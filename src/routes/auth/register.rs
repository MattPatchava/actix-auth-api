use actix_web::{error, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct RegisterPayload {
    email: String,
    plaintext_password: String,
    first_name: String,
    last_name: String,
}

pub async fn register(
    payload: web::Json<RegisterPayload>,
    db_pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    let email: &str = &payload.email;
    let plaintext_password: &str = &payload.plaintext_password;
    let first_name: &str = &payload.first_name;
    let last_name: &str = &payload.last_name;

    sqlx::query!(
        r#"
    INSERT INTO users (email, password_hash, first_name, last_name)
    VALUES ($1, $2, $3, $4)
    "#,
        email,
        // Change this after adding encryption logic
        plaintext_password,
        first_name,
        last_name
    )
    .execute(db_pool.get_ref())
    .await
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body("New user added"))
}
