use bech32::Error;

use crate::derivation_path::DerivationPathError;

use crate::no_std::*;
use core::{
    fmt::{Debug,Display},
    hash::Hash,
};

pub trait Format:Clone + Debug+ Display+ Send+ Sync+ 'static+ Eq+ Ord+ Sized+ Hash {
    
}
#[derive(Debug,Fail)]
pub enum  FormatError {
    #[fail(display= "{}: {}",_0,_1)]
    Crate(&'static str, String),

    #[fail(display = "{}", _0)]
    DerivationPathError(DerivationPathError),

    #[fail(display= "prefix wrong : {:?}", _0)]
    InvalidPrefix(Vec<u8>),

    #[fail(display= "unsupported deriva format : {}", _0)]
    UnspportedDerivationPath(String),

}

impl From<DerivationPathError> for FormatError {
    fn from(error: DerivationPathError) -> Self {
        FormatError::DerivationPathError(error)
    }
}