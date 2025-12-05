use chrono::{DateTime, Utc};
use contour_rust_pdk::command::{Command, Cron, EmptyJoins, Scraper, Transform};
use contour_rust_pdk::response::{ExtractResponse, TransformResponse};
use contour_rust_pdk::{extract_fn, transform_fn};
use extism_pdk::FnResult;
use serde::{Deserialize, Serialize};

#[test]
fn test_extract_fn_macro_with_scraper() {
    #[derive(Deserialize, Serialize)]
    struct TestData {
        id: String,
        value: i32,
    }

    #[extract_fn]
    pub fn extract_scraper_data(
        _scraping_data: Scraper<TestData>,
    ) -> FnResult<Option<ExtractResponse>> {
        // Process scraping data
        Ok(None)
    }

    // Test that the macro generates the correct extern function
    assert_eq!(unsafe { extract_scraper_data() }, 0);
}

#[test]
fn test_extract_fn_macro_with_cron() {
    #[extract_fn]
    pub fn extract_cron_job(_cron: Cron) -> FnResult<Option<ExtractResponse>> {
        // Process cron job
        Ok(None)
    }

    // Test that the macro generates the correct extern function
    assert_eq!(unsafe { extract_cron_job() }, 0);
}

#[test]
fn test_extract_fn_macro_with_command() {
    #[derive(Deserialize, Serialize)]
    struct CustomCommand {
        action: String,
        params: Vec<String>,
    }

    #[extract_fn]
    pub fn extract_command(_cmd: Command<CustomCommand>) -> FnResult<Option<ExtractResponse>> {
        // Process command
        Ok(None)
    }

    // Test that the macro generates the correct extern function
    assert_eq!(unsafe { extract_command() }, 0);
}

#[test]
fn test_transform_fn_macro() {
    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct SourceRecord {
        id: String,
        name: String,
        amount: f64,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct Metadata {
        processed_at: DateTime<Utc>,
        version: u32,
    }

    #[transform_fn]
    pub fn transform_records(
        _transform: Transform<SourceRecord, EmptyJoins, Metadata>,
    ) -> FnResult<TransformResponse<()>> {
        Ok(TransformResponse::None)
    }

    // Test that the macro generates the correct extern function
    assert_eq!(unsafe { transform_records() }, 0);
}
