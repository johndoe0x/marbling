

use crate::address::{Address,AddressError};
use crate::derivation_path::{DerivationPath,DerivationPathError};
use crate::extended_public_key::ExtendedPublickKey;

use crate::network::NetworkError;
use crate::key::{PrivateKey,PublicKey};

use crate::no_std::*;
use core::{fmt::{Debug,Display}, str::FromStr,};



pub trait ExtendedPrivateKey: Clone+ Debug+ Display + FromStr+ Send+ Sync+ 'static + Eq+ Sized{
    type Address: Address;
    type DerivationPath: DerivationPath;
    type ExtendedPublickKey: ExtendedPublickKey;

    type PrivateKey: PrivateKey;
    type PublicKey: PublicKey;

    fn gen_master_key_from_path(seed: &[u8], path: &Self::DerivationPath) -> Result<Self, ExtendedPrivateKeyError> ;
    fn gen_master_key(seed: &[u8]) -> Result<Self, ExtendedPrivateKeyError> ;
    fn return_private_key_from_path (&self, path: &Self::DerivationPath) -> Result<Self, ExtendedPrivateKeyError>;
    fn return_extended_publick_key (&self) -> Self::ExtendedPublickKey;
    fn return_key (&self) -> Self::PrivateKey ;
    fn return_public_key (&self) -> Self::PublicKey ;
    fn return_adddress (&self) -> Result<Self::Address, AddressError> ;
}

#[derive(Debug,Fail)]
pub enum ExtendedPrivateKeyError {
    #[fail(display= "{}:{}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display= "{}",_0)]
    DerivationPathError(DerivationPathError),

    #[fail(display= "invalid byte length:{}", _0)]
    InvalidByteLength(usize),

    #[fail(display= "invalid extended private key checksum:{{ expected: {:?}, got: {:?} }}", _0,_1)]
    InvalidChecksum(String, String),

    #[fail(display= "invalid version : {:?}", _0)]
    InvalidVersioning(Vec<u8>),

    #[fail(display ="maximum child depth reached: {}", _0)]
    MaximumChildDepthReached(u8),

    #[fail(display = "{}", _0)]
    Message(String),

    #[fail(display = "{}", _0)]
    NetworkError(NetworkError),

    #[fail(display="unsupported format: {}", _0)]
    UnsupportedFormat(String),

}

impl From<crate::no_std::io::Error> for ExtendedPrivateKeyError {
    fn from(error: crate::no_std::io::Error) -> Self {
        ExtendedPrivateKeyError::Crate("crate::no_std::io", format!("{:?}",error))
    }
}

impl From<DerivationPathError> for ExtendedPrivateKeyError {
    fn from(error: DerivationPathError) -> Self {
        ExtendedPrivateKeyError::DerivationPathError(error)
    }
}
impl From<NetworkError> for ExtendedPrivateKeyError {
    fn from(error: NetworkError) -> Self {
        ExtendedPrivateKeyError::NetworkError(error)
    }
}

impl From<base58::FromBase58Error> for ExtendedPrivateKeyError {
    fn from(error: base58::FromBase58Error) -> Self {
        ExtendedPrivateKeyError::Crate("base58", format!("{:?}", error))
    }
}

impl From<bech32::Error> for ExtendedPrivateKeyError {
    fn from(error: bech32::Error) -> Self {
        ExtendedPrivateKeyError::Crate("bech32", format!("{:?}", error))
    }    
}

impl From<core::array::TryFromSliceError> for ExtendedPrivateKeyError {
    fn from(error: core::array::TryFromSliceError) -> Self {
        ExtendedPrivateKeyError::Crate("core::array", format!("{:?}",error))
    }
}

impl From<core::num::ParseIntError> for ExtendedPrivateKeyError {
    fn from(error: core::num::ParseIntError) -> Self {
        ExtendedPrivateKeyError::Crate("core::num", format!("{:?}", error))
    }
}

impl From<crypto_mac::InvalidKeyLength> for ExtendedPrivateKeyError {
    fn from(error: crypto_mac::InvalidKeyLength) -> Self {
        ExtendedPrivateKeyError::Crate("crypto_mac", format!("{:?}", error))
    }
}

impl From<secp256k1::Error> for ExtendedPrivateKeyError {
    fn from(error: secp256k1::Error) -> Self {
        ExtendedPrivateKeyError::Crate("libsecp256k1", format!("{:?}",error))
    }
}