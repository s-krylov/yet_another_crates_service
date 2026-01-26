use argon2::password_hash;
use diesel::result::Error::{self, NotFound};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{Value, serde_json::json};
use rocket_db_pools::Database;
use rocket_db_pools::deadpool_redis::redis::RedisError;

pub mod authentication;
pub mod crates;
pub mod rustaceans;

#[derive(Database)]
#[database("postgres")]
pub struct DbConnection(rocket_db_pools::diesel::PgPool);

#[derive(Database)]
#[database("redis")]
pub struct CacheConnection(rocket_db_pools::deadpool_redis::Pool);

fn handle_error(error: Error) -> Custom<Value> {
    rocket::error!("{}", error);
    match error {
        NotFound => Custom(Status::NotFound, json!("Not Found")),
        _ => Custom(Status::InternalServerError, json!("Error")),
    }
}

pub fn handle_redis_error(error: RedisError) -> Custom<Value> {
    rocket::error!("{}", error);
    Custom(Status::InternalServerError, json!("Error"))
}

fn handle_password_error(error: password_hash::Error) -> Custom<Value> {
    rocket::error!("{}", error);
    Custom(Status::Unauthorized, json!("Error"))
}
