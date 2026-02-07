use std::env;

use chrono::{Datelike, Utc};
use tera::Context;

use crate::{
    commands::{create_db_connection, create_template_engine},
    mail::MailSender,
    repository::CratesRepository,
    schema::users::username,
};

pub async fn send_digest(email: String, hours_since: i32) {
    let mut conn = create_db_connection().await;
    let Ok(crates) = CratesRepository::list_since(&mut conn, hours_since).await else {
        eprintln!("Can't fetch latest crates");
        return;
    };
    if crates.is_empty() {
        eprintln!("No crates to inform about");
        return;
    }
    let mut context = Context::new();
    context.insert("crates", &crates);
    context.insert("year", &Utc::now().year());
    let template_engine = create_template_engine();
    let text = template_engine.render("email/digest.html", &context);
    let Ok(text) = text else {
        eprintln!("Can't create html page from template");
        return;
    };

    let user_name = env::var("SMTP_USERNAME");
    let Ok(user_name) = user_name else {
        eprintln!("Can't find SMTP_USERNAME environment variable");
        return;
    };
    let password = env::var("SMTP_PASSWORD");
    let Ok(password) = password else {
        eprintln!("Can't find SMTP_PASSWORD environment variable");
        return;
    };
    let hostname = env::var("SMTP_HOST");
    let Ok(hostname) = hostname else {
        eprintln!("Can't find SMTP_HOST environment variable");
        return;
    };
    let mail_sender = MailSender::new(user_name, password, hostname);
    mail_sender.send(email, "Latest crates digest".to_string(), text);
}
