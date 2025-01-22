use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DateTimeRange {
    pub from: DateTime<Utc>,
    pub until: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effective {
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
    DateTimeFixed(DateTime<FixedOffset>),
    DateTimeRange(DateTimeRange),
    DateTimeMultiRange(Vec<DateTimeRange>),
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
    pub resource_id: Uuid,
    pub debit: Decimal,
    pub credit: Decimal,
    pub ratio: Decimal,
    pub description: Option<String>,
    pub tags: Vec<Uuid>,
}

#[allow(clippy::too_many_arguments)]
impl LineInput {
    pub fn new(
        resource_id: Uuid,
        debit: Decimal,
        credit: Decimal,
        ratio: Decimal,
        description: Option<String>,
        tags: Vec<Uuid>,
    ) -> Self {
        Self {
            resource_id,
            debit,
            credit,
            ratio,
            description,
            tags,
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
}

impl<T> TagInput<T> {
    pub fn new(tag_type: String, source_key: String, name: Option<String>, tag: Option<T>) -> Self {
        Self {
            tag_type,
            source_key,
            name,
            tag,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryEntry {
    pub source_key: String,
}

impl QueryEntry {
    pub fn new(source_key: String) -> Self {
        Self { source_key }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryLastEntry {
    pub entry_type: String,
    pub before: DateTime<Utc>,
}

impl QueryLastEntry {
    pub fn new(entry_type: String, before: DateTime<Utc>) -> Self {
        Self { entry_type, before }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryResource {
    pub source_key: String,
    pub resource_type: String,
}

impl QueryResource {
    pub fn new(resource_type: String, source_key: String) -> Self {
        Self {
            resource_type,
            source_key,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryTagByTagType {
    pub tag_type: String,
    pub source_key: String,
}

impl QueryTagByTagType {
    pub fn new(tag_type: String, source_key: String) -> Self {
        Self {
            tag_type,
            source_key,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryTagsByTagType {
    pub tag_type: String,
}

impl QueryTagsByTagType {
    pub fn new(tag_type: String) -> Self {
        Self { tag_type }
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
