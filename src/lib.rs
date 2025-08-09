#![allow(improper_ctypes_definitions)]
#![allow(improper_ctypes)]

pub mod command;
pub mod csv;
pub mod io;
pub mod models;

// pub use contour_interface::{command, io, models};
pub use contour_rust_pdk_macros::listener_fn;

use anyhow::{Result, anyhow};
pub use extism_pdk::{self, FnResult};
use graphql_client::{GraphQLQuery, Response};
use io::TimezoneInput;
pub use rust_decimal;
pub use rust_decimal_macros::dec;
use serde::{Serialize, de::DeserializeOwned};
use std::str::FromStr;
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
#[extism_pdk::host_fn]
extern "ExtismHost" {
    fn make_request_host(input: String) -> String;
    fn config_host(input: String) -> String;
    fn query_host(input: String) -> String;
    fn upsert_resource_host(input: String) -> String;
    fn upsert_tag_host(input: String) -> String;
    fn upsert_entry_host(input: String) -> String;
    fn upsert_entries_host(input: String) -> String;
    fn delete_entry_host(input: String) -> String;
    fn delete_resource_host(input: String) -> String;
    fn find_timezone_host(input: String) -> String;
    fn upsert_records_host(input: String) -> String;
    fn delete_record_host(input: String) -> String;
    fn upsert_measurement_host(input: String) -> String;
}

#[cfg(not(target_arch = "wasm32"))]
#[mockall::automock]
pub mod host_fns {
    use anyhow::Result;

    unsafe extern "C" {
        pub fn make_request_host(input: String) -> Result<String>;
        pub fn config_host(input: String) -> Result<String>;
        pub fn query_host(input: String) -> Result<String>;
        pub fn upsert_resource_host(input: String) -> Result<String>;
        pub fn upsert_tag_host(input: String) -> Result<String>;
        pub fn upsert_entry_host(input: String) -> Result<String>;
        pub fn upsert_entries_host(input: String) -> Result<String>;
        pub fn delete_entry_host(input: String) -> Result<String>;
        pub fn delete_resource_host(input: String) -> Result<String>;
        pub fn find_timezone_host(input: String) -> Result<String>;
        pub fn upsert_records_host(input: String) -> Result<String>;
        pub fn delete_record_host(input: String) -> Result<String>;
        pub fn upsert_measurement_host(input: String) -> Result<String>;
    }
}

#[cfg(not(target_arch = "wasm32"))]
use host_fns::*;

pub fn query<Q: GraphQLQuery>(variables: Q::Variables) -> Result<Response<Q::ResponseData>> {
    let json = Q::build_query(variables);
    let result = unsafe { query_host(serde_json::to_string(&json)?)? };
    serde_json::from_str(&result).map_err(|_| anyhow!("Failed to parse query: {}", &result))
}

pub fn config(input: &str) -> Result<String> {
    unsafe { config_host(input.to_string()) }
}

pub fn upsert_resource<R: Serialize>(input: io::ResourceInput<R>) -> Result<Uuid> {
    let result = unsafe { upsert_resource_host(serde_json::to_string(&input)?)? };
    Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))
}

pub fn upsert_tag<T: Serialize>(input: io::TagInput<T>) -> Result<Uuid> {
    let result = unsafe { upsert_tag_host(serde_json::to_string(&input)?)? };
    Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))
}

pub fn upsert_entry<E: Serialize>(input: io::EntryInput<E>) -> Result<Uuid> {
    let result = unsafe { upsert_entry_host(serde_json::to_string(&input)?)? };
    Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))
}

pub fn upsert_entries<E: Serialize>(entries: Vec<io::EntryInput<E>>) -> Result<()> {
    unsafe { upsert_entries_host(serde_json::to_string(&entries)?)? };
    Ok(())
}

pub fn delete_entry(input: String) -> Result<()> {
    unsafe { delete_entry_host(input)? };
    Ok(())
}

pub fn delete_resource(input: String) -> Result<()> {
    unsafe { delete_resource_host(input)? };
    Ok(())
}

pub fn make_request<B: Serialize, R: DeserializeOwned + Send + Sync>(
    input: io::RequestInput<B>,
) -> Result<R> {
    let result = unsafe { make_request_host(serde_json::to_string(&input)?)? };
    serde_json::from_str::<R>(&result).map_err(|_| anyhow!("Failed to parse response: {}", &result))
}

pub fn find_timezone(input: TimezoneInput) -> Result<String> {
    let result = unsafe { find_timezone_host(serde_json::to_string(&input)?)? };
    Ok(result)
}

pub fn upsert_records<R: Serialize + DeserializeOwned>(
    input: io::RecordsInput<R>,
) -> Result<Vec<Uuid>> {
    let result = unsafe { upsert_records_host(serde_json::to_string(&input)?)? };
    serde_json::from_str::<Vec<String>>(&result)?
        .iter()
        .map(|id| Uuid::from_str(id.as_str()).map_err(|_| anyhow!("Invalid UUID")))
        .collect()
}

pub fn delete_record(input: String) -> Result<()> {
    unsafe { delete_record_host(input)? };
    Ok(())
}
