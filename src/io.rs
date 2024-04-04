use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effective {
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum InputLineType {
    Receivable,
    Asset,
    Liability,
    Equity,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputEntryAndLines<E> {
    pub effective: Effective,
    pub source_key: String,
    pub entry: E,
    pub entry_type: String,
    pub lines: Vec<InputLine>,
}

impl<E> InputEntryAndLines<E> {
    pub fn new(effective: Effective, source_key: String, entry: E, lines: Vec<InputLine>) -> Self {
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
pub struct InputLine {
    pub line_type: InputLineType,
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
impl InputLine {
    pub fn new(
        line_type: InputLineType,
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
pub struct InputRequest<B, M> {
    pub domain: String,
    pub endpoint: String,
    pub method: String,
    pub query_params: HashMap<String, String>,
    pub query_parameters: Vec<(String, String)>,
    pub body: Option<B>,
    pub headers: HashMap<String, String>,
    pub metadata: Option<M>,
    pub metadata_type: String,
    pub response_type: String,
}

pub struct RequestBuilder<B, M> {
    domain: String,
    endpoint: String,
    method: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    query_parameters: Vec<(String, String)>,
    body: Option<B>,
    metadata: Option<M>,
    metadata_type: String,
    response_type: String,
}

impl<B, M> RequestBuilder<B, M> {
    pub fn new(domain: String, path: String, method: String, response_type: String) -> Self {
        Self {
            domain,
            endpoint: path,
            method,
            query_params: HashMap::default(),
            query_parameters: Vec::new(),
            headers: HashMap::default(),
            body: None,
            metadata: None,
            metadata_type: "()".to_string(),
            response_type,
        }
    }

    pub fn add_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn add_query_params(mut self, query_params: HashMap<String, String>) -> Self {
        self.query_params = query_params;
        self
    }

    pub fn add_query_parameters(mut self, query_parameters: Vec<(String, String)>) -> Self {
        self.query_parameters = query_parameters;
        self
    }

    pub fn add_body(mut self, body: B) -> Result<Self> {
        self.body = Some(body);
        Ok(self)
    }

    pub fn add_metadata(mut self, metadata: M) -> Result<Self> {
        self.metadata_type = get_type(&metadata);
        self.metadata = Some(metadata);
        Ok(self)
    }

    pub fn build(self) -> InputRequest<B, M> {
        InputRequest {
            domain: self.domain,
            endpoint: self.endpoint,
            method: self.method,
            query_params: self.query_params,
            query_parameters: self.query_parameters,
            body: self.body,
            headers: self.headers,
            metadata: self.metadata,
            metadata_type: self.metadata_type,
            response_type: self.response_type,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputResource<R> {
    pub name: Option<String>,
    pub unit: String,
    pub source_key: String,
    pub resource: R,
    pub resource_type: String,
}

impl<R> InputResource<R> {
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
pub struct InputDimension {
    pub slug: String,
    pub name: Option<String>,
}

impl InputDimension {
    pub fn new(slug: String, name: Option<String>) -> Self {
        Self { slug, name }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputTag<T> {
    pub name: Option<String>,
    pub source_key: String,
    pub dimension_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub tag: T,
    pub tag_type: String,
}

impl<T> InputTag<T> {
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
pub struct InputAgent<A> {
    pub source_key: String,
    pub name: Option<String>,
    pub agent: A,
    pub agent_type: String,
}

impl<A> InputAgent<A> {
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
pub struct InputLineTag {
    pub line_id: Uuid,
    pub tag_id: Uuid,
}

impl InputLineTag {
    pub fn new(line_id: Uuid, tag_id: Uuid) -> Self {
        Self { line_id, tag_id }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteEntry {
    pub source_key: String,
}

impl DeleteEntry {
    pub fn new(source_key: String) -> Self {
        Self { source_key }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteResource {
    pub source_key: String,
}

impl DeleteResource {
    pub fn new(source_key: String) -> Self {
        Self { source_key }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteAgent {
    pub source_key: String,
}

impl DeleteAgent {
    pub fn new(source_key: String) -> Self {
        Self { source_key }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HandlerInput<C> {
    pub command_type: String,
    pub command: C,
}

impl<C> HandlerInput<C> {
    pub fn new(command_type: String, command: C) -> Self {
        Self {
            command_type,
            command,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StateOutput {
    pub archived: bool,
    pub data: Value,
}

impl StateOutput {
    pub fn new(archived: bool, data: Value) -> Self {
        Self { archived, data }
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

        let input_resource = InputResource::new(
            Some("test".to_string()),
            "test".to_string(),
            "test".to_string(),
            test_struct,
        );

        assert_eq!(get_type(&input_resource), "TestStruct");
    }
}
