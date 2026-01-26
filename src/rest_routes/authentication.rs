use rocket::response::status::Custom;
use rocket::serde::json::{Json, Value, serde_json::json};
use rocket_db_pools::Connection;

use crate::auth::{Credentials, authenticate_user};
use crate::repository::{SessionRepository, UsersRepository};
use crate::rest_routes::{CacheConnection, DbConnection, handle_error, handle_password_error};

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<DbConnection>,
    mut cache: Connection<CacheConnection>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    let credentials = credentials.into_inner();
    let user = UsersRepository::find_one_by_name(&mut db, credentials.username.clone())
        .await
        .map_err(handle_error)?;
    let session_id =
        authenticate_user(&user, credentials).map_err(|error| handle_password_error(error))?;
    SessionRepository::save_session(&mut cache, &session_id, &user).await?;
    return Ok(json!({
        "session_id": session_id
    }));
}
