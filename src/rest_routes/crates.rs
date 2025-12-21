use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{Json, Value, serde_json::json};
use rocket_db_pools::Connection;

use super::DbConnection;

use crate::models::NewCrates;
use crate::repository::CratesRepository;

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConnection>) -> Result<Value, Custom<Value>> {
    CratesRepository::list(&mut db, 100)
        .await
        .map(|crates| json!(crates))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate(mut db: Connection<DbConnection>, id: i32) -> Result<Value, Custom<Value>> {
    CratesRepository::find_ond(&mut db, id)
        .await
        .map(|a_crate| json!(a_crate))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db: Connection<DbConnection>,
    new_crate: Json<NewCrates>,
) -> Result<Custom<Value>, Custom<Value>> {
    CratesRepository::create(&mut db, new_crate.into_inner())
        .await
        .map(|a_crate| Custom(Status::Created, json!(a_crate)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::put("/crates/<id>", format = "json", data = "<new_crate>")]
pub async fn update_crate(
    mut db: Connection<DbConnection>,
    id: i32,
    new_crate: Json<NewCrates>,
) -> Result<Value, Custom<Value>> {
    CratesRepository::update(&mut db, id, new_crate.into_inner())
        .await
        .map(|a_crate| json!(a_crate))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(
    mut db: Connection<DbConnection>,
    id: i32,
) -> Result<NoContent, Custom<Value>> {
    CratesRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}
