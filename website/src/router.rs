use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::about::About;
use crate::pages::book::BookRedirect;
use crate::pages::contact::Contact;
use crate::pages::home::Home;
use crate::pages::page_not_found::PageNotFound;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/about")]
    About,

    #[at("/contact")]
    Contact,

    #[at("/")]
    Home,

    #[at("/spc")]
    SPCBook,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::About => {
            html! { <About /> }
        }

        Route::Contact => {
            html! { <Contact /> }
        }

        Route::Home => {
            html! { <Home /> }
        }

        Route::SPCBook => {
            html! { <BookRedirect /> }
        }

        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
