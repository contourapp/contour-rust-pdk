use std::{collections::HashMap, fmt::Debug, str::FromStr};

use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{Record, RecordAction};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Effective {
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
    DateTimeTz(DateTimeTz),
    DateTimeRange(DateTimeRange),
    DateTimeMultiRange(Vec<DateTimeRange>),
}

// Should move to Jiff at some point - https://github.com/BurntSushi/jiff/blob/HEAD/COMPARE.md
#[derive(Debug, Clone, PartialEq)]
pub struct DateTimeTz(pub DateTime<Tz>);

impl Serialize for DateTimeTz {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}[{}]", self.0.to_rfc3339(), self.0.timezone()))
    }
}

impl<'de> Deserialize<'de> for DateTimeTz {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut split = s.split('[');
        let d = split.next().ok_or(serde::de::Error::custom(
            "Could not get datetime from DateTimeTz",
        ))?;
        let datetime = DateTime::parse_from_rfc3339(d).map_err(serde::de::Error::custom)?;
        let tz = Tz::from_str(
            &split
                .next()
                .ok_or(serde::de::Error::custom("Could not get tz from DateTimeTz"))?
                .replace(']', ""),
        )
        .map_err(serde::de::Error::custom)?;
        Ok(DateTimeTz(datetime.with_timezone(&tz)))
    }
}

impl FromStr for DateTimeTz {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('[');
        let d = split
            .next()
            .ok_or(anyhow::anyhow!("Could not get datetime from DateTimeTz"))?;
        let datetime = DateTime::parse_from_rfc3339(d).map_err(|e| anyhow::anyhow!(e))?;
        let tz = Tz::from_str(
            &split
                .next()
                .ok_or(anyhow::anyhow!("Could not get tz from DateTimeTz"))?
                .replace(']', ""),
        )?;
        Ok(DateTimeTz(datetime.with_timezone(&tz)))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DateTimeRange {
    pub from: DateTimeTz,
    pub until: DateTimeTz,
}

impl DateTimeRange {
    pub fn new(from: DateTimeTz, until: DateTimeTz) -> Self {
        Self { from, until }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResourceSelector {
    Id(Uuid),
    SourceKey {
        resource_type: String,
        source_key: String,
    },
    SelectOrCreate {
        resource_type: String,
        source_key: String,
        name: Option<String>,
        unit: String,
        data_type: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TagSelector {
    Id(Uuid),
    SourceKey {
        tag_type: String,
        source_key: String,
    },
    SelectOrCreate {
        tag_type: String,
        source_key: String,
        name: Option<String>,
        data_type: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntryInput<E> {
    pub effective: Effective,
    pub source_key: String,
    pub entry: E,
    pub entry_type: String,
    pub lines: Vec<LineInput>,
}

impl<E> EntryInput<E> {
    pub fn new(effective: Effective, source_key: String, entry: E, lines: Vec<LineInput>) -> Self {
        let entry_type = get_type(&entry);

        Self {
            effective,
            source_key,
            entry,
            entry_type,
            lines,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineInput {
    pub resource: ResourceSelector,
    pub debit: Decimal,
    pub credit: Decimal,
    pub ratio: Decimal,
    pub description: Option<String>,
    pub tags: Vec<TagSelector>,
    pub parent_line_id: Option<Uuid>,
}

#[allow(clippy::too_many_arguments)]
impl LineInput {
    pub fn new(
        resource: ResourceSelector,
        debit: Decimal,
        credit: Decimal,
        ratio: Decimal,
        description: Option<String>,
        tags: Vec<TagSelector>,
    ) -> Self {
        Self {
            resource,
            debit,
            credit,
            ratio,
            description,
            tags,
            parent_line_id: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestInput<B> {
    pub domain: String,
    pub endpoint: String,
    pub method: String,
    pub parameters: Vec<(String, String)>,
    pub headers: HashMap<String, String>,
    pub body: Option<B>,
    pub response_type: String,
}

pub struct RequestBuilder<B> {
    domain: String,
    endpoint: String,
    method: String,
    parameters: Vec<(String, String)>,
    headers: HashMap<String, String>,
    body: Option<B>,
    response_type: String,
}

impl<B> RequestBuilder<B> {
    pub fn new(domain: String, path: String, method: String, response_type: String) -> Self {
        Self {
            domain,
            endpoint: path,
            method,
            parameters: Vec::new(),
            headers: HashMap::default(),
            body: None,
            response_type,
        }
    }

    pub fn add_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn add_parameters(mut self, query_parameters: Vec<(String, String)>) -> Self {
        self.parameters = query_parameters;
        self
    }

    pub fn add_body(mut self, body: B) -> Result<Self> {
        self.body = Some(body);
        Ok(self)
    }

    pub fn build(self) -> RequestInput<B> {
        RequestInput {
            domain: self.domain,
            endpoint: self.endpoint,
            method: self.method,
            parameters: self.parameters,
            body: self.body,
            headers: self.headers,
            response_type: self.response_type,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceInput<R> {
    pub name: Option<String>,
    pub resource_type: String,
    pub source_key: String,
    pub unit: String,
    pub resource: Option<R>,
    pub data_type: Option<String>,
}

impl<R> ResourceInput<R> {
    pub fn new(
        name: Option<String>,
        resource_type: String,
        source_key: String,
        unit: String,
        resource: Option<R>,
        data_type: Option<String>,
    ) -> Self {
        Self {
            name,
            resource_type,
            source_key,
            unit,
            resource,
            data_type,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagInput<T> {
    pub tag_type: String,
    pub source_key: String,
    pub name: Option<String>,
    pub tag: Option<T>,
    pub data_type: Option<String>,
}

impl<T> TagInput<T> {
    pub fn new(
        tag_type: String,
        source_key: String,
        name: Option<String>,
        tag: Option<T>,
        data_type: Option<String>,
    ) -> Self {
        Self {
            tag_type,
            source_key,
            name,
            tag,
            data_type,
        }
    }
}

pub fn get_type<T>(val: &T) -> String {
    std::any::type_name_of_val(&val)
        .split("::")
        .last()
        .unwrap()
        .replace('>', "")
        .to_string()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HandlerInput<C> {
    pub command_type: String,
    pub command: C,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimezoneInput {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordInput<R> {
    pub source_key: String,
    pub record_type: String,
    pub record: R,
    pub valid_from: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
}

impl<R> RecordInput<R> {
    pub fn new(
        source_key: String,
        record_type: String,
        record: R,
        valid_from: DateTime<Utc>,
        valid_until: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            source_key,
            record_type,
            record,
            valid_from,
            valid_until,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordsInput<R> {
    pub records: Vec<RecordInput<R>>,
    pub plugin_controlled_history: bool,
}

impl<R> RecordsInput<R> {
    pub fn new(records: Vec<RecordInput<R>>, plugin_controlled_history: bool) -> Self {
        Self {
            records,
            plugin_controlled_history,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformInput<T> {
    pub record: TransformRecordWrapper<T>,
    pub action: RecordAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformRecordWrapper<T> {
    pub record: Record<T>,
    pub joins: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct TestStruct {
        pub name: String,
    }

    #[test]
    fn test_get_type() {
        let test_struct = TestStruct {
            name: "test".to_string(),
        };

        assert_eq!(get_type(&test_struct), "TestStruct");
    }

    #[test]
    fn test_get_type_nested() {
        let test_struct = TestStruct {
            name: "test".to_string(),
        };

        let input_resource = ResourceInput::new(
            Some("test".to_string()),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            Some(test_struct),
            Some("TestStruct".to_string()),
        );

        assert_eq!(get_type(&input_resource), "TestStruct");
    }
}
