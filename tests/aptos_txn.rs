use move_core_types::language_storage::TypeTag;
use tw_aptos::liquid_staking::LiquidStakingOperation;
use tw_aptos::nft::NftOperation;
use tw_aptos::transaction_payload::convert_type_tag_to_struct_tag;
use tw_encoding::hex;
use tw_proto::Aptos::Proto;
use tw_proto::Aptos::Proto::SigningInput;

pub struct AccountCreation {
    to: String,
}

pub struct Transfer {
    pub(crate) to: String,
    pub(crate) amount: u64,
}

pub struct TokenTransfer {
    transfer: Transfer,
    tag: TypeTag,
}

pub struct RegisterToken {
    coin_type: TypeTag,
}
pub enum OpsDetails {
    RegisterToken(RegisterToken),
    LiquidStakingOps(LiquidStakingOperation),
    AccountCreation(AccountCreation),
    Transfer(Transfer),
    TokenTransfer(TokenTransfer),
    ImplicitTokenTransfer(TokenTransfer),
    NftOps(NftOperation),
}

pub fn setup_proto_transaction<'a>(
    sender: &'a str,
    keypair_str: &'a str,
    transaction_type: &'a str,
    sequence_number: i64,
    chain_id: u32,
    max_gas_amount: u64,
    timestamp: u64,
    gas_unit_price: u64,
    any_encoded: &'a str,
    ops_details: Option<OpsDetails>,
) -> SigningInput<'a> {
    let private = hex::decode(keypair_str).unwrap();

    let payload: Proto::mod_SigningInput::OneOftransaction_payload = match transaction_type {
        "transfer" => {
            if let OpsDetails::Transfer(transfer) = ops_details.unwrap() {
                Proto::mod_SigningInput::OneOftransaction_payload::transfer(
                    Proto::TransferMessage {
                        to: transfer.to.into(),
                        amount: transfer.amount,
                    },
                )
            } else {
                panic!("Unsupported arguments")
            }
        }
        "create_account" => {
            if let OpsDetails::AccountCreation(account) = ops_details.unwrap() {
                Proto::mod_SigningInput::OneOftransaction_payload::create_account(
                    Proto::CreateAccountMessage {
                        auth_key: account.to.into(),
                    },
                )
            } else {
                panic!("Unsupported arguments")
            }
        }
        "coin_transfer" => {
            if let OpsDetails::TokenTransfer(token_transfer) = ops_details.unwrap() {
                Proto::mod_SigningInput::OneOftransaction_payload::token_transfer(
                    Proto::TokenTransferMessage {
                        to: token_transfer.transfer.to.into(),
                        amount: token_transfer.transfer.amount,
                        function: Some(convert_type_tag_to_struct_tag(token_transfer.tag)),
                    },
                )
            } else {
                panic!("Unsupported arguments")
            }
        }
        "implicit_coin_transfer" => {
            if let OpsDetails::ImplicitTokenTransfer(token_transfer) = ops_details.unwrap() {
                Proto::mod_SigningInput::OneOftransaction_payload::token_transfer_coins(
                    Proto::TokenTransferCoinsMessage {
                        to: token_transfer.transfer.to.into(),
                        amount: token_transfer.transfer.amount,
                        function: Some(convert_type_tag_to_struct_tag(token_transfer.tag)),
                    },
                )
            } else {
                panic!("Unsupported arguments")
            }
        }
        "nft_ops" => {
            if let OpsDetails::NftOps(nft) = ops_details.unwrap() {
                Proto::mod_SigningInput::OneOftransaction_payload::nft_message(nft.into())
            } else {
                panic!("Unsupported arguments")
            }
        }
        "register_token" => {
            if let OpsDetails::RegisterToken(register_token) = ops_details.unwrap() {
                Proto::mod_SigningInput::OneOftransaction_payload::register_token(
                    Proto::ManagedTokensRegisterMessage {
                        function: Some(convert_type_tag_to_struct_tag(register_token.coin_type)),
                    },
                )
            } else {
                panic!("Unsupported arguments")
            }
        }
        "liquid_staking_ops" => {
            if let OpsDetails::LiquidStakingOps(liquid_staking_ops) = ops_details.unwrap() {
                Proto::mod_SigningInput::OneOftransaction_payload::liquid_staking_message(
                    liquid_staking_ops.into(),
                )
            } else {
                panic!("Unsupported arguments")
            }
        }
        "blind_sign_json" => Proto::mod_SigningInput::OneOftransaction_payload::None,
        _ => Proto::mod_SigningInput::OneOftransaction_payload::None,
    };

    let input = SigningInput {
        chain_id,
        sender: sender.into(),
        sequence_number,
        max_gas_amount,
        gas_unit_price,
        expiration_timestamp_secs: timestamp,
        private_key: private.into(),
        any_encoded: any_encoded.into(),
        transaction_payload: payload,
    };

    input
}
