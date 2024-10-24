use wasm_bindgen::prelude::*;
use web_sys::*;

mod wasm_data;
#[macro_use]
mod util;
mod aptos_test;

use wasm_data::*;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log!("Hello Wasm!");

    let data = WasmBinData::parse_data().unwrap();
    log!("{}", format!("{:?}", data));


    // 获取浏览器窗口
    let window = window().expect("should have a window");

    // 获取document对象
    let document: Document = window.document().expect("should have a document");

    // 获取origin
    let origin = window.location().origin().expect("should have an origin");
    web_sys::console::log_1(&format!("Origin: {}", origin).into());

    // 获取referer
    let referer = document.referrer();
    web_sys::console::log_1(&format!("Referer: {}", referer).into());
}
