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

    #[at("/spc")]
    SPCBook,

    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::About => {
            html! { <About /> }
        }

        Route::SPCBook => {
            html! { <BookRedirect /> }
        }

        Route::Contact => {
            html! { <Contact /> }
        }

        Route::Home => {
            html! { <Home /> }
        }

        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
