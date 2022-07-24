use crate::address::{Address, AddressError};
use crate::amount::AmountError;
use crate::extended_private_key::ExtendedPrivateKeyError;
use crate::format::Format;
use crate::private_key::{PrivateKey, PrivateKeyError};
use crate::public_key::PublicKey;
use crate::no_std::*;
use core::{fmt::{Debug, Display},hash::Hash};
use rlp;
pub trait TransactionId:Clone+ Debug+ Display+ Send+ Sync+ 'static+ Eq+ Ord+ Sized+ Hash {
    
}

pub trait Transaction: Clone+ Send+ Sync+ 'static{
    type Address: Address;
    type Format: Format;
    type PrivateKey: PrivateKey;
    type PublicKey: PublicKey;
    type TransactionId: TransactionId;
    type TransactionParams;

    fn new( params: &Self::TransactionParams) -> Result<Self, TransactionError>;

    fn sign( &self, private_key: &Self::PrivateKey) -> Result<Self, TransactionError>;

    fn from_transaction_bytes(transaction: &Vec<u8>) -> Result<Self, TransactionError>;

    fn to_transaction_bytes(&self)-> Result<Vec<u8>, TransactionError>;

    fn return_transaction_id (&self)-> Result<Self::TransactionId, TransactionError>;
}

#[derive(Debug, Fail)]
pub enum TransactionError {
    #[fail(display="Global; {}",_0)]
    AddressError(AddressError),

    #[fail(display="Global; {}",_0)]
    AmountError(AmountError),

    #[fail(display=" witness have a conflicting anchor")]
    ConflictingWitnessAnchor(),

    #[fail(display="{}:{}", _0, _1)]
    Crate(&'static str, String),   

    #[fail(display="Global; {}", _0)]
    ExtendedPrivateKeyError(ExtendedPrivateKeyError),



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
    NullError(()),

    #[fail(display= "{}", _0)]
    PrivateKeyError(PrivateKeyError),

    #[fail(display= "Joinsplits are not supported")]
    UnSupportedJoinsplits,

    #[fail(display= "Unsupported preimage operation on this address type: {}", _0)]
    UnsupportedPreimageOperation(String),

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
        TransactionError::NullError(error)
    }
}

impl From<AddressError> for TransactionError {
    fn from(error: AddressError) -> Self {
        TransactionError::AddressError(error)
        }
}

impl From<AmountError> for TransactionError {
    fn from(error: AmountError) -> Self {
        TransactionError::AmountError(error)
        }
}

impl From<ExtendedPrivateKeyError> for TransactionError {
    fn from(error: ExtendedPrivateKeyError) -> Self {
        TransactionError::ExtendedPrivateKeyError(error)
        }
}

impl From<PrivateKeyError> for TransactionError {  
    fn from(error: PrivateKeyError) -> Self {
        TransactionError::PrivateKeyError(error)
    }   
}

impl From<base58::FromBase58Error> for TransactionError{
    fn from(error: base58::FromBase58Error) -> Self{
        TransactionError::Crate("base58", format!("{:?}", error))
    }
}

impl From<base58_monero::base58::Error> for TransactionError{
    fn from(error: base58_monero::base58::Error) -> Self{
        TransactionError::Crate("base58_monero", format!("{:?}", error))
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
