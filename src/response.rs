use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractResponse {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}
