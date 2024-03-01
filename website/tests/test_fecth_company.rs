#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::fetch_company::CompanyData;
    use crate::services::fetch_company::FetchService;
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use wasm_bindgen_test::*;
    use yew::Callback;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_fetch_service_get_company_info() {
        let server = MockServer::start();

        let _mock = server.mock(|when, then| {
            when.method(GET).path("/test-url");
            then.status(200).body(r#"{"name": "Test Company"}"#);
        });

        let url = server.url("/test-url"); //format!("{}{}", server.url(), "/test-url");

        let (callback, fut) = Callback::<Result<CompanyData, anyhow::Error>>::once_future();
        FetchService::get_company_info("/test-url", callback).await;

        let result = fut.await;
        assert!(result.is_ok());
        let company_info = result.expect("Should have company info");
        assert_eq!(company_info.name, "Test Company");
    }
}
