use crate::components::footer::Footer;
use crate::components::nav_bar::Navbar;
use crate::router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::models::company::CompanyData;
use crate::services::fetch_company::FetchService;

#[function_component(App)]
pub fn app() -> Html {
    // Use state to store the company info
    let company_info = use_state(|| None::<CompanyData>);
    // Perform the fetch operation when the component mounts
    {
        let company_info = company_info.clone();
        use_effect_with((), move |_| {
            // Clone again to move into the async block
            let company_info = company_info.clone();

            // Your fetch logic here, simplified for brevity
            wasm_bindgen_futures::spawn_local(async move {
                let callback = Callback::from(move |data: Result<CompanyData, anyhow::Error>| {
                    if let Ok(data) = data {
                        company_info.set(Some(data))
                    }
                    // Handle error case here if necessary
                });

                FetchService::get_company_info("/static/company/details.json", callback).await;
            });

            // Cleanup function, no-op in this case
            || ()
        });
    }

    html! {
        <BrowserRouter>
            <Navbar company_name={company_info.as_ref().map(|info| info.company_name.clone())}/>
            <div>
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </div>
            <Footer />
        </BrowserRouter>
    }
}
