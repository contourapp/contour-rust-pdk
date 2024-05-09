use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
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
    DateTimeRange(DateTimeRange),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LineTypeInput {
    Receivable,
    Asset,
    Liability,
    Equity,
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
    pub line_type: LineTypeInput,
    pub consumes_line_id: Option<Uuid>,
    pub resource_id: Uuid,
    pub agent_id: Option<Uuid>,
    pub debit: Decimal,
    pub credit: Decimal,
    pub ratio: Decimal,
    pub description: Option<String>,
    pub tags: Vec<Uuid>,
}

#[allow(clippy::too_many_arguments)]
impl LineInput {
    pub fn new(
        line_type: LineTypeInput,
        consumes_line_id: Option<Uuid>,
        resource_id: Uuid,
        agent_id: Option<Uuid>,
        debit: Decimal,
        credit: Decimal,
        ratio: Decimal,
        description: Option<String>,
        tags: Vec<Uuid>,
    ) -> Self {
        Self {
            line_type,
            consumes_line_id,
            resource_id,
            agent_id,
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
    pub unit: String,
    pub source_key: String,
    pub resource: R,
    pub resource_type: String,
}

impl<R> ResourceInput<R> {
    pub fn new(name: Option<String>, unit: String, source_key: String, resource: R) -> Self {
        let resource_type = get_type(&resource);
        Self {
            name,
            unit,
            source_key,
            resource,
            resource_type,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DimensionInput {
    pub slug: String,
    pub name: Option<String>,
}

impl DimensionInput {
    pub fn new(slug: String, name: Option<String>) -> Self {
        Self { slug, name }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagInput<T> {
    pub name: Option<String>,
    pub source_key: String,
    pub dimension_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub tag: T,
    pub tag_type: String,
}

impl<T> TagInput<T> {
    pub fn new(
        name: Option<String>,
        source_key: String,
        dimension_id: Uuid,
        parent_id: Option<Uuid>,
        tag: T,
    ) -> Self {
        let tag_type = get_type(&tag);

        Self {
            name,
            source_key,
            dimension_id,
            parent_id,
            tag,
            tag_type,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentInput<A> {
    pub source_key: String,
    pub name: Option<String>,
    pub agent: A,
    pub agent_type: String,
}

impl<A> AgentInput<A> {
    pub fn new(source_key: String, name: Option<String>, agent: A) -> Self {
        let agent_type = get_type(&agent);
        Self {
            source_key,
            name,
            agent,
            agent_type,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryAgent {
    pub source_key: String,
}

impl QueryAgent {
    pub fn new(source_key: String) -> Self {
        Self { source_key }
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
pub struct QueryDimension {
    pub slug: String,
}

impl QueryDimension {
    pub fn new(slug: String) -> Self {
        Self { slug }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryResource {
    pub source_key: String,
}

impl QueryResource {
    pub fn new(source_key: String) -> Self {
        Self { source_key }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryTag {
    pub dimension_id: Uuid,
    pub source_key: String,
}

impl QueryTag {
    pub fn new(dimension_id: Uuid, source_key: String) -> Self {
        Self {
            dimension_id,
            source_key,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryTags {
    pub dimension_id: Uuid,
}

impl QueryTags {
    pub fn new(dimension_id: Uuid) -> Self {
        Self { dimension_id }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapingArgs {
    pub base_url: String,
    pub username: String,
    pub password: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub location: Option<String>,
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
            test_struct,
        );

        assert_eq!(get_type(&input_resource), "TestStruct");
    }
}
