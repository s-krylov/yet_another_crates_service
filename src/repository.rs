use crate::models::{
    Crates, NewCrates, NewRoles, NewRustaceans, NewUsers, NewUsersRoles, Roles, Rustaceans, Users,
    UsersRoles,
};
use crate::rest_routes::handle_redis_error;
use crate::schema;
use diesel::query_dsl::methods::{FilterDsl, FindDsl, LimitDsl, OffsetDsl};
use diesel::{
    BelongingToDsl, ExpressionMethods, JoinOnDsl, QueryDsl, QueryResult, SelectableHelper,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rocket::response::status::Custom;
use rocket::serde::json::Value;
use rocket_db_pools::deadpool_redis::redis::{AsyncCommands, RedisError};

pub struct RustaceansRepository;

impl RustaceansRepository {
    pub async fn find_ond(con: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustaceans> {
        FindDsl::find(schema::rustaceans::table, id)
            .get_result(con)
            .await
    }

    pub async fn list(
        con: &mut AsyncPgConnection,
        offset: i64,
        limit: i64,
    ) -> QueryResult<Vec<Rustaceans>> {
        LimitDsl::limit(OffsetDsl::offset(schema::rustaceans::table, offset), limit)
            .get_results(con)
            .await
    }

    pub async fn create(
        con: &mut AsyncPgConnection,
        rustacean: NewRustaceans,
    ) -> QueryResult<Rustaceans> {
        diesel::insert_into(schema::rustaceans::table)
            .values(rustacean)
            .get_result(con)
            .await
    }

    pub async fn update(
        con: &mut AsyncPgConnection,
        id: i32,
        rustacean: NewRustaceans,
    ) -> QueryResult<Rustaceans> {
        diesel::update(QueryDsl::find(schema::rustaceans::table, id))
            .set((
                schema::rustaceans::name.eq(rustacean.name),
                schema::rustaceans::email.eq(rustacean.email),
            ))
            .get_result(con)
            .await
    }

    pub async fn delete(con: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(QueryDsl::find(schema::rustaceans::table, id))
            .execute(con)
            .await
    }
}

pub struct CratesRepository;

impl CratesRepository {
    pub async fn find_ond(con: &mut AsyncPgConnection, id: i32) -> QueryResult<Crates> {
        FindDsl::find(schema::crates::table, id)
            .get_result(con)
            .await
    }

    pub async fn list(
        con: &mut AsyncPgConnection,
        offset: i64,
        limit: i64,
    ) -> QueryResult<Vec<Crates>> {
        LimitDsl::limit(OffsetDsl::offset(schema::crates::table, offset), limit)
            .get_results(con)
            .await
    }

    pub async fn create(con: &mut AsyncPgConnection, a_crate: NewCrates) -> QueryResult<Crates> {
        diesel::insert_into(schema::crates::table)
            .values(a_crate)
            .get_result(con)
            .await
    }

    pub async fn update(
        con: &mut AsyncPgConnection,
        id: i32,
        a_crate: NewCrates,
    ) -> QueryResult<Crates> {
        diesel::update(QueryDsl::find(schema::crates::table, id))
            .set((
                schema::crates::name.eq(a_crate.name),
                schema::crates::code.eq(a_crate.code),
                schema::crates::version.eq(a_crate.version),
                schema::crates::description.eq(a_crate.description),
            ))
            .get_result(con)
            .await
    }

    pub async fn delete(con: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(QueryDsl::find(schema::crates::table, id))
            .execute(con)
            .await
    }
}

pub struct UsersRepository;

impl UsersRepository {
    pub async fn create_or_update(
        con: &mut AsyncPgConnection,
        user: NewUsers,
        roles: Vec<String>,
    ) -> QueryResult<Users> {
        let user = Self::create(con, user).await?;
        let already_existing_user_roles =
            RolesRepository::find_user_roles_codes(con, &user, &roles[..])
                .await?
                .iter()
                .map(|e| e.code.clone())
                .collect::<Vec<_>>();
        let mut new_user_roles_to_add = roles.clone();
        new_user_roles_to_add.retain(|e| !already_existing_user_roles.contains(e));

        let mut existing_roles =
            RolesRepository::find_roles_codes(con, &new_user_roles_to_add).await?;
        // existing roles but missing connection to user - adding it
        let existing_roles_codes = existing_roles
            .iter()
            .map(|e| e.code.clone())
            .collect::<Vec<_>>();
        let mut new_roles_codes_to_add = new_user_roles_to_add.clone();
        new_roles_codes_to_add.retain(|e| !existing_roles_codes.contains(e));

        let new_new_roles = new_roles_codes_to_add
            .iter()
            .map(|role_code| NewRoles {
                code: role_code.clone(),
                name: format!("generic name for {}", role_code),
            })
            .collect::<Vec<NewRoles>>();

        let mut new_roles = RolesRepository::create_list(con, new_new_roles).await?;

        let mut user_roles_to_add = Vec::<_>::new();
        user_roles_to_add.append(&mut existing_roles);
        user_roles_to_add.append(&mut new_roles);

        let user_roles_to_add = user_roles_to_add
            .iter()
            .map(|e| NewUsersRoles {
                users_id: user.id,
                roles_id: e.id,
            })
            .collect::<Vec<_>>();

        let _ = UserRolesRepository::create_list(con, user_roles_to_add).await?;

        Ok(user)
    }

    pub async fn add_user_roles(
        con: &mut AsyncPgConnection,
        name: String,
        roles: Vec<String>,
    ) -> QueryResult<Vec<Roles>> {
        let user = Self::find_one_by_name(con, name).await?;

        let already_existing_user_roles =
            RolesRepository::find_user_roles_codes(con, &user, &roles[..])
                .await?
                .iter()
                .map(|e| e.code.clone())
                .collect::<Vec<_>>();
        let mut new_user_roles_to_add = roles.clone();
        new_user_roles_to_add.retain(|e| !already_existing_user_roles.contains(e));

        let mut existing_roles =
            RolesRepository::find_roles_codes(con, &new_user_roles_to_add).await?;
        // existing roles but missing connection to user - adding it
        let existing_roles_codes = existing_roles
            .iter()
            .map(|e| e.code.clone())
            .collect::<Vec<_>>();
        let mut new_roles_codes_to_add = new_user_roles_to_add.clone();
        new_roles_codes_to_add.retain(|e| !existing_roles_codes.contains(e));

        let new_new_roles = new_roles_codes_to_add
            .iter()
            .map(|role_code| NewRoles {
                code: role_code.clone(),
                name: format!("generic name for {}", role_code),
            })
            .collect::<Vec<NewRoles>>();

        let mut new_roles = RolesRepository::create_list(con, new_new_roles).await?;

        let mut user_roles_to_add = Vec::<_>::new();
        user_roles_to_add.append(&mut existing_roles);
        user_roles_to_add.append(&mut new_roles);

        let a_new_user_roles_to_add = user_roles_to_add
            .iter()
            .map(|e| NewUsersRoles {
                users_id: user.id,
                roles_id: e.id,
            })
            .collect::<Vec<_>>();

        let _ = UserRolesRepository::create_list(con, a_new_user_roles_to_add).await?;

        Ok(user_roles_to_add)
    }

    pub async fn safely_delete(con: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        let _ = UserRolesRepository::delete_by_user_id(con, id).await?;

        Self::delete(con, id).await?;
        RolesRepository::delete_dangling(con).await
    }

    pub async fn find_one(con: &mut AsyncPgConnection, id: i32) -> QueryResult<Users> {
        FindDsl::find(schema::users::table, id)
            .get_result(con)
            .await
    }

    pub async fn find_one_by_name(con: &mut AsyncPgConnection, name: String) -> QueryResult<Users> {
        FilterDsl::filter(schema::users::table, schema::users::username.eq(name))
            .get_result(con)
            .await
    }

    pub async fn list(
        con: &mut AsyncPgConnection,
        offset: i64,
        limit: i64,
    ) -> QueryResult<Vec<Users>> {
        LimitDsl::limit(OffsetDsl::offset(schema::users::table, offset), limit)
            .get_results(con)
            .await
    }

    pub async fn create(con: &mut AsyncPgConnection, user: NewUsers) -> QueryResult<Users> {
        diesel::insert_into(schema::users::table)
            .values(user)
            .get_result(con)
            .await
    }

    pub async fn update(
        con: &mut AsyncPgConnection,
        id: i32,
        user: NewUsers,
    ) -> QueryResult<Users> {
        diesel::update(QueryDsl::find(schema::users::table, id))
            .set((
                schema::users::username.eq(user.username),
                schema::users::password.eq(user.password),
            ))
            .get_result(con)
            .await
    }

    pub async fn delete(con: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(schema::users::table)
            .filter(schema::users::id.eq(id))
            .execute(con)
            .await
    }
}

pub struct RolesRepository;

impl RolesRepository {
    pub async fn find_one(con: &mut AsyncPgConnection, id: i32) -> QueryResult<Roles> {
        FindDsl::find(schema::roles::table, id)
            .get_result(con)
            .await
    }

    pub async fn list(
        con: &mut AsyncPgConnection,
        offset: i64,
        limit: i64,
    ) -> QueryResult<Vec<Roles>> {
        LimitDsl::limit(OffsetDsl::offset(schema::roles::table, offset), limit)
            .get_results(con)
            .await
    }

    pub async fn list_user_roles(
        con: &mut AsyncPgConnection,
        name: String,
    ) -> QueryResult<Vec<Roles>> {
        FilterDsl::filter(
            schema::roles::table
                .inner_join(schema::users_roles::table)
                .inner_join(
                    schema::users::table.on(schema::users::id.eq(schema::users_roles::users_id)),
                ),
            schema::users::username.eq(name),
        )
        .select(schema::roles::all_columns)
        .load::<Roles>(con)
        .await
    }

    pub async fn create(con: &mut AsyncPgConnection, role: NewRoles) -> QueryResult<Roles> {
        diesel::insert_into(schema::roles::table)
            .values(role)
            .get_result(con)
            .await
    }

    pub async fn create_list(
        con: &mut AsyncPgConnection,
        roles: Vec<NewRoles>,
    ) -> QueryResult<Vec<Roles>> {
        diesel::insert_into(schema::roles::table)
            .values(roles)
            .get_results(con)
            .await
    }

    pub async fn update(
        con: &mut AsyncPgConnection,
        id: i32,
        role: NewRoles,
    ) -> QueryResult<Roles> {
        diesel::update(QueryDsl::find(schema::roles::table, id))
            .set((
                schema::roles::name.eq(role.name),
                schema::roles::code.eq(role.code),
            ))
            .get_result(con)
            .await
    }

    pub async fn delete(con: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(QueryDsl::find(schema::roles::table, id))
            .execute(con)
            .await
    }

    pub async fn delete_dangling(con: &mut AsyncPgConnection) -> QueryResult<usize> {
        diesel::delete(schema::roles::table)
            .filter(
                schema::roles::id.ne_all(
                    schema::users_roles::table
                        .select(schema::users_roles::roles_id)
                        .into_boxed(),
                ),
            )
            .execute(con)
            .await
    }

    pub async fn find_user_roles_codes(
        con: &mut AsyncPgConnection,
        user: &Users,
        roles: &[String],
    ) -> QueryResult<Vec<Roles>> {
        FilterDsl::filter(
            UsersRoles::belonging_to(user).inner_join(schema::roles::table),
            schema::roles::code.eq_any(roles),
        )
        .select(Roles::as_select())
        .get_results(con)
        .await
    }

    pub async fn find_roles_codes(
        con: &mut AsyncPgConnection,
        roles: &[String],
    ) -> QueryResult<Vec<Roles>> {
        FilterDsl::filter(schema::roles::table, schema::roles::code.eq_any(roles))
            .get_results(con)
            .await
    }
}

pub struct UserRolesRepository;

impl UserRolesRepository {
    pub async fn find_one(con: &mut AsyncPgConnection, id: i32) -> QueryResult<UsersRoles> {
        FindDsl::find(schema::users_roles::table, id)
            .get_result(con)
            .await
    }

    pub async fn list(
        con: &mut AsyncPgConnection,
        offset: i64,
        limit: i64,
    ) -> QueryResult<Vec<UsersRoles>> {
        LimitDsl::limit(OffsetDsl::offset(schema::users_roles::table, offset), limit)
            .get_results(con)
            .await
    }

    pub async fn create(
        con: &mut AsyncPgConnection,
        user_role: NewUsersRoles,
    ) -> QueryResult<UsersRoles> {
        diesel::insert_into(schema::users_roles::table)
            .values(user_role)
            .get_result(con)
            .await
    }

    pub async fn create_list(
        con: &mut AsyncPgConnection,
        user_roles: Vec<NewUsersRoles>,
    ) -> QueryResult<Vec<UsersRoles>> {
        diesel::insert_into(schema::users_roles::table)
            .values(user_roles)
            .get_results(con)
            .await
    }

    pub async fn delete(con: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(schema::users_roles::table)
            .filter(schema::users_roles::id.eq(id))
            .execute(con)
            .await
    }

    pub async fn delete_user_role(
        con: &mut AsyncPgConnection,
        username: String,
        role: String,
    ) -> QueryResult<usize> {
        diesel::delete(schema::users_roles::table)
            .filter(schema::users_roles::users_id.eq_any(FilterDsl::filter(
                schema::users::table.select(schema::users::id),
                schema::users::username.eq(username),
            )))
            .filter(schema::users_roles::roles_id.eq_any(FilterDsl::filter(
                schema::roles::table.select(schema::roles::id),
                schema::roles::code.eq(role),
            )))
            .execute(con)
            .await?;

        RolesRepository::delete_dangling(con).await
    }

    pub async fn delete_by_user_id(
        con: &mut AsyncPgConnection,
        user_id: i32,
    ) -> QueryResult<usize> {
        diesel::delete(schema::users_roles::table)
            .filter(schema::users_roles::users_id.eq(user_id))
            .execute(con)
            .await
    }
}

pub struct SessionRepository;

impl SessionRepository {
    const EXIPRATION_DURATION: usize = 2 * 3600;

    pub async fn save_session(
        con: &mut rocket_db_pools::deadpool_redis::Connection,
        session_id: &str,
        user: &Users,
    ) -> Result<(), Custom<Value>> {
        let path = format!("login/{session_id}");
        con.set_ex::<String, i32, ()>(path, user.id, Self::EXIPRATION_DURATION)
            .await
            .map_err(|error| handle_redis_error(error))
    }

    pub async fn get_user_id(con: &mut rocket_db_pools::deadpool_redis::Connection, session_id: &str) -> Result<i32, RedisError> {
        con.get::<'_, String,  i32>(format!("login/{session_id}")).await
    }
}
