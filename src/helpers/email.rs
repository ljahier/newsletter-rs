use std::error::Error;

use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, header::ContentType},
};

use crate::APP_CONFIG;

#[tracing::instrument]
pub fn make_smtp_mailbox() -> Mailbox {
    let config = &APP_CONFIG
        .get()
        .expect("Configuration not initialized")
        .email
        .identity;
    Mailbox::new(
        Some(config.from_name.to_owned()),
        config.from_email.parse().expect("Adresse email invalide"),
    )
}

#[tracing::instrument]
pub fn send_email(
    mailer: &SmtpTransport,
    from: &Mailbox,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<(), Box<dyn Error>> {
    let email = Message::builder()
        .from(from.clone())
        .to(to.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body.to_owned())?;

    mailer.send(&email)?;
    Ok(())
}
