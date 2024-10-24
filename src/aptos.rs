use serde::{Deserialize, Serialize};
use sha3::Digest;
use ed25519_dalek::{Keypair, Signature, Signer};

#[derive(Serialize, Deserialize)]
struct RawTransaction {
    // 示例字段，实际字段根据需求修改
    to: String,
    amount: u64,
    // 其他字段...
}

pub struct AccountCreation {
    to: String,
}

pub struct Transfer {
    pub(crate) to: String,
    pub(crate) amount: u64,
}

pub struct TokenTransfer {
    transfer: Transfer,
    // tag: TypeTag,
}

pub struct RegisterToken {
    // coin_type: TypeTag,
}

pub enum OpsDetails {
    RegisterToken(RegisterToken),
    // LiquidStakingOps(LiquidStakingOperation),
    AccountCreation(AccountCreation),
    Transfer(Transfer),
    TokenTransfer(TokenTransfer),
    ImplicitTokenTransfer(TokenTransfer),
    // NftOps(NftOperation),
}

fn generate_signing_message(raw_transaction: &RawTransaction) -> Vec<u8> {
    let prefix_bytes = sha3::Sha3_256::digest(b"APTOS::RawTransaction");
    let bcs_bytes = bcs::to_bytes(raw_transaction).unwrap(); // 使用BCS序列化
    [prefix_bytes.as_slice(), &bcs_bytes].concat()
}


fn sign_transaction(signing_message: &[u8], private_key: &[u8]) -> Signature {
    let keypair = Keypair::from_bytes(private_key).expect("Invalid private key");
    keypair.sign(signing_message)
}

#[derive(Serialize, Deserialize)]
struct Authenticator {
    public_key: Vec<u8>,
    signature: Vec<u8>,
}

fn create_signed_transaction(
    raw_transaction: RawTransaction,
    private_key: &[u8],
) -> (RawTransaction, Authenticator) {
    let signing_message = generate_signing_message(&raw_transaction);
    let signature = sign_transaction(&signing_message, private_key);

    let public_key = Keypair::from_bytes(private_key)
        .expect("Invalid private key")
        .public
        .to_bytes()
        .to_vec();

    let authenticator = Authenticator {
        public_key,
        signature: signature.to_bytes().to_vec(),
    };

    (raw_transaction, authenticator)
}

// pub fn setup_proto_transaction<'a>(
//     sender: &'a str,
//     keypair_str: &'a str,
//     transaction_type: &'a str,
//     sequence_number: i64,
//     chain_id: u32,
//     max_gas_amount: u64,
//     timestamp: u64,
//     gas_unit_price: u64,
//     any_encoded: &'a str,
//     ops_details: Option<OpsDetails>,
// ) -> SigningInput<'a> {
//     let private = hex::decode(keypair_str).unwrap();
//
//     let payload: Proto::mod_SigningInput::OneOftransaction_payload = match transaction_type {
//         "transfer" => {
//             if let OpsDetails::Transfer(transfer) = ops_details.unwrap() {
//                 Proto::mod_SigningInput::OneOftransaction_payload::transfer(
//                     Proto::TransferMessage {
//                         to: transfer.to.into(),
//                         amount: transfer.amount,
//                     },
//                 )
//             } else {
//                 panic!("Unsupported arguments")
//             }
//         }
//         "create_account" => {
//             if let OpsDetails::AccountCreation(account) = ops_details.unwrap() {
//                 Proto::mod_SigningInput::OneOftransaction_payload::create_account(
//                     Proto::CreateAccountMessage {
//                         auth_key: account.to.into(),
//                     },
//                 )
//             } else {
//                 panic!("Unsupported arguments")
//             }
//         }
//         "coin_transfer" => {
//             if let OpsDetails::TokenTransfer(token_transfer) = ops_details.unwrap() {
//                 Proto::mod_SigningInput::OneOftransaction_payload::token_transfer(
//                     Proto::TokenTransferMessage {
//                         to: token_transfer.transfer.to.into(),
//                         amount: token_transfer.transfer.amount,
//                         function: Some(convert_type_tag_to_struct_tag(token_transfer.tag)),
//                     },
//                 )
//             } else {
//                 panic!("Unsupported arguments")
//             }
//         }
//         "implicit_coin_transfer" => {
//             if let OpsDetails::ImplicitTokenTransfer(token_transfer) = ops_details.unwrap() {
//                 Proto::mod_SigningInput::OneOftransaction_payload::token_transfer_coins(
//                     Proto::TokenTransferCoinsMessage {
//                         to: token_transfer.transfer.to.into(),
//                         amount: token_transfer.transfer.amount,
//                         function: Some(convert_type_tag_to_struct_tag(token_transfer.tag)),
//                     },
//                 )
//             } else {
//                 panic!("Unsupported arguments")
//             }
//         }
//         "nft_ops" => {
//             if let OpsDetails::NftOps(nft) = ops_details.unwrap() {
//                 Proto::mod_SigningInput::OneOftransaction_payload::nft_message(nft.into())
//             } else {
//                 panic!("Unsupported arguments")
//             }
//         }
//         "register_token" => {
//             if let OpsDetails::RegisterToken(register_token) = ops_details.unwrap() {
//                 Proto::mod_SigningInput::OneOftransaction_payload::register_token(
//                     Proto::ManagedTokensRegisterMessage {
//                         function: Some(convert_type_tag_to_struct_tag(
//                             register_token.coin_type,
//                         )),
//                     },
//                 )
//             } else {
//                 panic!("Unsupported arguments")
//             }
//         }
//         "liquid_staking_ops" => {
//             if let OpsDetails::LiquidStakingOps(liquid_staking_ops) = ops_details.unwrap() {
//                 Proto::mod_SigningInput::OneOftransaction_payload::liquid_staking_message(
//                     liquid_staking_ops.into(),
//                 )
//             } else {
//                 panic!("Unsupported arguments")
//             }
//         }
//         "blind_sign_json" => Proto::mod_SigningInput::OneOftransaction_payload::None,
//         _ => Proto::mod_SigningInput::OneOftransaction_payload::None,
//     };
//
//     let input = SigningInput {
//         chain_id,
//         sender: sender.into(),
//         sequence_number,
//         max_gas_amount,
//         gas_unit_price,
//         expiration_timestamp_secs: timestamp,
//         private_key: private.into(),
//         any_encoded: any_encoded.into(),
//         transaction_payload: payload,
//     };
//
//     input
// }

// // #[cfg(test)]
// mod tests {
//     use tw_aptos::signer::Signer;
//     use tw_encoding::hex;
//     use tw_proto::Aptos::Proto::SigningOutput;
//     use super::*;
//
//     fn test_tx_result(
//         output: SigningOutput,
//         expected_raw_txn_bytes_str: &str,
//         expected_signature_str: &str,
//         expected_encoded_txn_str: &str,
//         json_literal: &str,
//     ) {
//         assert!(output.error_message.is_empty());
//
//         assert_eq!(
//             hex::encode(output.raw_txn.to_vec(), false),
//             expected_raw_txn_bytes_str
//         );
//         assert_eq!(
//             hex::encode(output.authenticator.unwrap().signature.to_vec(), false),
//             expected_signature_str
//         );
//         assert_eq!(
//             hex::encode(output.encoded.to_vec(), false),
//             expected_encoded_txn_str
//         );
//
//         let json_value_expected: serde_json::Value = serde_json::from_str(json_literal).unwrap();
//         let json_value: serde_json::Value = serde_json::from_str(output.json.as_ref()).unwrap();
//         assert_eq!(json_value, json_value_expected);
//     }
//
//     // Successfully broadcasted https://explorer.aptoslabs.com/txn/0xb4d62afd3862116e060dd6ad9848ccb50c2bc177799819f1d29c059ae2042467?network=devnet
//     #[test]
//     fn test_aptos_sign_transaction_transfer() {
//         let input = setup_proto_transaction(
//             "0x07968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f30",
//             "5d996aa76b3212142792d9130796cd2e11e3c445a93118c08414df4f66bc60ec",
//             "transfer",
//             99,
//             33,
//             3296766,
//             3664390082,
//             100,
//             "",
//             Some(OpsDetails::Transfer(Transfer {
//                 to: "0x07968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f30"
//                     .to_string(),
//                 amount: 1000,
//             })),
//         );
//         let output = Signer::sign_proto(input);
//         test_tx_result(output,
//                        "07968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f3063000000000000000200000000000000000000000000000000000000000000000000000000000000010d6170746f735f6163636f756e74087472616e7366657200022007968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f3008e803000000000000fe4d3200000000006400000000000000c2276ada0000000021",
//                        "5707246db31e2335edc4316a7a656a11691d1d1647f6e864d1ab12f43428aaaf806cf02120d0b608cdd89c5c904af7b137432aacdd60cc53f9fad7bd33578e01",
//                        "07968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f3063000000000000000200000000000000000000000000000000000000000000000000000000000000010d6170746f735f6163636f756e74087472616e7366657200022007968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f3008e803000000000000fe4d3200000000006400000000000000c2276ada00000000210020ea526ba1710343d953461ff68641f1b7df5f23b9042ffa2d2a798d3adb3f3d6c405707246db31e2335edc4316a7a656a11691d1d1647f6e864d1ab12f43428aaaf806cf02120d0b608cdd89c5c904af7b137432aacdd60cc53f9fad7bd33578e01",
//                        r#"{
//             "expiration_timestamp_secs": "3664390082",
//             "gas_unit_price": "100",
//             "max_gas_amount": "3296766",
//             "payload": {
//                 "arguments": ["0x7968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f30","1000"],
//                 "function": "0x1::aptos_account::transfer",
//                 "type": "entry_function_payload",
//                 "type_arguments": []
//             },
//             "sender": "0x7968dab936c1bad187c60ce4082f307d030d780e91e694ae03aef16aba73f30",
//             "sequence_number": "99",
//             "signature": {
//                 "public_key": "0xea526ba1710343d953461ff68641f1b7df5f23b9042ffa2d2a798d3adb3f3d6c",
//                 "signature": "0x5707246db31e2335edc4316a7a656a11691d1d1647f6e864d1ab12f43428aaaf806cf02120d0b608cdd89c5c904af7b137432aacdd60cc53f9fad7bd33578e01",
//                 "type": "ed25519_signature"
//             }
//         }"#);
//     }
// }
