use lettre::{
    SmtpTransport, Transport,
    message::{MessageBuilder, header::ContentType},
    transport::smtp::authentication::Credentials,
};

pub struct MailSender {
    username: String,
    password: String,
    hostname: String,
}

impl MailSender {
    pub fn new(username: String, password: String, hostname: String) -> Self {
        Self {
            username,
            password,
            hostname,
        }
    }

    pub fn send(&self, email: String, subject: String, content: String) {
        let message = MessageBuilder::new()
            .subject(subject)
            .from(
                "cr8t_service <noname@mail.com>"
                    .parse()
                    .expect("Parse no reply mailbox failed"),
            )
            .to(email.parse().expect("Parse destination mailbox failed"))
            .header(ContentType::TEXT_HTML)
            .body(content)
            .expect("Failed to create mail");
        let credentials = Credentials::new(self.username.clone(), self.password.clone());
        let transport = SmtpTransport::relay(&self.hostname)
            .expect("Can't create smtp relay")
            .credentials(credentials)
            .build();

        dbg!(&transport);

        transport
            .send(&message)
            .expect("Email message was not send because of error");
    }
}
