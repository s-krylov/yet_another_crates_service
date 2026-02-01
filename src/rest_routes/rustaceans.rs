use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{Json, Value, serde_json::json};
use rocket_db_pools::Connection;

use super::DbConnection;
use super::handle_error;

use crate::models::{EditorUser, NewRustaceans, Users};
use crate::repository::RustaceansRepository;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(
    mut db: Connection<DbConnection>,
    _user: Users,
) -> Result<Value, Custom<Value>> {
    RustaceansRepository::list(&mut db, 0, 100)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(handle_error)
}

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean(
    mut db: Connection<DbConnection>,
    id: i32,
    _user: Users,
) -> Result<Value, Custom<Value>> {
    RustaceansRepository::find_ond(&mut db, id)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(handle_error)
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    mut db: Connection<DbConnection>,
    new_rustacean: Json<NewRustaceans>,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceansRepository::create(&mut db, new_rustacean.into_inner())
        .await
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(handle_error)
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<new_rustacean>")]
pub async fn update_rustacean(
    mut db: Connection<DbConnection>,
    id: i32,
    new_rustacean: Json<NewRustaceans>,
    _user: EditorUser,
) -> Result<Value, Custom<Value>> {
    RustaceansRepository::update(&mut db, id, new_rustacean.into_inner())
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(handle_error)
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    mut db: Connection<DbConnection>,
    id: i32,
    _user: EditorUser,
) -> Result<NoContent, Custom<Value>> {
    RustaceansRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(handle_error)
}
