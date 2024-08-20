#![allow(improper_ctypes_definitions)]
#![allow(improper_ctypes)]

pub mod command;
pub mod config;
pub mod csv;
pub mod io;
pub mod models;

// pub use contour_interface::{command, io, models};
pub use contour_macros::listener_fn;

use anyhow::Result;
pub use extism_pdk::{self, FnResult};
use graphql_client::{GraphQLQuery, Response};
pub use rust_decimal;
pub use rust_decimal_macros::dec;
use serde::{de::DeserializeOwned, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
#[extism_pdk::host_fn]
extern "ExtismHost" {
    fn query_host(input: String) -> String;
    fn query_dimension_host(input: String) -> String;
    fn query_entry_host(input: String) -> String;
    fn query_last_entry_host(input: String) -> String;
    fn query_resource_host(input: String) -> String;
    fn query_tag_host(input: String) -> String;
    fn query_tags_host(input: String) -> String;
    fn query_tag_by_data_type_host(input: String) -> String;
    fn query_tags_by_data_type_host(input: String) -> String;
    fn upsert_resource_host(input: String) -> String;
    fn upsert_tag_host(input: String) -> String;
    fn upsert_entry_host(input: String) -> String;
    fn update_resource_host(input: String) -> String;
    fn update_tag_host(input: String) -> String;
    fn update_entry_host(input: String) -> String;
    fn make_request_host(input: String) -> String;
}

#[cfg(not(target_arch = "wasm32"))]
#[mockall::automock]
pub mod host_fns {
    use anyhow::Result;

    extern "C" {
        pub fn query_host(input: String) -> Result<String>;
        pub fn query_dimension_host(input: String) -> Result<String>;
        pub fn query_entry_host(input: String) -> Result<String>;
        pub fn query_last_entry_host(input: String) -> Result<String>;
        pub fn query_resource_host(input: String) -> Result<String>;
        pub fn query_tag_host(input: String) -> Result<String>;
        pub fn query_tags_host(input: String) -> Result<String>;
        pub fn query_tag_by_data_type_host(input: String) -> Result<String>;
        pub fn query_tags_by_data_type_host(input: String) -> Result<String>;
        pub fn upsert_resource_host(input: String) -> Result<String>;
        pub fn upsert_tag_host(input: String) -> Result<String>;
        pub fn upsert_entry_host(input: String) -> Result<String>;
        pub fn update_resource_host(input: String) -> Result<String>;
        pub fn update_tag_host(input: String) -> Result<String>;
        pub fn update_entry_host(input: String) -> Result<String>;
        pub fn make_request_host(input: String) -> Result<String>;
    }
}

#[cfg(not(target_arch = "wasm32"))]
use host_fns::*;

pub fn query<Q: GraphQLQuery>(variables: Q::Variables) -> Result<Response<Q::ResponseData>> {
    let json = Q::build_query(variables);
    let result = unsafe { query_host(serde_json::to_string(&json)?)? };
    let output: Response<Q::ResponseData> = serde_json::from_str(&result)?;
    Ok(output)
}

pub fn query_dimension(input: io::QueryDimension) -> Result<Option<models::Dimension>> {
    let result = unsafe { query_dimension_host(serde_json::to_string(&input)?)? };
    let output = serde_json::from_str(&result)?;
    Ok(output)
}

pub fn query_entry<E: DeserializeOwned + Send + Sync>(
    input: io::QueryEntry,
) -> Result<Option<models::Entry<E>>> {
    let result = unsafe { query_entry_host(serde_json::to_string(&input)?)? };
    let output = serde_json::from_str(&result)?;
    Ok(output)
}

pub fn query_last_entry<E: DeserializeOwned + Send + Sync>(
    input: io::QueryLastEntry,
) -> Result<Option<models::Entry<E>>> {
    let result = unsafe { query_last_entry_host(serde_json::to_string(&input)?)? };
    let output = serde_json::from_str(&result)?;
    Ok(output)
}

pub fn query_resource<R: DeserializeOwned + Send + Sync>(
    input: io::QueryResource,
) -> Result<Option<models::Resource<R>>> {
    let result = unsafe { query_resource_host(serde_json::to_string(&input)?)? };
    let output = serde_json::from_str(&result)?;
    Ok(output)
}

pub fn query_tag<T: DeserializeOwned + Send + Sync>(
    input: io::QueryTag,
) -> Result<Option<models::Tag<T>>> {
    let result = unsafe { query_tag_host(serde_json::to_string(&input)?)? };
    let output = serde_json::from_str(&result)?;
    Ok(output)
}

pub fn query_tags<T: DeserializeOwned + Send + Sync>(
    input: io::QueryTags,
) -> Result<Vec<models::Tag<T>>> {
    let result = unsafe { query_tags_host(serde_json::to_string(&input)?)? };
    let output = serde_json::from_str(&result)?;
    Ok(output)
}

pub fn query_tag_by_data_type<T: DeserializeOwned + Send + Sync>(
    input: io::QueryTagByDataType,
) -> Result<Option<models::Tag<T>>> {
    let result = unsafe { query_tag_by_data_type_host(serde_json::to_string(&input)?)? };
    let output = serde_json::from_str(&result)?;
    Ok(output)
}

pub fn query_tags_by_data_type<T: DeserializeOwned + Send + Sync>(
    input: io::QueryTagsByDataType,
) -> Result<Vec<models::Tag<T>>> {
    let result = unsafe { query_tags_by_data_type_host(serde_json::to_string(&input)?)? };
    let output = serde_json::from_str(&result)?;
    Ok(output)
}

pub fn upsert_resource<R: Serialize>(input: io::ResourceInput<R>) -> Result<Uuid> {
    let result = unsafe { upsert_resource_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn upsert_tag<T: Serialize>(input: io::TagInput<T>) -> Result<Uuid> {
    let result = unsafe { upsert_tag_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn upsert_entry<E: Serialize>(input: io::EntryInput<E>) -> Result<Uuid> {
    let result = unsafe { upsert_entry_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn update_resource<R: Serialize>(input: io::ResourceInput<R>) -> Result<Uuid> {
    let result = unsafe { update_resource_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn update_tag<T: Serialize>(input: io::TagInput<T>) -> Result<Uuid> {
    let result = unsafe { update_tag_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn update_entry<E: Serialize>(input: io::EntryInput<E>) -> Result<Uuid> {
    let result = unsafe { update_entry_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn make_request<B: Serialize, R: DeserializeOwned + Send + Sync>(
    input: io::RequestInput<B>,
) -> Result<R> {
    let result = unsafe { make_request_host(serde_json::to_string(&input)?)? };
    let output = serde_json::from_str(&result)?;
    Ok(output)
}
