// use yew::prelude::*;
//
// #[function_component(Contact)]
// pub fn contact() -> Html {
//     html! {
//         <>
//             // Hero Section for Contact Page
//             <section class="hero is-primary">
//                 <div class="hero-body">
//                     <div class="container">
//                         <h1 class="title">
//                             { "Contact Us" }
//                         </h1>
//                         <h2 class="subtitle">
//                             { "We'd love to hear from you!" }
//                         </h2>
//                     </div>
//                 </div>
//             </section>
//
//             // Contact Information Section
//             <section class="section">
//                 <div class="container">
//                     <div class="columns">
//                         <div class="column is-half">
//                             <h3 class="title is-3">{ "Get In Touch" }</h3>
//                             <p>{ "Feel free to reach out to us with any questions, suggestions, or feedback." }</p>
//                             <br/>
//                             <p><strong>{ "Email: " }</strong>{ "eric.barber@adaptiveitsolution.com" }</p>
//                             <p><strong>{ "Phone: " }</strong>{ "+1 (480) 272-5014" }</p>
//                             // <p><strong>{ "Address: " }</strong>{ "123 Your Street, Your City, Country" }</p>
//                         </div>
//
//                         // Contact Form Section
//                         <div class="column is-half">
//                             <div class="field">
//                                 <label class="label">{ "Name" }</label>
//                                 <div class="control">
//                                     <input class="input" type="text" placeholder="Your Name"/>
//                                 </div>
//                             </div>
//                             <div class="field">
//                                 <label class="label">{ "Email" }</label>
//                                 <div class="control">
//                                     <input class="input" type="email" placeholder="Your Email"/>
//                                 </div>
//                             </div>
//                             <div class="field">
//                                 <label class="label">{ "Message" }</label>
//                                 <div class="control">
//                                     <textarea class="textarea" placeholder="Your Message"></textarea>
//                                 </div>
//                             </div>
//                             <div class="field">
//                                 <div class="control">
//                                     <button class="button is-link">{ "Send Message" }</button>
//                                 </div>
//                             </div>
//                         </div>
//                     </div>
//                 </div>
//             </section>
//         </>
//     }
// }

use crate::firebase::firebase::send_rust_email;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

#[derive(Serialize, Default, Clone)]
struct FormData {
    name: String,
    email: String,
    message: String,
}

#[function_component(Contact)]
pub fn contact() -> Html {
    let form_data = use_state(FormData::default);
    // let name = form_data.name.clone();
    // let email = form_data.email.clone();
    // let message = form_data.message.clone();

    let on_input = {
        let form_data = form_data.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let name = input.name();
            let value = input.value();
            let mut new_data = (*form_data).clone();
            match name.as_str() {
                "name" => new_data.name = value,
                "email" => new_data.email = value,
                "message" => new_data.message = value,
                _ => {}
            };
            form_data.set(new_data);
        })
    };

    // let on_input = {
    //     let form_data = form_data.clone();
    //     move |e: InputEvent| {
    //         let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
    //         let name = input.name();
    //         let value = input.value();
    //         form_data.set(FormData {
    //             name: if name == "name" {
    //                 value
    //             } else {
    //                 form_data.name.clone()
    //             },
    //             email: if name == "email" {
    //                 value
    //             } else {
    //                 form_data.email.clone()
    //             },
    //             message: if name == "message" {
    //                 value
    //             } else {
    //                 form_data.message.clone()
    //             },
    //             ..(*form_data).clone()
    //         });
    //     }
    // };

    let on_submit = {
        let form_data = form_data.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let data = (*form_data).clone();

            spawn_local(async move {
                let window = window().unwrap();
                let js_value = to_value(&data).unwrap();
                match send_rust_email(js_value).await {
                    Ok(_) => window
                        .alert_with_message("Email sent successfully!")
                        .unwrap(),
                    Err(err) => window
                        .alert_with_message(&format!("Error sending email: {:?}", err))
                        .unwrap(),
                }
            });
        })
    };

    // let on_submit = {
    //     let form_data = form_data.clone();
    //     Callback::from(move |e: SubmitEvent| {
    //         e.prevent_default(); // Prevent the form from submitting in the traditional way
    //         let data = form_data.clone();
    //
    //         spawn_local(async move {
    //             let window = window().unwrap();
    //             let js_value = to_value(&*data).unwrap();
    //             match send_rust_email(js_value).await {
    //                 Ok(_) => window
    //                     .alert_with_message("Email sent successfully!")
    //                     .unwrap(),
    //                 Err(err) => window
    //                     .alert_with_message(&format!("Error sending email: {:?}", err))
    //                     .unwrap(),
    //             }
    //         });
    //     })
    // };

    html! {
        <>
                // Hero Section for Contact Page
            <section class="hero is-primary">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">
                            { "Contact Us" }
                        </h1>
                        <h2 class="subtitle">
                            { "We'd love to hear from you!" }
                        </h2>
                    </div>
                </div>
            </section>

            // Contact Information Section
            <section class="section">
                <div class="container">
                    <div class="columns">
                        <div class="column is-half">
                            <h3 class="title is-3">{ "Get In Touch" }</h3>
                            <p>{ "Feel free to reach out to us with any questions, suggestions, or feedback." }</p>
                            <br/>
                            <p><strong>{ "Email: " }</strong>{ "eric.barber@adaptiveitsolution.com" }</p>
                            <p><strong>{ "Phone: " }</strong>{ "+1 (480) 272-5014" }</p>
                            // <p><strong>{ "Address: " }</strong>{ "123 Your Street, Your City, Country" }</p>
                        </div>

                        // Contact Form Section
                        // <div class="column is-half">
                        //     <form onsubmit={on_submit}>
                        //         <div class="field">
                        //             <label class="label">{ "Name" }</label>
                        //             <div class="control">
                        //                 <input class="input" type="text" name="name" placeholder="Your Name" value={form_data.name.clone()} oninput={on_input.clone()}/>
                        //             </div>
                        //         </div>
                        //         <div class="field">
                        //             <label class="label">{ "Email" }</label>
                        //             <div class="control">
                        //                 <input class="input" type="email" name="email" placeholder="Your Email" value={form_data.email.clone()} oninput={on_input.clone()}/>
                        //             </div>
                        //         </div>
                        //         <div class="field">
                        //             <label class="label">{ "Message" }</label>
                        //             <div class="control">
                        //                 <textarea class="textarea" name="message" placeholder="Your Message" value={form_data.message.clone()} oninput={on_input.clone()}></textarea>
                        //             </div>
                        //         </div>
                        //         <div class="field">
                        //             <div class="control">
                        //                 <button class="button is-link" type="submit">{ "Send Message" }</button>
                        //             </div>
                        //         </div>
                        //     </form>
                        // </div>
                    </div>
                </div>
            </section>
        </>

        // <>
        //     // Hero Section for Contact Page
        //     // Contact Information Section
        //
        //     // Contact Form Section - Updated with form submission handling
        //     <form onsubmit={on_submit}>
        //         <div class="field">
        //             <label class="label">{ "Name" }</label>
        //             <div class="control">
        //                 <input class="input" type="text" name="name" placeholder="Your Name" value={name} oninput={on_input.clone()}/>
        //             </div>
        //         </div>
        //         <div class="field">
        //             <label class="label">{ "Email" }</label>
        //             <div class="control">
        //                 <input class="input" type="email" name="email" placeholder="Your Email" value={email} oninput={on_input.clone()}/>
        //             </div>
        //         </div>
        //         <div class="field">
        //             <label class="label">{ "Message" }</label>
        //             <div class="control">
        //                 <textarea class="textarea" name="message" placeholder="Your Message" {oninput}>{form_data.message.clone()}</textarea>
        //             </div>
        //         </div>
        //         <div class="field">
        //             <div class="control">
        //                 <button class="button is-link" type="submit">{ "Send Message" }</button>
        //             </div>
        //         </div>
        //     </form>
        // </>
    }
}
