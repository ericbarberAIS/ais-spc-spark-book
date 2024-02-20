use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <p>{ "This is the home page for our website." }</p>
        </div>
    }
}
