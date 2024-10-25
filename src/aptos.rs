use aptos_crypto::{SigningKey, ValidCryptoMaterialStringExt};
use aptos_crypto::ed25519::Ed25519PrivateKey;
// use ed25519_dalek::{Keypair, Signature, Signer};
use serde::{Deserialize, Serialize};
// use sha3::Digest;
use crate::authenticator::AuthenticationKey;


pub fn get_private_key_addr(private_key: &str) {
    let private_key: Ed25519PrivateKey = ValidCryptoMaterialStringExt::from_encoded_string(private_key).unwrap();
    let public_key = SigningKey::verifying_key(&private_key);
    let address = AuthenticationKey::ed25519(&public_key).account_address();
    println!("address: {}", address)
}


// #[derive(Debug, Serialize, Deserialize)]
// pub struct Authenticator {
//     pub public_key: Vec<u8>,
//     pub signature: Vec<u8>,
// }
//
// fn gen_signing_message(txn_bcs_bytes: Vec<u8>) -> Vec<u8> {
//     let prefix_bytes = sha3::Sha3_256::digest(b"APTOS::RawTransaction");
//     // let bcs_bytes = bcs::to_bytes(raw_transaction).unwrap(); // 使用BCS序列化
//     [prefix_bytes.as_slice(), &txn_bcs_bytes].concat()
// }
//
// fn sign_transaction(signing_message: &[u8], private_key: &[u8]) -> Signature {
//     let keypair = Keypair::from_bytes(private_key).expect("Invalid private key");
//     keypair.sign(signing_message)
// }
//
// pub fn create_signed_transaction(txn_bcs_bytes: Vec<u8>, private_key: &[u8]) -> Authenticator {
//     // let txn_bcs_bytes = gen_signing_message(txn_bcs_bytes);
//     let signature = sign_transaction(&txn_bcs_bytes, private_key);
//
//     let public_key = Keypair::from_bytes(private_key)
//         .expect("Invalid private key")
//         .public
//         .to_bytes()
//         .to_vec();
//
//     let authenticator = Authenticator {
//         public_key,
//         signature: signature.to_bytes().to_vec(),
//     };
//     authenticator
// }

// #[derive(Serialize, Deserialize)]
// struct RawTransaction {
//     // 示例字段，实际字段根据需求修改
//     to: String,
//     amount: u64,
//     // 其他字段...
// }
// pub struct RawTransaction {
//     /// Sender's address.
//     sender: AccountAddress,
//
//     /// Sequence number of this transaction. This must match the sequence number
//     /// stored in the sender's account at the time the transaction executes.
//     sequence_number: u64,
//
//     /// The transaction payload, e.g., a script to execute.
//     payload: TransactionPayload,
//
//     /// Maximal total gas to spend for this transaction.
//     max_gas_amount: u64,
//
//     /// Price to be paid per gas unit.
//     gas_unit_price: u64,
//
//     /// Expiration timestamp for this transaction, represented
//     /// as seconds from the Unix Epoch. If the current blockchain timestamp
//     /// is greater than or equal to this time, then the transaction has
//     /// expired and will be discarded. This can be set to a large value far
//     /// in the future to indicate that a transaction does not expire.
//     expiration_timestamp_secs: u64,
//
//     /// Chain ID of the Aptos network this transaction is intended for.
//     chain_id: u8,
// }
