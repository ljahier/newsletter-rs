use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(flatten)]
    pub sub: Session,
    pub exp: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub user_email: String,
}

impl Session {
    pub fn new(user_email: String, user_id: String) -> Self {
        Self {
            user_email,
            user_id,
        }
    }
}
