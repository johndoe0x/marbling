use crate::address::{Address,AddressError};
use crate::derivation_path::{DerivationPath, DerivationPathError};
use crate::extended_private_key::ExtendedPrivateKey;
use crate::network::NetworkError;
use crate::public_key::{PublicKey, PublicKeyError};

use crate::no_std::*;
use core::{
    fmt::{Debug,Display},
    str::FromStr,
};
pub trait ExtendedPublickKey: Clone+ Debug+Display+FromStr+Send+Sync+'static+ Eq+ Sized {
    type Address: Address;
    type DerivationPath: DerivationPath;
    type ExtendedPrivateKey: ExtendedPrivateKey;
    type PublicKey: PublicKey;

    fn from_extended_private_key(extended_private_key: &Self::ExtendedPrivateKey) ->Self;

    fn derive_public_key(&self, path: &Self::DerivationPath) -> Result<Self, ExtendedPublickKeyError>;

    fn to_public_key(&self)-> Self::PublicKey;

    fn to_adddress(&self ) -> Result<Self::Address, AddressError>;
}

#[derive(Debug, Fail)]

pub enum ExtendedPublickKeyError {
    #[fail(display= "{}:{}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display="{}",_0)]
    DerivationPathError(DerivationPathError),

    #[fail(display="invalid bytes length:{}", _0)]
    InvalidByteLength(usize),

    #[fail(display="invalid extended private checksum :{{ expected: {:?}, got: {:?},}}", _0, _1)]
    InvalidChecksum(String, String),

    #[fail(display="invalid child number: {{ expected: {:?}, got: {:?},}}", _0, _1)]
    InvalidChildNumber(u32, u32),

    #[fail(display="invalid version : {:?}", _0)]
    InvalidVersioning(Vec<u8>),

    #[fail(display="maximum child depth reached: {}", _0)]
    MaximumChildDepthReached(u8),

    #[fail(display="{}", _0)]
    Message(String),

    #[fail(display="{}", _0)]
    NetworkError(NetworkError),

    #[fail(display="{}", _0)]
    PublicKeyError(PublicKeyError),

    #[fail(display="unsupported format: {}", _0)]
    UnsupportedFormat(String),
}

impl From<crate::no_std::io::Error> for ExtendedPublickKeyError {
    fn from(error: crate::no_std::io::Error) -> Self {
        ExtendedPublickKeyError::Crate("crate::no_std:io", format!("{:?}", error))
    }
}

impl From<DerivationPathError> for ExtendedPublickKeyError {
    fn from(error: DerivationPathError) -> Self {
        ExtendedPublickKeyError::DerivationPathError(error)
    }    
}

impl From<NetworkError> for ExtendedPublickKeyError {
    fn from(error: NetworkError) -> Self {
        ExtendedPublickKeyError::NetworkError(error)
    }
}

impl From<PublicKeyError> for ExtendedPublickKeyError {
    fn from(error: PublicKeyError) -> Self {
        ExtendedPublickKeyError::PublicKeyError(error)
    }
}

impl From<base58::FromBase58Error> for ExtendedPublickKeyError {
    fn from(error: base58::FromBase58Error) -> Self {
        ExtendedPublickKeyError::Crate("base58", format!("{:?}", error))
    }
}

impl From<bech32::Error> for ExtendedPublickKeyError {
    fn from(error: bech32::Error) -> Self {
        ExtendedPublickKeyError::Crate("bech32", format!("{:?}", error))
    }
}

impl From<core::array::TryFromSliceError> for ExtendedPublickKeyError {
    fn from(error: core::array::TryFromSliceError) -> Self {
        ExtendedPublickKeyError::Crate("core::array", format!("{:?}", error))
    }
}

impl From<core::num::ParseIntError> for ExtendedPublickKeyError {
    fn from(error: core::num::ParseIntError) -> Self {
        ExtendedPublickKeyError::Crate("core::num", format!("{:?}", error))
    }
}

impl From<crypto_mac::InvalidKeyLength> for ExtendedPublickKeyError {
    fn from(error: crypto_mac::InvalidKeyLength) -> Self {
        ExtendedPublickKeyError::Crate("crypto_mac", format!("{:?}", error))
    }
}

impl From<secp256k1::Error> for ExtendedPublickKeyError {
    fn from(error: secp256k1::Error) -> Self {
        ExtendedPublickKeyError::Crate("libsecp256k1", format!("{:?}", error))
    }
}
