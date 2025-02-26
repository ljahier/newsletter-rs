use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub assets: AssetsConfig,
    pub database: DatabaseConfig,
    pub email: EmailConfig,
    pub site: SiteConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub log_level: String,
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct AssetsConfig {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub sqlite: SqliteConfig,
}

#[derive(Debug, Deserialize)]
pub struct SqliteConfig {
    pub filename: String,
}

#[derive(Debug, Deserialize)]
pub struct EmailConfig {
    pub smtp: SmtpConfig,
    pub identity: IdentityConfig,
}

#[derive(Debug, Deserialize)]
pub struct SmtpConfig {
    pub server_host: String,
    pub server_port: u16,
    pub server_starttls: bool,
    pub auth_user: String,
    pub auth_password: String,
}

#[derive(Debug, Deserialize)]
pub struct IdentityConfig {
    pub from_name: String,
    pub from_email: String,
}

#[derive(Debug, Deserialize)]
pub struct SiteConfig {
    pub name: String,
    pub admin_emails: Vec<String>,
    pub site_url: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.server.log_level.trim().is_empty() {
            return Err("server.log_level is empty".into());
        }
        if self.server.host.trim().is_empty() {
            return Err("server.host is empty".into());
        }
        if self.assets.path.trim().is_empty() {
            return Err("assets.path is empty".into());
        }
        if self.database.sqlite.filename.trim().is_empty() {
            return Err("database.sqlite.filename is empty".into());
        }
        if self.email.smtp.server_host.trim().is_empty() {
            return Err("email.smtp.server_host is empty".into());
        }
        if self.email.smtp.auth_user.trim().is_empty() {
            return Err("email.smtp.auth_user is empty".into());
        }
        if self.email.smtp.auth_password.trim().is_empty() {
            return Err("email.smtp.auth_password is empty".into());
        }
        if self.email.identity.from_name.trim().is_empty() {
            return Err("email.identity.from_name is empty".into());
        }
        if self.email.identity.from_email.trim().is_empty() {
            return Err("email.identity.from_email is empty".into());
        }
        if self.site.name.trim().is_empty() {
            return Err("site.name is empty".into());
        }
        if self.site.admin_emails.is_empty() {
            return Err("site.admin_emails is empty".into());
        }
        if self.site.site_url.trim().is_empty() {
            return Err("site.site_url is empty".into());
        }
        Ok(())
    }
}
