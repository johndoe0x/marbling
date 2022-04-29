

use crate::key::{PublicKey,PrivateKey,KeyError};
use crate::no_std::*;

use crate::derivation_path::DerivationPathError;

use core::{
    fmt::{Debug,Display},
    hash::Hash,
    str::FromStr,
};

pub trait Address : 'static+ Clone+ Debug+ Display+ FromStr+ Hash+ PartialEq+ Eq+ Ord+ Send+ Sized+ Sync+ 'static+ Hash {
    type NetworkType;
    type PrivateKey: PrivateKey;
    type PublicKey: PublicKey;

    fn return_address_from_private_key(private_key: &Self::PrivateKey, network_type: &Self::NetworkType) -> Result<Self, AddressError>;
    fn return_address_from_public_key(public_key: &Self::PublicKey, network_type: &Self::NetworkType) -> Result<Self, AddressError>;
        
}
#[derive(Debug, Fail)]
pub enum AddressError {
    #[fail(display = "{} : {}", _0, _1)]
    Crate(&'static str, String),
    
    #[fail(display = "invalid format conversion from {:?} to {:?}", _0, _1)]
    IncompatibleFormat(String, String),

    #[fail(display = "Invalid address : {}", _0)]
    InvalidAddress(String),

    #[fail(display = "invalid byte length : {}", _0)]
    InvalidByteLength(String),

    #[fail(display = "invalid character length : {}", _0)]
    InvalidCharaterLength(usize),

    #[fail(display = "invalid Address Checksum : {{ expected: {:?}, got: {:?} }}", _0, _1)]
    InvalidChecksum(String, String),

    #[fail(display = "invalid network: {{ expected: {:?}, got: {:?} }}", _0, _1)]
    InvalidNetwork(String, String),

    #[fail(display = "invalid prefix: {:?}", _0)]
    InvalidPrefix(Vec<u8>),

    #[fail(display = "invalid address prefix length : {:?}", _0)]
    InvalidPrefixLength(usize),

    #[fail(display = "{}", _0)]
    Message(String),
    
    #[fail(display = " {}", _0)]
    PrivateKeyError(KeyError),

    #[fail(display = " {}", _0)]
    PublicKeyError(KeyError),

    #[fail(display = "{}", _0)]
    DerivationPathError(DerivationPathError),

    #[fail(display= "unsupported derive format : {}", _0)]
    UnspportedDerivationPath(String),
}

impl From<crate::no_std::io::Error> for AddressError {
    fn from(error: crate::no_std::io::Error) -> Self {
        AddressError::Crate("crate::no_std::io", format!("{:?}", error))
    }
}

impl From<crate::no_std::FromUtf8Error> for AddressError {
    fn from(error: crate::no_std::FromUtf8Error) -> Self {
        AddressError::Crate("crate::no_std", format!("{:?}", error))
    }    
}

impl From<&'static str> for AddressError {
    fn from(msg: &'static str) -> Self {
        AddressError::Message(msg.into())
    }
}

impl From<DerivationPathError> for AddressError {
    fn from(error: DerivationPathError) -> Self {
        AddressError::DerivationPathError(error)
    }
}