use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Contact {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub email: String,
    pub unsubscribe_token: Option<String>,
    pub custom_fields: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct ContactListWithMembers {
    pub id: String,
    pub name: String,
    pub list_type: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub members: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct NewContactRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub email: String,
    pub custom_fields: Option<String>,
}

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct ContactEmail {
    pub email: String,
}
