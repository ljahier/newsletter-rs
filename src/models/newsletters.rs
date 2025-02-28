use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct NewsletterRaw {
    pub id: String,
    pub name: String,
    pub send_date: Option<DateTime<Utc>>,
    pub status: String,
    pub content_html: Option<String>,
    pub content_plain: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub sent_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub contact_lists: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct NewsletterWithLists {
    pub id: String,
    pub name: String,
    pub send_date: Option<DateTime<Utc>>,
    pub status: String,
    pub content_html: Option<String>,
    pub content_plain: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub sent_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub contact_lists: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct NewsletterRequest {
    pub name: String,
    pub send_date: Option<String>,
    pub content_type: String,
    pub content: String,
    pub action: String,
    pub contact_list_ids: Option<Vec<String>>,
}
