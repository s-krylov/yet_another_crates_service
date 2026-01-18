use diesel::result::Error::{self, NotFound};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{Value, serde_json::json};
use rocket_db_pools::Database;

pub mod crates;
pub mod rustaceans;
pub mod authentication;

#[derive(Database)]
#[database("postgres")]
pub struct DbConnection(rocket_db_pools::diesel::PgPool);

fn handle_error(error: Error) -> Custom<Value> {
    rocket::error!("{}", error);
    match error {
        NotFound => Custom(Status::NotFound, json!("Not Found")),
        _ => Custom(Status::InternalServerError, json!("Error")),
    }
}
