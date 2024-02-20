use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>
            <h1>{ "About Us" }</h1>
            <p>{ "This is the about page for our website." }</p>
        </div>
    }
}
