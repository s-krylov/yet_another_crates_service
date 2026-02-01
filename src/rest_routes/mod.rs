use diesel::result::Error::{self, NotFound};
use rocket::Request;
use rocket::http::Status;

use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::response::status::Custom;
use rocket::serde::json::{Value, serde_json::json};
use rocket_db_pools::Connection;
use rocket_db_pools::Database;
use rocket_db_pools::deadpool_redis::redis::RedisError;
use std::error::Error as StdError;

use crate::models::EditorUser;
use crate::models::RoleCodes;
use crate::models::Users;
use crate::repository::RolesRepository;
use crate::repository::SessionRepository;
use crate::repository::UsersRepository;

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

fn reply_with_unauthorized<T: StdError>(error: T) -> Custom<Value> {
    rocket::error!("{}", error);
    Custom(Status::Unauthorized, json!("Error"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Users {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = request.headers();
        let auth = headers
            .get_one("Authorization")
            .and_then(|h| h.strip_prefix("Bearer "));
        let Some(session_id) = auth else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        let cache_outcome = request.guard::<Connection<CacheConnection>>().await;
        let Outcome::Success(mut cache) = cache_outcome else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        let user_id = SessionRepository::get_user_id(&mut cache, session_id).await;
        let Ok(user_id) = user_id else {
            return Outcome::Error((Status::Unauthorized, ()));
        };

        let db_outcome = request.guard::<Connection<DbConnection>>().await;
        let Outcome::Success(mut db) = db_outcome else {
            return Outcome::Error((Status::Unauthorized, ()));
        };

        let user = UsersRepository::find_one(&mut db, user_id).await;
        return match user {
            Ok(user) => Outcome::Success(user),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        };
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_outcome = request.guard::<Users>().await;
        let Outcome::Success(user) = user_outcome else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        rocket::info!("user = {user:?}");
        let db_outcome = request.guard::<Connection<DbConnection>>().await;
        let Outcome::Success(mut db) = db_outcome else {
            return Outcome::Error((Status::Unauthorized, ()));
        };

        let roles = RolesRepository::list_user_roles(&mut db, user.username.clone()).await;
        let Ok(roles) = roles else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        rocket::info!("roles = {roles:?}");
        if roles
            .iter()
            .any(|r| r.code == RoleCodes::Admin || r.code == RoleCodes::Editor)
        {
            Outcome::Success(EditorUser(user))
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}
