use crate::address::{Address,AddressError};


use crate::no_std::*;
use core::{fmt::{Debug,Display},str::FromStr,};
use rand::Rng;

pub trait Key : Clone + Debug + Display + FromStr + Send + Sync + 'static + Eq + Sized {
    type Address: Address;

    
}
#[derive(Debug, Fail)]
pub enum KeyError {
    #[fail(display= "{}: {}",_0, _1)]
    Crate(&'static str, String),

    #[fail(display= "Invalid key byte length: {}", _0)]
    InvalidByteLength(usize),

    #[fail(display= "Invalid key char length: {}", _0)]
    InvalidCharaterLength(usize),

    #[fail(display="Invalid key checksum : {{ expected : {:?}, found: {:?} }}", _0, _1)]
    InvalidKeyChecksum(String,String),

    #[fail(display="Invalid key prefix : {:?}", _0)]
    InvalidKeyPrefix(Vec<u8>),

    #[fail(display="Wrong network :{{ expected: {:?}, found: {:?} }} ", _0, _1)]
    InvalidNetwork(String,String),

    #[fail(display="{}", _0)]
    Message(String),

    #[fail(display="Unsupported Key format")]
    UnsupportedFormat,
}

impl From<crate::no_std::io::Error> for KeyError{
    fn from(error: crate::no_std::io::Error) -> Self {
        KeyError::Crate("crate::no_std::io", format!("{:?}", error))

    }
}

impl From<base58::FromBase58Error> for KeyError{
    fn from(error: base58::FromBase58Error) -> Self{
        KeyError::Crate("base58", format!("{:?}", error))
    }
}
impl From<bech32::Error> for KeyError {
    fn from(error: bech32::Error) -> Self {
        KeyError::Crate("bech32", format!("{:?}", error))
    }
}

impl From<secp256k1::Error> for KeyError{
    fn from(error: secp256k1::Error) -> Self{
        KeyError::Crate("libsecp256k1", format!("{:?}", error))
    }
}

impl From<hex::FromHexError> for KeyError {
    fn from(error: hex::FromHexError) -> Self{
        KeyError::Crate("hex", format!("{:?}", error))
    }
}

impl From<&'static str> for KeyError {
    fn from(msg: &'static str) -> Self {
        KeyError::Message(msg.into())
    }
}

impl From<rand_core::Error> for KeyError {
    fn from(error: rand_core::Error) -> Self {
        KeyError::Crate("rand", format!("{:?}", error))
    }
}
pub trait PublicKey:Key {
    type PrivateKey: PrivateKey;
    /// Returns the address corresponding to the given public key.
    fn return_public_key(private_key: &Self::PrivateKey) -> Self;

    fn return_address(&self) -> Result<Self::Address, AddressError>;
}

pub trait PrivateKey:Key {
    type PublicKey:PublicKey;
    /// Return radom generated new private key
    fn new_key<R: Rng>(rng: &mut R) -> Result<Self,KeyError>;

    /// Return public key from given key
    fn return_public_key(&self) -> Self::PublicKey;

    fn return_address(&self) -> Result<Self::Address, AddressError>;
   
}


