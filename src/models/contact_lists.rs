use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct ContactList {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "type")]
    pub list_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct NewContactListRequest {
    pub name: String,
    pub list_type: String,
}
