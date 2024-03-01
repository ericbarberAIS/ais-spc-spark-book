use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

// #[function_component(Home)]
// pub fn home() -> Html {
//     html! {
//         <div>
//             <h1>{ "Home" }</h1>
//             <p>{ "This is the home page for our website." }</p>
//         </div>
//     }
// }

#[function_component(Home)]
pub fn landing_page() -> Html {
    html! {
        <>
            // Hero Section
            <section class="section" style="background-image: url('static/images/disorganized-background-image.webp'); background-size: cover; background-position: center;">
                <div class="container">
                    <div style="background: rgba(255, 255, 255, 0.7); padding: 2rem; border-radius: 8px;">
                        <p class="title">
                            { "Transforming Cloud Complexity into Organization Efficiency" }
                        </p>
                        <p class="subtitle">
                            { "Discover how analysis of cloud solutions can streamline operations for organizations of all sizes." }
                        </p>
                    </div>
                </div>
            </section>

            // Benefits Section
            <section class="section">
                <div class="container">
                    <div style="background: rgba(255, 255, 255, 0.7); padding: 2rem; border-radius: 8px;">
                        <h1 class="title">{ "Experience Unparalleled Efficiency and Scalability" }</h1>
                        <div class="content">
                            <p>{ "Discover the benefits of implementing your cloud management system, including improved visibility, scalability, operational efficiency, and user satisfaction." }</p>
                        </div>
                    </div>
                </div>
            </section>

            // Call to Action (Final)
            <section class="section" style="background-image: url('static/images/organized-background-image.webp'); background-size: cover; background-position: center;">
                <div class="container has-text-centered">
                    <div style="background: rgba(255, 255, 255, 0.7); padding: 2rem; border-radius: 8px;">
                        <h1 class="title">{ "Ready to Elevate Your Cloud Experience?" }</h1>
                        <div class="buttons is-centered is-justify-content-center">
                            <Link<Route> classes={classes!("button", "is-large", "is-primary")} to={Route::Contact}>
                                { "Contact Us Today" }
                            </Link<Route>>
                        </div>
                    </div>
                </div>
            </section>

            // Footer
            // <footer class="footer">
            //     <div class="content has-text-centered">
            //         <p>{ "© 2024 Your Company. All Rights Reserved." }</p>
            //     </div>
            // </footer>
        </>
    }
}
