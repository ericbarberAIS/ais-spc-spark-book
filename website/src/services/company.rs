use crate::models::company::CompanyData;
use yew::prelude::*;
//
// pub async fn load_company_static_data() -> Option<CompanyData> {
//     // Temporarily using static data
//     // let static_company_data =[CompanyData {
//     // company_name: "Adaptive IT Solution",
//     // contact_email: "eric.barber@adaptiveitsolution.com",
//     // contact_name: "Eric Austin Barber"
//     // }];
//     //
//     // // Convert static data to a state to mimic the original structure
//     // let company_data = use_state(|| static_company_data);
//
//     let company_data = use_state(|| {
//         // Load and parse the JSON data at compile time
//         let data_str = include_str!("../static/company/details.json");
//         console::log_1(&format!("Loaded data: {}", data_str).into());
//         serde_json::from_str::<CompanyData>(data_str).unwrap_or_else(|_| String::new())
//     });
//
//     // Serialize and log the events data for debugging
//     if let Ok(company_data) = to_string(&*company_data) {
//         console::log_1(&format!("Company data: {}", company_data).into());
//     } else {
//         console::log_1(&"Failed to serialize company data".into());
//     }
// }
//
#[function_component(CompanyDataLoad)]
pub fn event_listing() -> Html {
    // // Temporarily using static data
    // let static_company_data = [CompanyData {
    //     company_name: "Adaptive IT Solution".to_string(),
    //     contact_email: "eric.barber@adaptiveitsolution.com".to_string(),
    //     contact_name: "Eric Austin Barber".to_string(),
    // }];
    // // // Convert static data to a state to mimic the original structure
    // let static_company_data = use_state(|| static_company_data);

    let company_data = use_state(|| {
        // Load and parse the JSON data at compile time
        let data_str = include_str!("../static/company/details.json");
        serde_json::from_str::<CompanyData>(data_str)
            .unwrap_or_else(|_| panic!("Failed to load company data file."))
    });

    html! {
        <>
            <h1>{"Company Details"}</h1>
            <div>{format!("Company Name: {}", company_data.company_name)}</div>
            <div>{format!("Contact Email: {}", company_data.contact_email)}</div>
            <div>{format!("Contact Name: {}", company_data.contact_name)}</div>
        </>
    }
}
