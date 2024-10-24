use tw_aptos::signer::Signer;
use tw_proto::Aptos::Proto;
use wasm_bindgen::prelude::*;
use web_sys::*;

mod wasm_data;
#[macro_use]
mod util;
mod aptos;

use aptos::*;
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

#[wasm_bindgen]
pub fn sign_txn() -> Result<String, JsValue> {
    let input = setup_proto_transaction(
        "0x07968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f30",
        "5d996aa76b3212142792d9130796cd2e11e3c445a93118c08414df4f66bc60ec",
        "transfer",
        99,
        33,
        3296766,
        3664390082,
        100,
        "",
        Some(OpsDetails::Transfer(Transfer {
            to: "0x07968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f30".to_string(),
            amount: 1000,
        })),
    );
    let output = Signer::sign_proto(input);
    Ok(format!("{:?}", output))
    // util::output_to_json(output)
}
