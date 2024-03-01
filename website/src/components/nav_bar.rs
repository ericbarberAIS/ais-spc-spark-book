use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct NavbarProps {
    pub company_name: Option<String>,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let navbar_active = use_state(|| false);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();
        Callback::from(move |_| navbar_active.set(!*navbar_active))
    };

    let active_class = if *navbar_active { "is-active" } else { "" };

    html! {
        <div class="hero-head">
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    // Display the company name if it's available
                    <h1 class="navbar-item is-size-3">
                        { props.company_name.clone().unwrap_or_else(|| "Default Company".to_string()) }
                    </h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={toggle_navbar}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-end mr-2">
                        // Add router links here

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

                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>

                        <Link<Route> classes={classes!("navbar-item")} to={Route::About}>
                            { "About Us" }
                        </Link<Route>>

                        <Link<Route> classes={classes!("navbar-item")} to={Route::Contact}>
                            { "Contact" }
                        </Link<Route>>

                        // Additional links and components as before
                    </div>
                </div>
            </nav>
        </div>
    }
}
