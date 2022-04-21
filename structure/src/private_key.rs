use crate::address::{Address,AddressError};
use crate::format::Format;
use crate::public_key::PublicKey;

use crate::no_std::*;
use core::{fmt::{Debug,Display},str::FromStr,};


use rand::Rng;

pub trait PrivateKey: Clone+ Debug+ Display+FromStr+ Send+ Sync+'static+ Eq+ Sized {
    type Address:Address;
    type Format:Format;
    type PublicKey:PublicKey;

    fn new<R: Rng>(rng: &mut R) -> Result<Self,PrivateKeyError>;
    fn to_public_key(&self) -> Self::PublicKey;
    fn to_addess(&self, format: &Self::Format) -> Result<Self::Address, AddressError>;
}

#[derive(Debug, Fail)]

pub enum PrivateKeyError {
    #[fail(display="{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display="wrong byte length: {}", _0 )]
    InvalidByteLength(usize),

    #[fail(display="Invalid char length: {}", _0)]
    InvalidCharaterLength(usize),

    #[fail(display="Invalid private key checksum : {{ expected : {:?}, found: {:?} }}", _0, _1)]
    InvalidChecksum(String,String),

    #[fail(display="Invalid private key prefix : {:?}", _0)]
    InvalidPrefix(Vec<u8>),

    #[fail(display="Wrong network :{{ expected: {:?}, found: {:?} }} ", _0, _1)]
    InvalidNetwork(String,String),

    #[fail(display="{}", _0)]
    Message(String),

    #[fail(display="Unsupported format")]
    UnsupportedFormat,
}

impl From<crate::no_std::io::Error> for PrivateKeyError {
    fn from(error: crate::no_std::io::Error) -> Self {
        PrivateKeyError::Crate("crate::no_std::io", format!("{:?}", error))
    }
}

impl From<&'static str> for PrivateKeyError {
    fn from(msg: &'static str) -> Self {
        PrivateKeyError::Message(msg.into())
    }
}

impl From<base58::FromBase58Error> for PrivateKeyError {
    fn from(error: base58::FromBase58Error) -> Self {
        PrivateKeyError::Crate("base58", format!("{:?}", error))
    }
}

impl From<rand_core::Error> for PrivateKeyError {
    fn from(error: rand_core::Error) -> Self {
        PrivateKeyError::Crate("rand", format!("{:?}", error))
    }
}

impl From<secp256k1::Error> for PrivateKeyError {
    fn from(error: secp256k1::Error) -> Self {
        PrivateKeyError::Crate("libsecp256k1", format!("{:?}", error))
    }
}

