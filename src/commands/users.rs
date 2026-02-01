use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::{models::NewUsers, repository::UsersRepository};

use super::{convert_to_role_codes, create_db_connection};

pub async fn create_user(username: String, password: String, roles: Vec<String>) {
    let mut connection = create_db_connection().await;

    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password");

    let roles = convert_to_role_codes(&roles);
    let user = UsersRepository::create_or_update(
        &mut connection,
        NewUsers {
            username,
            password: hashed_password.to_string(),
        },
        roles,
    )
    .await;
    if let Ok(user) = user {
        println!("User successfully created: {:?}", user);
    } else {
        eprintln!("Failed to create a user: {:?}", user)
    }
}

pub async fn list_users() {
    let mut connection = create_db_connection().await;

    let result = UsersRepository::list(&mut connection, 0, 1000).await;
    if let Ok(users) = result {
        println!("Users list below:");
        for u in users {
            println!("{u:?}");
        }
    } else {
        eprintln!("Failed to list users: {:?}", result);
    }
}

pub async fn find_user(name: String) {
    let mut connection = create_db_connection().await;

    let result = UsersRepository::find_one_by_name(&mut connection, name).await;
    if let Ok(user) = result {
        println!("Found user below: {user:?}");
    } else {
        eprintln!("Failed to find user: {:?}", result);
    }
}

pub async fn delete_user(id: i32) {
    let mut connection = create_db_connection().await;

    let result = UsersRepository::safely_delete(&mut connection, id).await;
    if let Ok(_) = result {
        println!("User with id {id} successfully deleted");
    } else {
        eprintln!("Failed to delete user with id: {id}");
    }
}
