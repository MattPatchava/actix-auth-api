use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};

pub fn hash_password(plaintext: &str) -> Result<String> {
    let salt: SaltString = SaltString::generate(OsRng);

    let argon2: Argon2 = Argon2::default();

    let encoded_hash: String = match argon2.hash_password(plaintext.as_bytes(), &salt) {
        Ok(encoded_hash) => encoded_hash.to_string(),
        Err(e) => return Err(anyhow!(e)),
    };

    Ok(encoded_hash)
}

pub fn verify_password(password_input: &str, stored_hash: &str) -> Result<bool> {
    let parsed_hash: PasswordHash = PasswordHash::new(&stored_hash).map_err(|e| anyhow!(e))?;

    match Argon2::default().verify_password(password_input.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
