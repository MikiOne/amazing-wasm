// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::bail;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
// use aptos_crypto::multi_ed25519::{MultiEd25519PublicKey, MultiEd25519Signature};
use crate::aptos::hash::HashValue;
use crate::aptos::traits::{CryptoMaterialError, Signature};
use crate::ed25519::{Ed25519PublicKey, Ed25519Signature};
use aptos_crypto_derive::{CryptoHasher, DeserializeKey, SerializeKey};
use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[repr(u8)]
pub enum Scheme {
    Ed25519 = 0,
    MultiEd25519 = 1,
    SingleKey = 2,
    MultiKey = 3,
    NoScheme = 250,
    /// Scheme identifier used to derive addresses (not the authentication key) of objects and
    /// resources accounts. This application serves to domain separate hashes. Without such
    /// separation, an adversary could create (and get a signer for) a these accounts
    /// when a their address matches matches an existing address of a MultiEd25519 wallet.
    /// Add new derived schemes below.
    DeriveAuid = 251,
    DeriveObjectAddressFromObject = 252,
    DeriveObjectAddressFromGuid = 253,
    DeriveObjectAddressFromSeed = 254,
    DeriveResourceAccountAddress = 255,
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
            Scheme::Ed25519 => "Ed25519",
            Scheme::MultiEd25519 => "MultiEd25519",
            Scheme::SingleKey => "SingleKey",
            Scheme::MultiKey => "MultiKey",
            Scheme::NoScheme => "NoScheme",
            Scheme::DeriveAuid => "DeriveAuid",
            Scheme::DeriveObjectAddressFromObject => "DeriveObjectAddressFromObject",
            Scheme::DeriveObjectAddressFromGuid => "DeriveObjectAddressFromGuid",
            Scheme::DeriveObjectAddressFromSeed => "DeriveObjectAddressFromSeed",
            Scheme::DeriveResourceAccountAddress => "DeriveResourceAccountAddress",
        };
        write!(f, "Scheme::{}", display)
    }
}

/// An `AccountAuthenticator` is an an abstraction of a signature scheme. It must know:
/// (1) How to check its signature against a message and public key
/// (2) How to convert its public key into an `AuthenticationKeyPreimage` structured as
/// (public_key | signature_scheme_id).
/// Each on-chain `Account` must store an `AuthenticationKey` (computed via a sha3 hash of `(public
/// key bytes | scheme as u8)`).
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AccountAuthenticator {
    /// Ed25519 Single signature
    Ed25519 {
        public_key: Ed25519PublicKey,
        signature: Ed25519Signature,
    },
}

impl AccountAuthenticator {
    /// Unique identifier for the signature scheme
    pub fn scheme(&self) -> Scheme {
        match self {
            Self::Ed25519 { .. } => Scheme::Ed25519,
        }
    }

    /// Create a single-signature ed25519 authenticator
    pub fn ed25519(public_key: Ed25519PublicKey, signature: Ed25519Signature) -> Self {
        Self::Ed25519 {
            public_key,
            signature,
        }
    }
    //
    // /// Create a multisignature ed25519 authenticator
    // pub fn multi_ed25519(
    //     public_key: MultiEd25519PublicKey,
    //     signature: MultiEd25519Signature,
    // ) -> Self {
    //     Self::MultiEd25519 {
    //         public_key,
    //         signature,
    //     }
    // }
    //
    // /// Create a single-signature authenticator
    // pub fn single_key(authenticator: SingleKeyAuthenticator) -> Self {
    //     Self::SingleKey { authenticator }
    // }
    //
    // /// Create a multi-signature authenticator
    // pub fn multi_key(authenticator: MultiKeyAuthenticator) -> Self {
    //     Self::MultiKey { authenticator }
    // }

    /// Return Ok if the authenticator's public key matches its signature, Err otherwise
    pub fn verify(&self, bcs_msg: Vec<u8>) -> anyhow::Result<()> {
        match self {
            Self::Ed25519 {
                public_key,
                signature,
            } => signature.verify(bcs_msg, public_key),
        }
    }

    /// Return the raw bytes of `self.public_key`
    pub fn public_key_bytes(&self) -> Vec<u8> {
        match self {
            Self::Ed25519 { public_key, .. } => public_key.to_bytes().to_vec(),
            // Self::MultiEd25519 { public_key, .. } => public_key.to_bytes().to_vec(),
            // Self::SingleKey { authenticator } => authenticator.public_key_bytes(),
            // Self::MultiKey { authenticator } => authenticator.public_key_bytes(),
            // Self::NoAccountAuthenticator => vec![],
        }
    }

    /// Return the raw bytes of `self.signature`
    pub fn signature_bytes(&self) -> Vec<u8> {
        match self {
            Self::Ed25519 { signature, .. } => signature.to_bytes().to_vec(),
            // Self::MultiEd25519 { signature, .. } => signature.to_bytes().to_vec(),
            // Self::SingleKey { authenticator } => authenticator.signature_bytes(),
            // Self::MultiKey { authenticator } => authenticator.signature_bytes(),
            // Self::NoAccountAuthenticator => vec![],
        }
    }

    /// Return an authentication key derived from `self`'s public key and scheme id
    // pub fn authentication_key(&self) -> Option<AuthenticationKey> {
    //     if let Self::NoAccountAuthenticator = self {
    //         None
    //     } else {
    //         Some(AuthenticationKey::from_preimage(
    //             self.public_key_bytes(),
    //             self.scheme(),
    //         ))
    //     }
    // }

    /// Return the number of signatures included in this account authenticator.
    pub fn number_of_signatures(&self) -> usize {
        match self {
            Self::Ed25519 { .. } => 1,
            // Self::MultiEd25519 { signature, .. } => signature.signatures().len(),
            // Self::SingleKey { .. } => 1,
            // Self::MultiKey { authenticator } => authenticator.signatures.len(),
            // Self::NoAccountAuthenticator => 0,
        }
    }
}

/// A struct that represents an account authentication key. An account's address is the last 32
/// bytes of authentication key used to create it
#[derive(
    Clone,
    Copy,
    // CryptoHasher,
    Debug,
    // DeserializeKey,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    // SerializeKey,
)]
// #[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub struct AuthenticationKey([u8; AuthenticationKey::LENGTH]);

impl AuthenticationKey {
    /// The number of bytes in an authentication key.
    pub const LENGTH: usize = AccountAddress::LENGTH;

    /// Create an authentication key from `bytes`
    pub const fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes)
    }

    /// Return an authentication key that is impossible (in expectation) to sign for--useful for
    /// intentionally relinquishing control of an account.
    pub const fn zero() -> Self {
        Self([0; 32])
    }

    /// Create an authentication key from a preimage by taking its sha3 hash
    pub fn from_preimage(mut public_key_bytes: Vec<u8>, scheme: Scheme) -> AuthenticationKey {
        public_key_bytes.push(scheme as u8);
        AuthenticationKey::new(*HashValue::sha3_256_of(&public_key_bytes).as_ref())
    }

    /// Construct a preimage from a transaction-derived AUID as (txn_hash || auid_scheme_id)
    // pub fn auid(mut txn_hash: Vec<u8>, auid_counter: u64) -> Self {
    //     txn_hash.extend(auid_counter.to_le_bytes().to_vec());
    //     Self::from_preimage(txn_hash, Scheme::DeriveAuid)
    // }

    // pub fn object_address_from_object(
    //     source: &AccountAddress,
    //     derive_from: &AccountAddress,
    // ) -> AuthenticationKey {
    //     let mut bytes = source.to_vec();
    //     bytes.append(&mut derive_from.to_vec());
    //     Self::from_preimage(bytes, Scheme::DeriveObjectAddressFromObject)
    // }

    /// Create an authentication key from an Ed25519 public key
    pub fn ed25519(public_key: &Ed25519PublicKey) -> AuthenticationKey {
        Self::from_preimage(public_key.to_bytes().to_vec(), Scheme::Ed25519)
    }

    /// Create an authentication key from a MultiEd25519 public key
    // pub fn multi_ed25519(public_key: &MultiEd25519PublicKey) -> Self {
    //     Self::from_preimage(public_key.to_bytes(), Scheme::MultiEd25519)
    // }

    /// Create an authentication key from an AnyPublicKey
    // pub fn any_key(public_key: AnyPublicKey) -> AuthenticationKey {
    //     Self::from_preimage(public_key.to_bytes(), Scheme::SingleKey)
    // }

    /// Create an authentication key from multiple AnyPublicKeys
    // pub fn multi_key(public_keys: MultiKey) -> AuthenticationKey {
    //     Self::from_preimage(public_keys.to_bytes(), Scheme::MultiKey)
    // }

    /// Return the authentication key as an account address
    pub fn account_address(&self) -> AccountAddress {
        AccountAddress::new(self.0)
    }

    /// Construct a vector from this authentication key
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    // /// Create a random authentication key. For testing only
    // pub fn random() -> Self {
    //     let mut rng = OsRng;
    //     let buf: [u8; Self::LENGTH] = rng.gen();
    //     AuthenticationKey::new(buf)
    // }
}

// impl ValidCryptoMaterial for AuthenticationKey {
//     fn to_bytes(&self) -> Vec<u8> {
//         self.to_vec()
//     }
// }

impl fmt::Display for AccountAuthenticator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AccountAuthenticator[scheme id: {:?}, public key: {}, signature: {}]",
            self.scheme(),
            hex::encode(self.public_key_bytes()),
            hex::encode(self.signature_bytes())
        )
    }
}

impl TryFrom<&[u8]> for AuthenticationKey {
    type Error = CryptoMaterialError;

    fn try_from(bytes: &[u8]) -> Result<AuthenticationKey, CryptoMaterialError> {
        if bytes.len() != Self::LENGTH {
            return Err(CryptoMaterialError::WrongLengthError);
        }
        let mut addr = [0u8; Self::LENGTH];
        addr.copy_from_slice(bytes);
        Ok(AuthenticationKey(addr))
    }
}

impl TryFrom<Vec<u8>> for AuthenticationKey {
    type Error = CryptoMaterialError;

    fn try_from(bytes: Vec<u8>) -> Result<AuthenticationKey, CryptoMaterialError> {
        AuthenticationKey::try_from(&bytes[..])
    }
}

// impl FromStr for AuthenticationKey {
//     type Err = Error;
//
//     fn from_str(s: &str) -> Result<Self> {
//         ensure!(
//             !s.is_empty(),
//             "authentication key string should not be empty.",
//         );
//         let bytes_out = ::hex::decode(s)?;
//         let key = AuthenticationKey::try_from(bytes_out.as_slice())?;
//         Ok(key)
//     }
// }

impl AsRef<[u8]> for AuthenticationKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::LowerHex for AuthenticationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl fmt::Display for AuthenticationKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        // Forward to the LowerHex impl with a "0x" prepended (the # flag).
        write!(f, "{:#x}", self)
    }
}

// #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
// pub struct MultiKey {
//     public_keys: Vec<AnyPublicKey>,
//     signatures_required: u8,
// }
//
// impl From<MultiEd25519PublicKey> for MultiKey {
//     fn from(multi_ed25519_public_key: MultiEd25519PublicKey) -> Self {
//         let public_keys: Vec<AnyPublicKey> = multi_ed25519_public_key
//             .public_keys()
//             .iter()
//             .map(|key| AnyPublicKey::ed25519(key.clone()))
//             .collect();
//         let signatures_required = *multi_ed25519_public_key.threshold();
//         MultiKey {
//             public_keys,
//             signatures_required,
//         }
//     }
// }

// impl MultiKey {
//     pub fn new(public_keys: Vec<AnyPublicKey>, signatures_required: u8) -> Result<Self> {
//         ensure!(
//             signatures_required > 0,
//             "The number of required signatures is 0."
//         );
//
//         ensure!(
//             public_keys.len() >= signatures_required as usize,
//             "The number of public keys is smaller than the number of required signatures, {} < {}",
//             public_keys.len(),
//             signatures_required
//         );
//
//         Ok(Self {
//             public_keys,
//             signatures_required,
//         })
//     }
//
//     pub fn is_empty(&self) -> bool {
//         self.public_keys.is_empty()
//     }
//
//     pub fn len(&self) -> usize {
//         self.public_keys.len()
//     }
//
//     pub fn public_keys(&self) -> &[AnyPublicKey] {
//         &self.public_keys
//     }
//
//     pub fn signatures_required(&self) -> u8 {
//         self.signatures_required
//     }
//
//     pub fn to_bytes(&self) -> Vec<u8> {
//         bcs::to_bytes(&self).expect("Only unhandleable errors happen here.")
//     }
// }

// #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
// pub enum AnyPublicKey {
//     Ed25519 {
//         public_key: Ed25519PublicKey,
//     },
//     Secp256k1Ecdsa {
//         public_key: secp256k1_ecdsa::PublicKey,
//     },
//     Secp256r1Ecdsa {
//         public_key: secp256r1_ecdsa::PublicKey,
//     },
//     // Keyless {
//     //     public_key: KeylessPublicKey,
//     // },
//     // FederatedKeyless {
//     //     public_key: FederatedKeylessPublicKey,
//     // },
// }
//
// impl AnyPublicKey {
//     pub fn ed25519(public_key: Ed25519PublicKey) -> Self {
//         Self::Ed25519 { public_key }
//     }
//
//     pub fn secp256k1_ecdsa(public_key: secp256k1_ecdsa::PublicKey) -> Self {
//         Self::Secp256k1Ecdsa { public_key }
//     }
//
//     pub fn secp256r1_ecdsa(public_key: secp256r1_ecdsa::PublicKey) -> Self {
//         Self::Secp256r1Ecdsa { public_key }
//     }
//
//     // pub fn keyless(public_key: KeylessPublicKey) -> Self {
//     //     Self::Keyless { public_key }
//     // }
//     //
//     // pub fn federated_keyless(public_key: FederatedKeylessPublicKey) -> Self {
//     //     Self::FederatedKeyless { public_key }
//     // }
//
//     pub fn to_bytes(&self) -> Vec<u8> {
//         bcs::to_bytes(self).expect("Only unhandleable errors happen here.")
//     }
// }
//
// impl TryFrom<&[u8]> for AnyPublicKey {
//     type Error = CryptoMaterialError;
//
//     fn try_from(bytes: &[u8]) -> Result<Self, CryptoMaterialError> {
//         bcs::from_bytes::<AnyPublicKey>(bytes)
//             .map_err(|_e| CryptoMaterialError::DeserializationError)
//     }
// }
