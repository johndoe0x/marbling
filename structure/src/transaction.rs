use crate::address::{Address, AddressError};
use crate::AmountError;
use crate::ExtendedPrivateKeyError;
use crate::PrivateKey;
use crate::KeyError;
use crate::PublicKey;
use crate::no_std::*;
use core::{fmt::{Debug, Display},hash::Hash};
use rlp;
pub trait TransactionId:Clone+ Debug+ Display+ Send+ Sync+ 'static+ Eq+ Ord+ Sized+ Hash {
    
}

pub trait Transaction: Clone+ Send+ Sync+ 'static{
    type Address: Address;
    type PrivateKey: PrivateKey;
    type PublicKey: PublicKey;
    type TransactionId: TransactionId;
    type TransactionParams;

    fn return_unsigned_transaction( params: &Self::TransactionParams) -> Result<Self, TransactionError>;

    fn return_signed_transaction( &self, private_key: &Self::PrivateKey) -> Result<Self, TransactionError>;

    fn return_transaction_from_bytes(transaction: &Vec<u8>) -> Result<Self, TransactionError>;

    fn return_bytes_from_transaction(&self)-> Result<Vec<u8>, TransactionError>;

    fn return_transaction_id (&self)-> Result<Self::TransactionId, TransactionError>;
}

#[derive(Debug, Fail)]
pub enum TransactionError {
    #[fail(display="Global; {}",_0)]
    AddressError(AddressError),

    #[fail(display="Global; {}",_0)]
    AmountError(AmountError),

    #[fail(display="Global; {}", _0)]
    ExtendedPrivateKeyError(ExtendedPrivateKeyError),

    #[fail(display="{}:{}", _0, _1)]
    Crate(&'static str, String),   

    #[fail(display="ZCASH; invalid ephemeral key {}", _0)]
    InvalidEphemeralKey(String),

    #[fail(display = "{}",_0)]
    InvalidInputs(String),

    #[fail(display= "invalid ouput address: {}", _0)]
    InvalidOuputAddress(String),

    #[fail(display= "invalid output description: {}", _0)]
    InvalidOutputDescription(String),

    #[fail(display= "Ethereum; invalid RLP length: expected -9 , got - {}", _0)]
    InvalidRlpLength(usize),

    #[fail(display= "invalid script public key format : {}", _0)]
    InvalidScriptPublicKey(String),

    #[fail(display= "invalid segwit flag: {}", _0)]
    InvalidSegwitFlag(usize),

    #[fail(display= "invalid sepend information")]
    InvalidSpendInformation,

    #[fail(display= "invalid  transaction id : {:?}", _0)]
    InvalidTransactionId(usize),

    #[fail(display= "invalid integer size: {:?}", _0)]
    InvalidSizeInteger(usize),

    #[fail(display= "{}", _0)]
    Message(String),

    #[fail(display= "missing diversifier, check address is for the sapling.")]
    MissingDiversifierSapling,

    #[fail(display= "missing output address")]
    MissingOutputAddress,

    #[fail(display= "missing output amount")]
    MissingOutputAmount,

    #[fail(display= "missing output script public key")]
    MissingOutputScriptPubKey,

    #[fail(display= "missing output params")]
    MissingOutputParams,

    #[fail(display= "missing spend information")]
    MissingSpendInformation,

    #[fail(display= "missing spend params")]
    MissingSpendParams,

    #[fail(display= "Null : {:?}", _0)]
    NullException(()),

    #[fail(display= "{}", _0)]
    KeyError(KeyError),

    #[fail(display= "Joinsplits are not supported")]
    UnSupportedJoinsplits,

    #[fail(display= "Unsupported preimage operation on this address type: {}", _0)]
    UnsupportedPreimageOperation(String),

    #[fail(display="ZCASH; witness conflicting anchor")]
    ConflictingWitnessAnchor(),

    #[fail(display="ZCASH; decryption failed for enc_cyphertext: {}", _0)]
    FailedNoteDecrytion(String),

    #[fail(display="ZCASH; invalid binding signature for the transaction.")]
    InvalidBindingSignature()

}

impl From<crate::no_std::io::Error> for TransactionError {
    fn from(error: crate::no_std::io::Error) -> Self {
        TransactionError::Crate("crate::no_std::io", format!("{:?}", error))
    }
}
impl From< &'static str> for TransactionError {
    fn from(msg: &'static str) -> Self {
        TransactionError::Message(msg.into())
    }
}

impl From<()> for TransactionError {
    fn from(error: ()) -> Self{
        TransactionError::NullException(error)
    }
}

impl From<AddressError> for TransactionError {
    fn from(error: AddressError) -> Self {
        TransactionError::AddressError(error)
        }
}

impl From<ExtendedPrivateKeyError> for TransactionError {
    fn from(error: ExtendedPrivateKeyError) -> Self {
        TransactionError::ExtendedPrivateKeyError(error)
        }
}

impl From<KeyError> for TransactionError {  
    fn from(error: KeyError) -> Self {
        TransactionError::KeyError(error)
    }   
}

impl From<base58::FromBase58Error> for TransactionError{
    fn from(error: base58::FromBase58Error) -> Self{
        TransactionError::Crate("base58", format!("{:?}", error))
    }
}

impl From<bech32::Error> for TransactionError {
    fn from(error: bech32::Error) -> Self {
        TransactionError::Crate("bech32", format!("{:?}", error))
    }
}

impl From<core::num::ParseIntError> for TransactionError {
    fn from(error: core::num::ParseIntError) -> Self {
        TransactionError::Crate("core::num", format!("{:?}", error))
    }
}
impl From<core::str::ParseBoolError> for TransactionError {
    fn from(error: core::str::ParseBoolError) -> Self {
        TransactionError::Crate("core::str", format!("{:?}", error))
    }
}

#[cfg(feature="ff")]
impl From<ff::PrimeFieldDecodingError> for TransactionError {
    fn from(error: ff::PrimeFieldDecodingError) -> Self {
        TransactionError::Crate("ff", format!("{:?}", error))
    }
}

impl From<hex::FromHexError> for TransactionError{
    fn from(error: hex::FromHexError) -> Self {
        TransactionError::Crate("hex", format!("{:?}", error))
    }
}

impl From<rlp::DecoderError> for TransactionError{
    fn from(error: rlp::DecoderError) -> Self {
        TransactionError::Crate("rlp", format!("{:?}", error))
    }
}

impl From<secp256k1::Error> for TransactionError{
    fn from(error: secp256k1::Error) -> Self {
        TransactionError::Crate("secp256k1", format!("{:?}", error))
    }
}

impl From<serde_json::error::Error> for TransactionError{
    fn from(error: serde_json::error::Error) -> Self {
        TransactionError::Crate("serde_json", format!("{:?}", error))
    }
}

impl From<uint::FromDecStrErr> for TransactionError{
    fn from(error: uint::FromDecStrErr) -> Self {
        TransactionError::Crate("uint", format!("{:?}", error))
    }
}
