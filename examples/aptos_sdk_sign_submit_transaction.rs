use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::rest_client::AptosBaseUrl;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use aptos_sdk::{
    rest_client::Client,
    transaction_builder::TransactionFactory,
    types::{chain_id::ChainId, LocalAccount},
};
use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_sdk::types::account_address::AccountAddress;
use amazing_wasm::aptos::signer;

#[tokio::main]
async fn main() {
    main_().await;
}

/// aptos move run --function-id 'default::message::set_message' --args 'string:hello'
async fn main_() {
    let (ed_private_key, addr, private_key) = get_private_key_and_addr();

    // let client = Client::new(AptosBaseUrl::Devnet.to_url());
    // let (account, state) = client.get_account(addr).await.unwrap().into_parts();
    // let sequence_number = account.sequence_number;
    // let chain_id = ChainId::new(state.chain_id);
    //
    // // 1SUI=10.pow(9)MIST 1APTOS=10.pow(8)octas if max_gas/gas_price is too large get MAX_GAS_UNITS_EXCEEDS_MAX_GAS_UNITS_BOUND=13
    // let max_gas = 10u64.pow(6);
    // // let gas_unit_price = client.estimate_gas_price().await.unwrap().into_inner().gas_estimate;
    // let gas_unit_price = 100;
    //
    // // Sign and submit transaction
    // let transaction_factory = TransactionFactory::new(chain_id)
    //     .with_gas_unit_price(gas_unit_price)
    //     .with_max_gas_amount(max_gas);
    // // .with_transaction_expiration_time(self.gas_options.expiration_secs);
    // // https://explorer.aptoslabs.com/account/0xb411e3fd045765c73deca67f91be38131373dbf9eec0309068403558fe0bc202/modules/code/message?network=testnet
    // // module hello_blockchain::message {
    // let payload = TransactionPayload::EntryFunction(EntryFunction::new(
    //     // module_id::member_id
    //     ModuleId::new(addr, "message".parse().unwrap()),
    //     // if model_id+function_id wrong, get LINKER_ERROR
    //     "set_message".parse().unwrap(),
    //     vec![],
    //     vec![aptos_sdk::bcs::to_bytes("hello").unwrap()],
    // ));
    // let sender_account = &mut LocalAccount::new(addr, ed_private_key, sequence_number);
    // let transaction =
    //     sender_account.sign_with_transaction_builder(transaction_factory.payload(payload));
    //
    // println!("transaction={transaction:#?}");

    // authenticator: Ed25519 {
    //     public_key: Ed25519PublicKey(f89f9bc908655c3f613b251534b422ea4a9e7b8686a7f36e9be008a6ef52d293),
    //     signature: Ed25519Signature(1696ddae498d496fa5bd4e9df05c0e3b7d4fd2733284c646c938fbd1a29e2fbf43781420dfda25596e4ba320fea4c83638afd39eaa79c1f25abd10d8cc87cd0a),
    // },
    let  bcs_bytes: [u8; 159] = [181, 233, 125, 176, 127, 160, 189, 14, 85, 152, 170, 54, 67, 169, 188, 111, 102, 147, 189, 220, 26, 159, 236, 158, 103, 74, 70, 30, 170, 0, 177, 147, 217, 95, 153, 19, 67, 118, 167, 29, 143, 110, 61, 248, 72, 196, 178, 87, 197, 198, 132, 75, 14, 2, 184, 208, 214, 86, 194, 25, 188, 66, 152, 36, 5, 0, 0, 0, 0, 0, 0, 0, 2, 217, 95, 153, 19, 67, 118, 167, 29, 143, 110, 61, 248, 72, 196, 178, 87, 197, 198, 132, 75, 14, 2, 184, 208, 214, 86, 194, 25, 188, 66, 152, 36, 7, 109, 101, 115, 115, 97, 103, 101, 11, 115, 101, 116, 95, 109, 101, 115, 115, 97, 103, 101, 0, 1, 6, 5, 104, 101, 108, 108, 111, 64, 66, 15, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 150, 120, 27, 103, 0, 0, 0, 0, 155];

    let res = signer::Signer::new(private_key).sing_msg(bcs_bytes.to_vec());
    println!("res={res:#?}");

    // println!("transaction={transaction:#?}");
    // let response = client.submit_and_wait(&transaction).await.unwrap();
    // println!("response={response:#?}");
}


pub fn get_private_key_and_addr() -> (Ed25519PrivateKey, AccountAddress, String) {
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct Config {
        profiles: Profiles,
    }
    #[derive(Deserialize)]
    struct Profiles {
        default: DefaultProfile,
    }
    #[derive(Deserialize)]
    struct DefaultProfile {
        private_key: String,
        // rest_url: String
    }

    let home = std::env::var("HOME").unwrap();
    let yml_str = std::fs::read_to_string(format!("{home}/.aptos/config.yaml")).unwrap();
    let config: Config = serde_yml::from_str(&yml_str).unwrap();
    let private_key = config.profiles.default.private_key;

    let ed_pri_key = <Ed25519PrivateKey as aptos_sdk::crypto::ValidCryptoMaterialStringExt>::from_encoded_string(&private_key).unwrap();
    let public_key = aptos_sdk::crypto::SigningKey::verifying_key(&ed_pri_key);
    let address = aptos_sdk::types::transaction::authenticator::AuthenticationKey::ed25519(&public_key).account_address();
    let address = aptos_pubkey_to_addr(public_key.to_bytes());
    (ed_pri_key, address, private_key)
}

fn aptos_pubkey_to_addr(pubkey: [u8; 32]) -> AccountAddress {
    let mut pub_key = pubkey.to_vec();
    pub_key.push(0);
    use sha3::Digest;
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(pub_key);
    let digest = hasher.finalize();
    AccountAddress::from_bytes(digest).unwrap()
}

