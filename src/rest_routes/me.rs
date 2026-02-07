use rocket::serde::json::{Value, serde_json::json};

use crate::models::Users;

#[rocket::get("/me")]
pub fn me(user: Users) -> Value {
    json!(user)
}
