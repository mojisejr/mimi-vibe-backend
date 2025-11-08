use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reading {
    pub id: i64,
    pub user_id: i64,
    pub question: String,
}
