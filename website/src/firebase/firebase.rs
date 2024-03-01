use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

#[wasm_bindgen(module = "/firebase.js")]
extern "C" {
    #[wasm_bindgen(js_name = "sendFirebaseEmail")]
    pub fn send_firebase_email(email_data: &JsValue) -> Promise;
}

// Adjusted function to correctly handle the Promise and convert it to a Rust Future
#[wasm_bindgen]
pub async fn send_rust_email(email_data: JsValue) -> Result<JsValue, JsValue> {
    // Call the send_firebase_email function and await its Promise
    let promise = send_firebase_email(&email_data);
    let result = JsFuture::from(promise).await;

    // Directly return the result of awaiting the Promise, which is a Result<JsValue, JsValue>
    result
}

// // This function demonstrates how to wrap the call to send_firebase_email
// // and convert a Rust Future into a JavaScript Promise.
// #[wasm_bindgen]
// pub async fn send_rust_email(email_data: JsValue) -> Promise {
//     // Here, we immediately convert the Rust future representing the async operation
//     // into a JavaScript Promise. The future_to_promise utility handles conversion
//     // of Rust's Result to a JavaScript Promise that resolves or rejects.
//     future_to_promise(async move {
//         // Call the send_firebase_email function.
//         // Since send_firebase_email now returns a Promise, you can directly use it
//         // without further conversion. This example assumes that send_firebase_email
//         // has been adjusted to return a Promise directly.
//         let promise = send_firebase_email(&email_data);
//
//         // Await the JS promise using .await. Convert the JsValue result into a Rust Result.
//         let result = wasm_bindgen_futures::JsFuture::from(promise).await;
//
//         // Process the result, converting it to a Result<JsValue, JsValue> as needed.
//         // For simplicity, this example directly returns the result of the await.
//         // You might want to add error handling or other logic based on your app's needs.
//         result
//     })
// }
