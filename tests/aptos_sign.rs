use std::str::FromStr;

use crate::aptos_txn::{setup_proto_transaction, OpsDetails, Transfer};
use amazing_wasm::aptos;
use tw_aptos::signer::Signer;
use tw_encoding::hex;
use tw_proto::Aptos::Proto::SigningOutput;

pub mod aptos_txn;

use aptos::create_signed_transaction;

// Successfully broadcasted https://explorer.aptoslabs.com/txn/0xb4d62afd3862116e060dd6ad9848ccb50c2bc177799819f1d29c059ae2042467?network=devnet
#[test]
fn test_aptos_sign_transaction_transfer() {
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
    let raw_txn_bytes = output.encoded.to_vec();
    let res = create_signed_transaction(raw_txn_bytes, "5d996aa76b3212142792d9130796cd2e11e3c445a93118c08414df4f66bc60ec".as_bytes());
    println!("{:?}", res);

    assert_eq!(
        hex::encode(res.signature, false),
        "5707246db31e2335edc4316a7a656a11691d1d1647f6e864d1ab12f43428aaaf806cf02120d0b608cdd89c5c904af7b137432aacdd60cc53f9fad7bd33578e01"
    );

    // test_tx_result(output,
    //                "07968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f3063000000000000000200000000000000000000000000000000000000000000000000000000000000010d6170746f735f6163636f756e74087472616e7366657200022007968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f3008e803000000000000fe4d3200000000006400000000000000c2276ada0000000021",
    //                "5707246db31e2335edc4316a7a656a11691d1d1647f6e864d1ab12f43428aaaf806cf02120d0b608cdd89c5c904af7b137432aacdd60cc53f9fad7bd33578e01",
    //                "07968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f3063000000000000000200000000000000000000000000000000000000000000000000000000000000010d6170746f735f6163636f756e74087472616e7366657200022007968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f3008e803000000000000fe4d3200000000006400000000000000c2276ada00000000210020ea526ba1710343d953461ff68641f1b7df5f23b9042ffa2d2a798d3adb3f3d6c405707246db31e2335edc4316a7a656a11691d1d1647f6e864d1ab12f43428aaaf806cf02120d0b608cdd89c5c904af7b137432aacdd60cc53f9fad7bd33578e01",
    //                r#"{
    //         "expiration_timestamp_secs": "3664390082",
    //         "gas_unit_price": "100",
    //         "max_gas_amount": "3296766",
    //         "payload": {
    //             "arguments": ["0x7968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f30","1000"],
    //             "function": "0x1::aptos_account::transfer",
    //             "type": "entry_function_payload",
    //             "type_arguments": []
    //         },
    //         "sender": "0x7968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f30",
    //         "sequence_number": "99",
    //         "signature": {
    //             "public_key": "0xea526ba1710343d953461ff68641f1b7df5f23b9042ffa2d2a798d3adb3f3d6c",
    //             "signature": "0x5707246db31e2335edc4316a7a656a11691d1d1647f6e864d1ab12f43428aaaf806cf02120d0b608cdd89c5c904af7b137432aacdd60cc53f9fad7bd33578e01",
    //             "type": "ed25519_signature"
    //         }
    //     }"#);
}

fn test_tx_result(
    output: SigningOutput,
    expected_raw_txn_bytes_str: &str,
    expected_signature_str: &str,
    expected_encoded_txn_str: &str,
    json_literal: &str,
) {
    assert!(output.error_message.is_empty());

    assert_eq!(
        hex::encode(output.raw_txn.to_vec(), false),
        expected_raw_txn_bytes_str
    );
    assert_eq!(
        hex::encode(output.authenticator.unwrap().signature.to_vec(), false),
        expected_signature_str
    );
    assert_eq!(
        hex::encode(output.encoded.to_vec(), false),
        expected_encoded_txn_str
    );

    let json_value_expected: serde_json::Value = serde_json::from_str(json_literal).unwrap();
    let json_value: serde_json::Value = serde_json::from_str(output.json.as_ref()).unwrap();
    assert_eq!(json_value, json_value_expected);
}
