use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
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
