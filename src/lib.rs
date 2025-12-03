#![allow(improper_ctypes_definitions)]
#![allow(improper_ctypes)]

pub mod command;
pub mod csv;
#[cfg(not(target_arch = "wasm32"))]
use host_fns::*;
pub mod io;
pub mod models;
pub mod response;

pub use contour_rust_pdk_macros::listener_fn;

use anyhow::{Result, anyhow};
pub use extism_pdk::{self, FnResult};
pub use rust_decimal;
pub use rust_decimal_macros::dec;
use serde::{Serialize, de::DeserializeOwned};
use std::str::FromStr;
use uuid::Uuid;

use crate::io::{
    DeletedRecordHistoryInput, EntryInput, RecordHistoryInput, RequestInput, ResourceInput,
    TagInput, TimezoneInput,
};

#[cfg(target_arch = "wasm32")]
#[extism_pdk::host_fn]
extern "ExtismHost" {
    fn make_request_host(input: String) -> String;
    fn config_host(input: String) -> String;
    fn upsert_resource_host(input: String) -> String;
    fn upsert_tag_host(input: String) -> String;
    fn upsert_entry_host(input: String) -> String;
    fn find_timezone_host(input: String) -> String;
    fn upsert_records_host(input: String) -> String;
    fn upsert_measurement_host(input: String) -> String;
    fn delete_records_host(input: String) -> String;
}

#[cfg(not(target_arch = "wasm32"))]
#[mockall::automock]
pub mod host_fns {
    use anyhow::Result;

    unsafe extern "C" {
        pub fn make_request_host(input: String) -> Result<String>;
        pub fn config_host(input: String) -> Result<String>;
        pub fn upsert_resource_host(input: String) -> Result<String>;
        pub fn upsert_tag_host(input: String) -> Result<String>;
        pub fn upsert_entry_host(input: String) -> Result<String>;
        pub fn find_timezone_host(input: String) -> Result<String>;
        pub fn upsert_records_host(input: String) -> Result<String>;
        pub fn upsert_measurement_host(input: String) -> Result<String>;
        pub fn delete_records_host(input: String) -> Result<String>;
    }
}

pub fn config(input: &str) -> Result<String> {
    unsafe { config_host(input.to_string()) }
}

pub fn upsert_resource<R: Serialize>(input: ResourceInput<R>) -> Result<Uuid> {
    let result = unsafe { upsert_resource_host(serde_json::to_string(&input)?)? };
    Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))
}

pub fn upsert_tag<T: Serialize>(input: TagInput<T>) -> Result<Uuid> {
    let result = unsafe { upsert_tag_host(serde_json::to_string(&input)?)? };
    Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))
}

pub fn upsert_entry(input: EntryInput) -> Result<Uuid> {
    let result = unsafe { upsert_entry_host(serde_json::to_string(&input)?)? };
    Uuid::from_str(&result).map_err(|_| anyhow::anyhow!("Invalid UUID"))
}

pub fn upsert_record_histories<R: Serialize + DeserializeOwned, M: Serialize + DeserializeOwned>(
    input: Vec<RecordHistoryInput<R, M>>,
) -> Result<()> {
    unsafe { upsert_records_host(serde_json::to_string(&input)?)? };
    Ok(())
}

pub fn delete_record_histories(input: Vec<DeletedRecordHistoryInput>) -> Result<()> {
    unsafe { delete_records_host(serde_json::to_string(&input)?)? };
    Ok(())
}

pub fn make_request<B: Serialize, R: DeserializeOwned + Send + Sync>(
    input: RequestInput<B>,
) -> Result<R> {
    let result = unsafe { make_request_host(serde_json::to_string(&input)?)? };
    serde_json::from_str::<R>(&result).map_err(|_| anyhow!("Failed to parse response: {}", &result))
}

pub fn find_timezone(input: TimezoneInput) -> Result<String> {
    let result = unsafe { find_timezone_host(serde_json::to_string(&input)?)? };
    Ok(result)
}
