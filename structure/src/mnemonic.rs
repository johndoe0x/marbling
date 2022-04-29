


use crate::address::{Address,AddressError};
use crate::extended_private_key::{ExtendedPrivateKey,ExtendedPrivateKeyError};
use crate::extended_public_key::ExtendedPublickKey;

use crate::key:: {PrivateKey,PublicKey,KeyError};
use crate::wordlists::WordlistError;
use crate::no_std::*;
use core::{
    fmt::{Debug, Display},
    str::FromStr,
};


use rand::Rng;



pub trait Mnemonic:Clone+ Debug+ Display+ FromStr+ Send+ Sync+ 'static+ Eq+ Sized {
    type Address:Address;
    type PrivateKey: PrivateKey;
    type PublicKey:PublicKey;

    fn gen_new_mnemonic<R: Rng> (rng: &mut R) ->  Result<Self, MnemonicError>;
    fn return_mnemonic_from_phrase(phrase: &str) -> Result<Self, MnemonicError>;
    fn return_phase_from_mnemonic(&self) -> Result<String, MnemonicError>;
    fn return_private_key_from_mnemonic(&self, password: Option<&str>) -> Result<Self::PrivateKey, MnemonicError>;
    fn return_public_key_from_mnemonic (&self, password: Option<&str>) -> Result<Self::PublicKey, MnemonicError>;
    fn return_address_from_mnemonic( &self, password: Option<&str>) -> Result< Self::Address, MnemonicError>; 
}


pub trait MnemonicCount:Mnemonic {
    fn return_mnemonic_given_word_count<R:Rng>(rng: &mut R, word_count: u8) -> Result<Self,MnemonicError>;
}

pub trait MnemonicExtendedKeys: Mnemonic {
    type ExtendedPrivateKey: ExtendedPrivateKey;
    type ExtendedPublickKey: ExtendedPublickKey;
    
    fn return_extended_private_key_from_mnemonic( &self, password: Option<&str>) -> Result<Self::ExtendedPrivateKey, MnemonicError>;

    fn return_extended_publick_key_from_mnemonic( &self, password: Option<&str>) -> Result<Self::ExtendedPublickKey, MnemonicError>;
}

#[derive(Debug, Fail)]
pub enum MnemonicError {
    #[fail(display="{}", _0)]
    AddressError(AddressError),

    #[fail(display="{}: {}", _0, _1)]
    Crate(&'static str,String),

    #[fail(display="{}", _0)]
    ExtendedPrivateKeyError(ExtendedPrivateKeyError),

    #[fail(display="Invalid checksum word:{{ expected: {:?} but got: {:?}", _0,_1)]
    InvalidChecksumWord(String, String),

    #[fail(display="Invalid decoding from word to seed")]
    InvalidDecoding,

    #[fail(display="Invalid entropy Length: {}", _0)]
    InvalidEntropyLength(usize),

    #[fail(display="Invalid wordlist index: {}", _0)]
    InvalidIndex(usize),

    #[fail(display="Invalid phrase: {}", _0)]
    InvalidPhrase(String),

    #[fail(display="Invalid mnemonic word count: {}", _0)]
    InvalidWordCount(u8),

    #[fail(display="Missing the last checksum word")]
    MissingChecksumWord,

    #[fail(display="Missing words in mnemonic")]
    MissingWord,

    #[fail(display="{}", _0)]
    PrivateKeyError(KeyError),

    #[fail(display="{}",_0)]
    WordlistError(WordlistError),

}

impl From<crate::no_std::io::Error> for MnemonicError {
    fn from(error: crate::no_std::io::Error) -> Self {
        MnemonicError::Crate("crate::no_std::io", format!("{}", error))
    }
}

impl From<AddressError> for MnemonicError {
    fn from(error: AddressError) -> Self {
        MnemonicError::AddressError(error)
    }
}

impl From<ExtendedPrivateKeyError> for MnemonicError {
    fn from(error: ExtendedPrivateKeyError) -> Self {
        MnemonicError::ExtendedPrivateKeyError(error)
    }
}

impl From<KeyError> for MnemonicError {
    fn from(error: KeyError) -> Self {
        MnemonicError::PrivateKeyError(error)
    }
}

impl From<WordlistError> for MnemonicError {
    fn from(error: WordlistError) -> Self {
        MnemonicError::WordlistError(error)
    }
}

impl From<rand_core::Error> for MnemonicError {
    fn from(error: rand_core::Error) -> Self {
        MnemonicError::Crate("rand", format!("{:?}", error))
    }
}