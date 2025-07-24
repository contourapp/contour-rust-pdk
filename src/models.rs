use std::collections::HashMap;

use chrono::{DateTime, FixedOffset, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum EntryStatus {
    AutoPosted,
    ManuallyPosted,
    InReview,
    Unposted,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entry<E> {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub effective_at: DateTime<FixedOffset>,
    pub status: EntryStatus,
    pub entry: Option<E>,
    pub entry_type: String,
    pub source_key: Option<String>,
    pub instance_id: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Copy, Eq, PartialEq)]
pub enum LineType {
    Asset,
    Liability,
    Equity,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Line {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub line_type: LineType,
    pub debit: Decimal,
    pub credit: Decimal,
    pub ratio: Decimal,
    pub description: Option<String>,
    pub entry_id: Option<Uuid>,
    pub resource_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Instance {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub slug: String,
    pub name: Option<String>,
    pub plugin_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Resource<R> {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: Option<String>,
    pub unit: String,
    pub source_key: Option<String>,
    pub instance_id: Option<Uuid>,
    pub resource: Option<R>,
    pub resource_type: String,
    pub data_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RequestMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request<B, M, Re> {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub method: RequestMethod,
    pub endpoint: String,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub body: B,
    pub metadata: M,
    pub metadata_type: String,
    pub response: Re,
    pub response_type: String,
    pub status_code: i32,
    pub instance_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag<T> {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub data_type: Option<String>,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub tag: T,
    pub tag_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RecordAction {
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record<R> {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub valid_from: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
    pub org_id: Uuid,
    pub record: R,
    pub record_type: Option<String>,
    pub instance_id: Option<Uuid>,
    pub source_key: Option<String>,
    pub job_id: Option<Uuid>,
}
