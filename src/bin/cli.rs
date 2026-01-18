use clap::{Command, arg, command, value_parser};
use cr8t_service::commands::roles::{add_roles_for_user, delete_user_role, list_roles_for_user};
use cr8t_service::commands::users::{create_user, delete_user, find_user, list_users};

#[tokio::main]
async fn main() {
    let command_matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("users")
                .about("users section")
                .arg_required_else_help(true)
                .subcommands([
                    Command::new("create")
                        .about("create user")
                        .arg_required_else_help(true)
                        .args([
                            arg!(<username>).required(true),
                            arg!(<password>).required(true),
                            arg!(<roles>)
                                .required(true)
                                .num_args(1..)
                                .value_delimiter(','),
                        ]),
                    Command::new("list").about("list all user"),
                    Command::new("find")
                        .about("find user by name")
                        .arg_required_else_help(true)
                        .args([arg!(<username>).required(true)]),
                    Command::new("delete")
                        .about("delete user")
                        .arg_required_else_help(true)
                        .args([arg!(<id>).value_parser(value_parser!(i32)).required(true)]),
                ]),
            Command::new("roles")
                .about("working with roles")
                .arg_required_else_help(true)
                .subcommands([
                    Command::new("list")
                        .about("list user roles")
                        .arg_required_else_help(true)
                        .args([arg!(<username>).required(true)]),
                    Command::new("add")
                        .about("add user roles")
                        .arg_required_else_help(true)
                        .args([
                            arg!(<username>).required(true),
                            arg!(<roles>)
                                .required(true)
                                .num_args(1..)
                                .value_delimiter(','),
                        ]),
                    Command::new("delete")
                        .about("delete user role")
                        .arg_required_else_help(true)
                        .args([arg!(<username>).required(true), arg!(<role>).required(true)]),
                ]),
        ])
        .get_matches();

    match command_matches.subcommand() {
        Some(("users", sub_command)) => match sub_command.subcommand() {
            Some(("create", sub_matches)) => {
                let username = sub_matches
                    .get_one::<String>("username")
                    .expect("username parameter expected to be set")
                    .clone();
                let password = sub_matches
                    .get_one::<String>("password")
                    .expect("password parameter expected to be set")
                    .clone();

                let roles = sub_matches
                    .get_many::<String>("roles")
                    .expect("roles parameter expected to be set")
                    .cloned()
                    .collect::<Vec<_>>();
                create_user(username, password, roles).await
            }
            Some(("list", _)) => list_users().await,
            Some(("find", sub_matches)) => {
                let username = sub_matches
                    .get_one::<String>("username")
                    .expect("username parameter expected to be set")
                    .clone();
                find_user(username).await
            }
            Some(("delete", sub_matches)) => {
                let id = sub_matches
                    .get_one::<i32>("id")
                    .expect("user id should be presented");
                delete_user(*id).await
            }
            _ => unreachable!("Shold not happen"),
        },
        Some(("roles", sub_command)) => match sub_command.subcommand() {
            Some(("list", sub_matches)) => {
                let username = sub_matches
                    .get_one::<String>("username")
                    .expect("username parameter expected to be set")
                    .clone();
                list_roles_for_user(username).await;
            }
            Some(("add", sub_matches)) => {
                let username = sub_matches
                    .get_one::<String>("username")
                    .expect("username parameter expected to be set")
                    .clone();
                let roles = sub_matches
                    .get_many::<String>("roles")
                    .expect("roles parameter expected to be set")
                    .cloned()
                    .collect::<Vec<_>>();
                add_roles_for_user(username, roles).await
            }
            Some(("delete", sub_matches)) => {
                let username = sub_matches
                    .get_one::<String>("username")
                    .expect("username parameter expected to be set")
                    .clone();
                let role = sub_matches
                    .get_one::<String>("role")
                    .expect("role parameter expected to be set")
                    .clone();
                delete_user_role(username, role).await;
            }
            _ => unreachable!("Shold not happen"),
        },
        _ => unreachable!("Shold not happen"),
    }
}
