use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{Json, Value, serde_json::json};
use rocket_db_pools::Connection;

use super::DbConnection;
use super::handle_error;

use crate::models::{NewCrates, Users};
use crate::repository::CratesRepository;

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConnection>, _user: Users) -> Result<Value, Custom<Value>> {
    CratesRepository::list(&mut db, 0, 100)
        .await
        .map(|crates| json!(crates))
        .map_err(handle_error)
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate(mut db: Connection<DbConnection>, id: i32, _user: Users) -> Result<Value, Custom<Value>> {
    CratesRepository::find_ond(&mut db, id)
        .await
        .map(|a_crate| json!(a_crate))
        .map_err(handle_error)
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db: Connection<DbConnection>,
    new_crate: Json<NewCrates>,
    _user: Users,
) -> Result<Custom<Value>, Custom<Value>> {
    CratesRepository::create(&mut db, new_crate.into_inner())
        .await
        .map(|a_crate| Custom(Status::Created, json!(a_crate)))
        .map_err(handle_error)
}

#[rocket::put("/crates/<id>", format = "json", data = "<new_crate>")]
pub async fn update_crate(
    mut db: Connection<DbConnection>,
    id: i32,
    new_crate: Json<NewCrates>,
    _user: Users,
) -> Result<Value, Custom<Value>> {
    CratesRepository::update(&mut db, id, new_crate.into_inner())
        .await
        .map(|a_crate| json!(a_crate))
        .map_err(handle_error)
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(
    mut db: Connection<DbConnection>,
    id: i32,
    _user: Users,
) -> Result<NoContent, Custom<Value>> {
    CratesRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(handle_error)
}
