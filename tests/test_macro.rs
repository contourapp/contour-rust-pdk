use contour_rust_pdk::command::Scraper;
use contour_rust_pdk_macros::listener_fn;
use extism_pdk::FnResult;
use serde::Deserialize;

#[test]
fn test_listener_fn_macro() {
    #[derive(Deserialize)]
    struct Test;

    #[listener_fn]
    pub fn test_fn(_scraping_data: Scraper<Test>) -> FnResult<()> {
        Ok(())
    }

    assert_eq!(unsafe { test_fn() }, 0);
}
