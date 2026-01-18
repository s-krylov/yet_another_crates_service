use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rocket::{
    response::status::Custom,
    serde::json::{Json, Value, serde_json::json},
};
use rocket_db_pools::Connection;
use serde::Deserialize;

use crate::{
    repository::UsersRepository,
    rest_routes::{DbConnection, handle_error},
};

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<DbConnection>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    let argon2 = Argon2::default();
    let credentials = credentials.into_inner();
    UsersRepository::find_one_by_name(&mut db, credentials.username.clone())
        .await
        .map(|user| {
            let hashed_password =
                PasswordHash::new(&user.password).expect("Failed to restore hashed password");
            let result = argon2.verify_password(credentials.password.as_bytes(), &hashed_password);
            if result.is_ok() {
                return json!("Success");
            } else {
                return json!("Error");
            }
        })
        .map_err(handle_error)
}
