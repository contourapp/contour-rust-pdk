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
pub use rust_decimal;
pub use rust_decimal_macros::dec;
use serde::{de::DeserializeOwned, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
#[extism_pdk::host_fn]
extern "ExtismHost" {
    fn query_agent_host(input: String) -> String;
    fn query_dimension_host(input: String) -> String;
    fn query_entry_host(input: String) -> String;
    fn query_last_entry_host(input: String) -> String;
    fn query_resource_host(input: String) -> String;
    fn query_tag_host(input: String) -> String;
    fn query_tags_host(input: String) -> String;
    fn insert_agent_host(input: String) -> String;
    fn insert_entry_host(input: String) -> String;
    fn insert_lines_host(input: String) -> String;
    fn insert_resource_host(input: String) -> String;
    fn insert_tag_host(input: String) -> String;
    fn insert_request_host(input: String) -> String;
    fn upsert_agent_host(input: String) -> String;
    fn upsert_entry_host(input: String) -> String;
    fn upsert_lines_host(input: String);
    fn upsert_entry_and_lines_host(input: String) -> String;
    fn upsert_resource_host(input: String) -> String;
    fn upsert_tag_host(input: String) -> String;
    fn scrape_sierrapay_host(input: String) -> String;
    fn scrape_cryptopay_host(input: String) -> String;
    fn make_request_host(input: String) -> String;
}

#[cfg(not(target_arch = "wasm32"))]
#[mockall::automock]
pub mod host_fns {
    use anyhow::Result;

    extern "C" {
        pub fn query_agent_host(input: String) -> Result<String>;
        pub fn query_dimension_host(input: String) -> Result<String>;
        pub fn query_entry_host(input: String) -> Result<String>;
        pub fn query_last_entry_host(input: String) -> Result<String>;
        pub fn query_resource_host(input: String) -> Result<String>;
        pub fn query_tag_host(input: String) -> Result<String>;
        pub fn query_tags_host(input: String) -> Result<String>;
        pub fn insert_agent_host(input: String) -> Result<String>;
        pub fn insert_entry_host(input: String) -> Result<String>;
        pub fn insert_lines_host(input: String) -> Result<String>;
        pub fn insert_resource_host(input: String) -> Result<String>;
        pub fn insert_tag_host(input: String) -> Result<String>;
        pub fn insert_request_host(input: String) -> Result<String>;
        pub fn upsert_agent_host(input: String) -> Result<String>;
        pub fn upsert_entry_host(input: String) -> Result<String>;
        pub fn upsert_lines_host(input: String) -> Result<()>;
        pub fn upsert_entry_and_lines_host(input: String) -> Result<String>;
        pub fn upsert_resource_host(input: String) -> Result<String>;
        pub fn upsert_tag_host(input: String) -> Result<String>;
        pub fn scrape_sierrapay_host(input: String) -> Result<String>;
        pub fn scrape_cryptopay_host(input: String) -> Result<String>;
        pub fn make_request_host(input: String) -> Result<String>;
    }
}

#[cfg(not(target_arch = "wasm32"))]
use host_fns::*;

pub fn query_agent<A: DeserializeOwned>(input: io::QueryAgent) -> Result<Option<models::Agent<A>>> {
    let result = unsafe { query_agent_host(serde_json::to_string(&input)?)? };
    let output = serde_json::from_str(&result)?;
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

pub fn insert_agent<A: Serialize>(input: io::InputAgent<A>) -> Result<Uuid> {
    let result = unsafe { insert_agent_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn insert_request<B: Serialize, M: Serialize>(input: io::InputRequest<B, M>) -> Result<Uuid> {
    let result = unsafe { insert_request_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn insert_entry<E: Serialize>(input: io::InputEntry<E>) -> Result<Uuid> {
    let result = unsafe { insert_entry_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn insert_lines(input: io::InputLines) -> Result<()> {
    unsafe { insert_lines_host(serde_json::to_string(&input)?)? };
    Ok(())
}

pub fn insert_resource<R: Serialize>(input: io::InputResource<R>) -> Result<Uuid> {
    let result = unsafe { insert_resource_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn insert_tag<T: Serialize>(input: io::InputTag<T>) -> Result<Uuid> {
    let result = unsafe { insert_tag_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn upsert_agent<A: Serialize>(input: io::InputAgent<A>) -> Result<Uuid> {
    let result = unsafe { upsert_agent_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn upsert_entry<E: Serialize>(input: io::InputEntry<E>) -> Result<Uuid> {
    let result = unsafe { upsert_entry_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn upsert_lines(input: io::InputLines) -> Result<()> {
    unsafe { upsert_lines_host(serde_json::to_string(&input)?)? };
    Ok(())
}

pub fn upsert_entry_and_lines<E: Serialize>(input: io::InputEntry<E>) -> Result<Uuid> {
    let result = unsafe { upsert_entry_and_lines_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn upsert_resource<R: Serialize>(input: io::InputResource<R>) -> Result<Uuid> {
    let result = unsafe { upsert_resource_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn upsert_tag<T: Serialize>(input: io::InputTag<T>) -> Result<Uuid> {
    let result = unsafe { upsert_tag_host(serde_json::to_string(&input)?)? };
    let output = Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))?;
    Ok(output)
}

pub fn scrape_sierrapay(input: io::ScrapingArgs) -> Result<String> {
    let result = unsafe { scrape_sierrapay_host(serde_json::to_string(&input)?)? };
    Ok(result)
}

pub fn scrape_cryptopay(input: io::ScrapingArgs) -> Result<String> {
    let result = unsafe { scrape_cryptopay_host(serde_json::to_string(&input)?)? };
    Ok(result)
}

pub fn make_request<B: Serialize, M: Serialize, R: DeserializeOwned + Send + Sync>(
    input: io::InputRequest<B, M>,
) -> Result<R> {
    let result = unsafe { make_request_host(serde_json::to_string(&input)?)? };
    let output = serde_json::from_str(&result)?;
    Ok(output)
}
