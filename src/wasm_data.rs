// #![cfg(feature = "wasm")]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

pub const WASM_DATA: &[u8] = include_bytes!("../embed/WasmData");

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct WasmBinData {
    private_key: String,
    domain: String,
}

#[wasm_bindgen]
impl WasmBinData {
    pub fn parse_data() -> Result<Self, JsValue> {
        serde_json::from_slice(WASM_DATA).map_err(|err| JsValue::from(err.to_string()))
    }
}

#[wasm_bindgen]
impl WasmBinData {
    #[wasm_bindgen(getter)]
    pub fn get_private_key(&self) -> String {
        self.private_key.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }
}

// use aptos_sdk::crypto::{ed25519::Ed25519PrivateKey, ed25519::Ed25519PublicKey, Signature};
// use aptos_sdk::transaction_builder::TransactionBuilder;
//
// #[wasm_bindgen]
// pub struct AptosSigner {
//     private_key: Ed25519PrivateKey,
//     public_key: Ed25519PublicKey,
// }
//
// #[wasm_bindgen]
// impl AptosSigner {
//     #[wasm_bindgen(constructor)]
//     pub fn new(private_key_hex: &str) -> AptosSigner {
//         let private_key = Ed25519PrivateKey::from_hex(private_key_hex).expect("Invalid private key");
//         let public_key = private_key.public_key();
//         AptosSigner { private_key, public_key }
//     }
//
//     pub fn sign_transaction(&self, transaction_data: &[u8]) -> String {
//         let signature: Signature = self.private_key.sign(transaction_data);
//         signature.to_hex()
//     }
//
//     pub fn get_public_key(&self) -> String {
//         self.public_key.to_hex()
//     }
// }
