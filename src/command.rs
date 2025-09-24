use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{Record, RecordAction};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Command<V> {
    Cron(Cron),
    Email(Email),
    Scraper(Scraper<V>),
    Manual(Manual<V>),
    Transform(Transform<V>),
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
pub struct Transform<T, J = serde_json::Value> {
    pub records: Vec<TransformRecord<T, J>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformRecord<T, J = serde_json::Value> {
    pub valid_from: String,
    pub valid_until: Option<String>,
    pub record_type: String,
    pub source_key: Option<String>,
    // Nested record data
    pub record: T,
    // Join data nested within each record
    pub joins: HashMap<String, Option<Vec<Box<TransformRecord<J>>>>>,
}

// Transform command and related types - matching the JS PDK structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transform<TJoins = serde_json::Value> {
    pub records: Vec<TransformRecord<TJoins>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransformRecord<TJoins = serde_json::Value> {
    pub id: String,
    pub valid_from: String,
    pub valid_until: Option<String>,
    pub record_type: String,
    pub instance_id: Option<String>,
    pub source_key: Option<String>,
    pub record: serde_json::Value,
    pub joins: Option<TJoins>,
}

// Legacy TransformInput for backward compatibility - will be deprecated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformInput<T> {
    pub record: Record<T>,
    pub action: RecordAction,
}
