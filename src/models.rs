use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
};

use super::config::{ConfigDomainAuth, ConfigDomainRateLimit, ConfigListenerConfig};
use super::io::LineTypeInput;
use super::{command::Command as InterfaceCommand, config::ConfigPlugin};

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
pub enum CommandStatus {
    Pending,
    Running,
    Completed,
    Failed, // Delete this at some point
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Command<V> {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub command_type: String,
    pub instance_id: Option<Uuid>,
    pub command: InterfaceCommand<V>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CallResult {
    Ok,
    Err,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Call {
    pub id: Uuid,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub result: CallResult,
    pub command_id: Uuid,
    pub plugin_id: Uuid,
    pub ok: Option<serde_json::Value>,
    pub err: Option<serde_json::Value>,
    pub downstream_command_ids: Vec<Uuid>,
    pub log: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SideEffect {
    pub id: Uuid,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub function_name: String,
    pub input: Option<serde_json::Value>,
    pub output: Option<serde_json::Value>,
    pub call_id: Uuid,
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

impl From<&str> for &LineType {
    fn from(method: &str) -> Self {
        match method {
            "receivable" => &LineType::Receivable,
            "asset" => &LineType::Asset,
            "liability" => &LineType::Liability,
            "equity" => &LineType::Equity,
            _ => panic!("Invalid line type"),
        }
    }
}

impl From<&LineTypeInput> for &LineType {
    fn from(method: &LineTypeInput) -> Self {
        match method {
            LineTypeInput::Receivable => &LineType::Receivable,
            LineTypeInput::Asset => &LineType::Asset,
            LineTypeInput::Liability => &LineType::Liability,
            LineTypeInput::Equity => &LineType::Equity,
        }
    }
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
    pub agent_id: Option<Uuid>,
    #[serde(
        serialize_with = "serialize_range",
        deserialize_with = "deserialize_range",
        flatten
    )]
    pub sys_period: Option<PgRange<DateTime<Utc>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Plugin {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub version: String,
    pub hash: String,
    pub data: Vec<u8>,
    pub config: ConfigPlugin,
    pub schemas: serde_json::Value,
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
pub struct Listener {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub slug: String,
    pub instance_id: Uuid,
    pub listener_type: String,
    pub config: ConfigListenerConfig,
    #[serde(
        serialize_with = "serialize_range",
        deserialize_with = "deserialize_range",
        flatten
    )]
    pub sys_period: Option<PgRange<DateTime<Utc>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Domain {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub slug: String,
    pub instance_id: Uuid,
    pub environments: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub auth: ConfigDomainAuth,
    pub rate_limit: Option<ConfigDomainRateLimit>,
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
pub struct Agent<A> {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: Option<String>,
    pub source_key: Option<String>,
    pub instance_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub agent: Option<A>,
    pub agent_type: String,
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

impl From<&str> for RequestMethod {
    fn from(method: &str) -> Self {
        match method {
            "GET" => RequestMethod::Get,
            "POST" => RequestMethod::Post,
            "PATCH" => RequestMethod::Patch,
            "PUT" => RequestMethod::Put,
            "DELETE" => RequestMethod::Delete,
            _ => panic!("Invalid request method"),
        }
    }
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
pub struct Dimension {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub slug: String,
    pub name: Option<String>,
    pub instance_id: Option<Uuid>,
    #[serde(
        serialize_with = "serialize_range",
        deserialize_with = "deserialize_range",
        flatten
    )]
    pub sys_period: Option<PgRange<DateTime<Utc>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag<T: Send + Sync> {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub dimension_id: Uuid,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
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
