use crate::components::footer::Footer;
use crate::components::nav_bar::Navbar;
use crate::router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::models::company::CompanyData;
use crate::services::company::CompanyDataLoad;

pub struct App {}
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                // { self.view_nav(ctx.link()) }
            // <Navbar company_data=self.company_data.clone()/>
            <Navbar />
            <CompanyDataLoad />
            <div class="container">
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </div>
            <Footer />
            </BrowserRouter>
        }
    }
}
