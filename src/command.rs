use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Inserted<R> {
    pub id: Uuid,
    pub record: R,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Updated<R> {
    pub id: Uuid,
    pub record: R,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Deleted {
    pub id: Uuid,
}
