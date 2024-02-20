use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div>
            <h1>{ "Contact Us" }</h1>
            <p>{ "This is the contact page for our website." }</p>
        </div>
    }
}
