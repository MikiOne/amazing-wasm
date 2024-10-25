use move_core_types::account_address::AccountAddress;

use crate::aptos::authenticator::AuthenticationKey;
use crate::aptos::traits::ValidCryptoMaterialStringExt;
use crate::ed25519::{Ed25519PrivateKey, Ed25519PublicKey};
use crate::traits::SigningKey;

pub struct Signer {
    ed_pri_key: Ed25519PrivateKey,
    ed_pub_key: Ed25519PublicKey,
    acct_addr: AccountAddress,
}

impl Signer {
    pub fn new(pri_key: impl Into<String>) -> Self {
        let pri_key = pri_key.into();
        let private_key = pri_key.as_str();
        let ed_pri_key: Ed25519PrivateKey =
            ValidCryptoMaterialStringExt::from_encoded_string(private_key).unwrap();
        let ed_pub_key = SigningKey::verifying_key(&ed_pri_key);
        let acct_addr = AuthenticationKey::ed25519(&ed_pub_key).account_address();
        Self {
            ed_pri_key,
            ed_pub_key,
            acct_addr,
        }
    }

    pub fn sing_msg(&self, bcs_msg: Vec<u8>) {
        let ss = self.ed_pri_key.sign_arbitrary_message(bcs_msg.as_ref());
        println!("{:?}", ss);
    }
}
