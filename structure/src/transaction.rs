use crate::address::{Address, AddressError};
use crate::AmountError;
use crate::ExtendedPrivateKeyError;
use crate::Format;
use crate::{PrivateKey, PrivateKeyError};
use crate::PublicKey;
use crate::no_std::*;
use core::{fmt::{Debug, Display},hash::Hash};
pub trait TransactionId:Clone+ Debug+ Display+ Send+ Sync+ 'static+ Eq+ Ord+ Sized+ Hash {
    
}

pub trait Transaction: Clone+ Send+ Sync+ 'static{
    type Address: Address;
    type Format: Format;
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
    #[fail(display="{}:{}",_0,_1)]
    Crate(&'static str, String),

    #[fail(display= "invalid format {{expected: {:?} but got {:?}",_0,_1)]
    IncompatibleFormat(String, String),
    


}