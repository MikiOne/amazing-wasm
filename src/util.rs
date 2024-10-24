use js_sys::Promise;
use std::time::Duration;
use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

pub async fn sleep(duration: Duration) {
    JsFuture::from(Promise::new(&mut |yes, _| {
        window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &yes,
                duration.as_millis() as i32,
            )
            .unwrap();
    })).await.unwrap();
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn output_to_json<T: Serialize>(output: T) -> Result<String, JsValue> {
    let result = serde_json::to_string(&output)
        .map_err(|e| js_value(&format!("Failed to serialize Output: {:?}", e)))?;

    Ok(result)
}

pub fn js_value(json: &String) -> JsValue {
    JsValue::from_str(&*json)
}
