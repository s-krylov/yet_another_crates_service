use argon2::password_hash::Error;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rand::Rng;
use rand::distr::Alphanumeric;
use serde::Deserialize;

use crate::models::Users;

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn authenticate_user(user: &Users, credentials: Credentials) -> Result<String, Error> {
    let argon2 = Argon2::default();
    let hashed_password = PasswordHash::new(&user.password)?;
    let _ = argon2.verify_password(credentials.password.as_bytes(), &hashed_password)?;

    let session_id = rand::rng()
        .sample_iter(Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    Ok(session_id)
}
