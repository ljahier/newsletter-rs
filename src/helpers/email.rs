use std::sync::OnceLock;
use std::time::Duration;

use crate::config::config::EmailConfig;
use lettre::message::{Mailbox, header};
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use tracing::error;

static EMAIL_CONFIG: OnceLock<Email> = OnceLock::new();
const SMTP_TIMEOUT: Duration = Duration::from_secs(6);

#[derive(Debug, Clone)]
pub struct Email {
    mailer: SmtpTransport,
    from: Mailbox,
}

impl Email {
    pub fn init(config: &EmailConfig) {
        let helper = Self::new(&config);
        EMAIL_CONFIG
            .set(helper)
            .expect("EmailHelper déjà initialisé");
    }

    pub fn get() -> &'static Email {
        EMAIL_CONFIG.get().expect("EmailHelper non initialisé")
    }

    pub fn new(config: &EmailConfig) -> Self {
        let creds = Credentials::new(
            config.smtp.auth_user.clone(),
            config.smtp.auth_password.clone(),
        );

        let tls_parameters = TlsParameters::new(config.smtp.server_host.clone())
            .expect("Failed to initialize TLS parameters");
        let tls = if config.smtp.server_starttls {
            Tls::Required(tls_parameters)
        } else {
            Tls::Wrapper(tls_parameters)
        };

        let mailer = SmtpTransport::relay(&config.smtp.server_host)
            .expect("Échec de la configuration SMTP relay")
            .port(config.smtp.server_port)
            .credentials(creds)
            .tls(tls)
            .timeout(Some(SMTP_TIMEOUT))
            .build();

        let from = Mailbox::new(
            Some(config.identity.from_name.clone()),
            config
                .identity
                .from_email
                .parse()
                .expect("Invalid email address"),
        );

        Self { mailer, from }
    }

    pub fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), lettre::transport::smtp::Error> {
        let email = Message::builder()
            .from(self.from.clone())
            .to(to.parse().expect("Erreur parsing destinataire"))
            .subject(subject)
            .header(header::ContentType::TEXT_HTML)
            .body(body.to_string())
            .expect("Erreur création e-mail");

        if let Err(err) = self.mailer.send(&email) {
            error!("Erreur lors de l'envoi à {}: {:?}", to, err);
            return Err(err);
        }
        Ok(())
    }
}
