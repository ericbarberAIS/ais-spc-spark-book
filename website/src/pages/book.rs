use web_sys::window;
use yew::prelude::*;

#[function_component(BookRedirect)]
pub fn book_redirect() -> Html {
    use_effect(|| {
        if let Some(window) = window() {
            let _ = window.location().set_href("/book/index.html");
        }
        || ()
    });

    // JavaScript redirection or a message with a link to the book
    html! {
        <>
            <h1>{ "Redirecting to the Book..." }</h1>
            <p>{ "If you are not redirected, " }<a href="/book/index.html">{"click here"}</a>{"."}</p>
        </>
    }
    // html! {
    //     <>
    //         <iframe src="/book/index.html" style="width: 100%; height: 100vh; border: none;" />
    //     </>
    // }
}
