use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
};

use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PgRange<T> {
    start: std::collections::Bound<T>,
    end: std::collections::Bound<T>,
}

pub type Envs = BTreeMap<String, String>;

fn serialize_range<S, T>(range: &Option<PgRange<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    if let Some(PgRange { start, end }) = range {
        Range { start, end }.serialize(serializer)
    } else {
        None::<Range<u128>>.serialize(serializer)
    }
}

fn deserialize_range<'de, D, T>(deserializer: D) -> Result<Option<PgRange<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    let range = Option::<Range<T>>::deserialize(deserializer)?;
    Ok(range.map(|Range { start, end }| PgRange {
        start: std::collections::Bound::Included(start),
        end: std::collections::Bound::Excluded(end),
    }))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum EntryStatus {
    AutoPosted,
    ManuallyPosted,
    InReview,
    Unposted,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entry<E: Send + Sync> {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub effective_at: DateTime<Utc>,
    pub status: EntryStatus,
    pub entry: Option<E>,
    pub entry_type: String,
    pub source_key: Option<String>,
    pub instance_id: Option<Uuid>,
    #[serde(
        serialize_with = "serialize_range",
        deserialize_with = "deserialize_range",
        flatten
    )]
    pub sys_period: Option<PgRange<DateTime<Utc>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Copy, Eq, PartialEq)]
pub enum LineType {
    Receivable,
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
    pub consumes_line_id: Option<Uuid>,
    pub entry_id: Option<Uuid>,
    pub resource_id: Uuid,
    #[serde(
        serialize_with = "serialize_range",
        deserialize_with = "deserialize_range",
        flatten
    )]
    pub sys_period: Option<PgRange<DateTime<Utc>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Instance {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub slug: String,
    pub name: Option<String>,
    pub plugin_id: Uuid,
    #[serde(
        serialize_with = "serialize_range",
        deserialize_with = "deserialize_range",
        flatten
    )]
    pub sys_period: Option<PgRange<DateTime<Utc>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Resource<Res: Send + Sync> {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: Option<String>,
    pub unit: String,
    pub source_key: Option<String>,
    pub instance_id: Option<Uuid>,
    pub resource: Option<Res>,
    pub resource_type: String,
    #[serde(
        serialize_with = "serialize_range",
        deserialize_with = "deserialize_range",
        flatten
    )]
    pub sys_period: Option<PgRange<DateTime<Utc>>>,
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
pub struct Tag<T: Send + Sync> {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub data_type: String,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub dimension_value_id: Option<Uuid>,
    pub tag: T,
    pub tag_type: String,
    #[serde(
        serialize_with = "serialize_range",
        deserialize_with = "deserialize_range",
        flatten
    )]
    pub sys_period: Option<PgRange<DateTime<Utc>>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineTag {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub line_id: Uuid,
    pub tag_id: Uuid,
    #[serde(
        serialize_with = "serialize_range",
        deserialize_with = "deserialize_range",
        flatten
    )]
    pub sys_period: Option<PgRange<DateTime<Utc>>>,
}
