use super::create_db_connection;
use crate::repository::{RolesRepository, UserRolesRepository, UsersRepository};

pub async fn list_roles_for_user(name: String) {
    let mut connection = create_db_connection().await;

    let roles = RolesRepository::list_user_roles(&mut connection, name.clone()).await;

    if let Ok(roles) = roles {
        println!("User {name} have the following roles: {roles:?}");
    } else {
        eprintln!("Failed to find roles for the user {name}");
    }
}

pub async fn add_roles_for_user(name: String, roles: Vec<String>) {
    let mut connection = create_db_connection().await;

    let roles = UsersRepository::add_user_roles(&mut connection, name.clone(), roles).await;
    if let Ok(roles) = roles {
        println!("User {name} now have the following roles: {roles:?}");
    } else {
        eprintln!("Failed to find roles for the user {name}");
    }
}

pub async fn delete_user_role(username: String, role: String) {
    let mut connection = create_db_connection().await;

    let result = UserRolesRepository::delete_user_role(&mut connection, username.clone(), role.clone()).await;
    if let Ok(_) = result {
        println!("User {username} role {role} was successfully deleted");
    } else {
        eprintln!("Failed to delete role {role} for user {username}");
    }
}