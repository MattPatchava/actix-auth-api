use anyhow::{anyhow, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
// The Claims struct represents the payload of a JWT
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

pub fn generate_jwt(sub: &str, long_lived: &bool) -> Result<String> {
    // The following implementation is separated into more atomic steps to allow me to add individual
    // comments to clearly explain how this process works to support my learning. Normally, I
    // would chain each of the methods rather than assigning variables.

    // SystemTime is an abstraction over the system clock, and wraps std::time_t in C
    let now: SystemTime = SystemTime::now();

    // .checked_add() safely adds a Duration to a SystemTime instance. It returns
    // Option<SystemTime> because the result could theoretically overflow if it exceeds
    // SystemTime::MAX.
    let expiry: SystemTime = match now.checked_add(Duration::from_secs(900)) {
        Some(t) => t,
        None => return Err(anyhow!("Could not compute expiration time")),
    };

    // The Duration type ensures type safety when adding SystemTime instances
    let duration: Duration = expiry.duration_since(UNIX_EPOCH)?;

    // .as_secs() converts a Duration to a u64 representing whole seconds, and dropping any
    // fractional part
    let exp_secs: u64 = duration.as_secs();

    let claims: Claims = Claims {
        sub: sub.to_owned(),
        exp: exp_secs,
    };

    let secret: String = std::env::var("JWT_SECRET").map_err(|e| anyhow!(e))?;

    encode(
        // A Header represents the JOSE header of a JWT. A JOSE header is the first part of a JWT
        // and contains metadata such as the algorithm used for signing and token type.
        // Header::default() initialises a JOSE with default values.
        &Header::default(),
        &claims,
        // An EncodingKey wrapper is used because rather than passing the bytes directly as this
        // provides a uniform interface for different key types (from_secret, from_ec_pem, etc.)
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| anyhow!(e))
}

// TokenData is a struct that holds your decoded claims (Claims instance), and a Header instance
pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>> {
    let secret: String = std::env::var("JWT_SECRET").map_err(|_| anyhow!("JWT_SECRET not set"))?;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&secret.as_bytes()),
        // A Validation struct defines the validation rules when decoding a JWT. The
        // Validation::default() function returns an instance that assumes HS256
        &Validation::default(),
    )
    .map_err(|e| anyhow!(e))
}
