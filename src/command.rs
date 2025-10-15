use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Command<V> {
    Cron(Cron),
    Email(Email),
    Scraper(Scraper<V>),
    Manual(Manual<V>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cron {
    pub from: DateTime<Utc>,
    pub until: DateTime<Utc>,
    pub first: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manual<C> {
    pub command: C,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attachment {
    pub content_type: String,
    // base64 encoded
    pub data: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Email {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scraper<C> {
    pub items: Vec<C>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform<T, J> {
    pub records: Vec<TransformRecord<T, J>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformRecord<T, J> {
    pub source_key: String,
    pub record_type: String,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    // Nested record data
    pub record: T,
    // Join data nested within each record
    pub joins: J,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyJoins {}
