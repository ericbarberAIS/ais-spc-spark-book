use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

pub struct Navbar {
    pub navbar_active: bool,
}

pub enum Msg {
    ToggleNavbar,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let active_class = if self.navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Company Name" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={ctx.link().callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        // Add your links here

                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>

                        <Link<Route> classes={classes!("navbar-item")} to={Route::About}>
                            { "About Us" }
                        </Link<Route>>

                        <Link<Route> classes={classes!("navbar-item")} to={Route::Contact}>
                            { "Contact" }
                        </Link<Route>>

                        // Navbar drop down
                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "Material" }
                            </div>
                            <div class="navbar-dropdown">
                                <Link<Route> classes={classes!("navbar-item")} to={Route::SPCBook}>
                                    { "Statistical Process Control" }
                                </Link<Route>>
                            </div>
                        </div>
                        // end of drop down menu
                    </div>
                </div>
            </nav>
        }
    }
}
