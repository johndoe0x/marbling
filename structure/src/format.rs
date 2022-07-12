use crate::derivation_path::DerivationPathError;

use crate::no_std::*;
use core::{
    fmt::{Debug,Display},
    str::FromStr,
};

pub trait Format: Clone + Debug + Display + Send + 'static + Eq + Ord + Sized + Hash {}

#[derive(Debug, Fail)]
pub enum FormatError {
    #[fail(display = "{}:{}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display="{}",_0)]
    DerivationPath(DerivationPathError),

    #[fail(display= "invalid address prefix: {:?}", _0)]
    InvalidPrefix(Vec<u8>),

    #[fail(display= "invalid version bytes: {:?}", _0)]
    InvalidVersionBytes(Vec<u8>),

    #[fail(display= "unsupported derivation path for the format :{}", _0)]
    UnspportedDerivationPath(String),
}

impl From<DerivationPathError> for FormatError {
    fn from(err: DerivationPathError) -> Self {
        FormatError::DerivationPath(err)
    }
}

impl From<base58_monero::base58::Error> for FormatError {
    fn from(error: base58_monero::base58::Error) -> Self {
        FormatError::Crate("base58_monero", format!("{:?}", error))
    }
}