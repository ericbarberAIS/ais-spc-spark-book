// fetch_service.rs
use gloo_utils::format::JsValueSerdeExt;
// use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::Callback;

use crate::models::company::CompanyData;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct CompanyData {
//     pub company_name: String,
//     pub contact_email: String,
//     pub contact_name: String,
// }

pub struct FetchService;

impl FetchService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_company_info(
        url: &str,
        callback: Callback<Result<CompanyData, anyhow::Error>>,
    ) {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(url, &opts).unwrap();

        let window = web_sys::window().unwrap();
        let fetch = window.fetch_with_request(&request);

        spawn_local(async move {
            let response = wasm_bindgen_futures::JsFuture::from(fetch).await;

            match response {
                Ok(resp) => {
                    let resp: Response = resp.dyn_into().unwrap();
                    if resp.ok() {
                        let json = wasm_bindgen_futures::JsFuture::from(resp.json().unwrap()).await;
                        if let Ok(json) = json {
                            let company_info: Result<CompanyData, _> = json.into_serde();
                            let company_info = company_info.map_err(|e| anyhow::Error::from(e));
                            callback.emit(company_info);
                        } else {
                            callback.emit(Err(anyhow::Error::msg("Failed to parse JSON")));
                        }
                    } else {
                        callback.emit(Err(anyhow::Error::msg("HTTP request failed")));
                    }
                }
                Err(_) => callback.emit(Err(anyhow::Error::msg("Fetch failed"))),
            }
        });
    }
}

// use crate::models::company::CompanyData;
// use yew::prelude::*;
// //
// // pub async fn load_company_static_data() -> Option<CompanyData> {
// //     // Temporarily using static data
// //     // let static_company_data =[CompanyData {
// //     // company_name: "Adaptive IT Solution",
// //     // contact_email: "eric.barber@adaptiveitsolution.com",
// //     // contact_name: "Eric Austin Barber"
// //     // }];
// //     //
// //     // // Convert static data to a state to mimic the original structure
// //     // let company_data = use_state(|| static_company_data);
// //
// //     let company_data = use_state(|| {
// //         // Load and parse the JSON data at compile time
// //         let data_str = include_str!("../static/company/details.json");
// //         console::log_1(&format!("Loaded data: {}", data_str).into());
// //         serde_json::from_str::<CompanyData>(data_str).unwrap_or_else(|_| String::new())
// //     });
// //
// //     // Serialize and log the events data for debugging
// //     if let Ok(company_data) = to_string(&*company_data) {
// //         console::log_1(&format!("Company data: {}", company_data).into());
// //     } else {
// //         console::log_1(&"Failed to serialize company data".into());
// //     }
// // }
// //
// #[function_component(CompanyDataLoad)]
// pub fn event_listing() -> Html {
//     // // Temporarily using static data
//     // let static_company_data = [CompanyData {
//     //     company_name: "Adaptive IT Solution".to_string(),
//     //     contact_email: "eric.barber@adaptiveitsolution.com".to_string(),
//     //     contact_name: "Eric Austin Barber".to_string(),
//     // }];
//     // // // Convert static data to a state to mimic the original structure
//     // let static_company_data = use_state(|| static_company_data);
//
//     let company_data = use_state(|| {
//         // Load and parse the JSON data at compile time
//         let data_str = include_str!("../static/company/details.json");
//         serde_json::from_str::<CompanyData>(data_str)
//             .unwrap_or_else(|_| panic!("Failed to load company data file."))
//     });
//
//     html! {
//         <>
//             <h1>{"Company Details"}</h1>
//             <div>{format!("Company Name: {}", company_data.company_name)}</div>
//             <div>{format!("Contact Email: {}", company_data.contact_email)}</div>
//             <div>{format!("Contact Name: {}", company_data.contact_name)}</div>
//         </>
//     }
// }
