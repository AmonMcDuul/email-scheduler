use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Message {
    pub id: Option<String>,
    pub email: String,
    pub message_body: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub send_at: Option<DateTime<Utc>>,
    pub send: Option<bool>,
}
