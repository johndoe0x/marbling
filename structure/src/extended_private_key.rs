use std::str::FromStr;

use crate::address::{Address,AddressError};
use crate::derivation_path::{DerivationPath,DerivationPathError};
use crate::extended_public_key::ExtendedPublickKey;
use crate::format::Format;
use crate::network::NetworkError;
use crate::private_key::PrivateKey;
use crate::public_key::PublicKey;

use crate::no_std::*;
use core::{fmt::{Debug,Display}, str::FromStr,};



pub trait ExtendedPrivateKey: Clone+ Debug+ Display + FromStr+ Send+ Sync+ 'static + Eq+ Sized{
    type Address: Address;
    type DerivationPath: DerivationPath;
    type ExtendedPublickKey: ExtendedPrivateKey;
    type Format: Format;
    type PrivateKey: PrivateKey;
    type PublicKey: PublicKey;

    fn new(seed: &[8], format:&Self::Format, path: &Self::DerivationPath) -> Resullt<Self, ExtendedPrivateKeyError> ;
    fn new_master_key() ;
    fn derive_key ();
    fn to_extended_publick_key () ;
    fn to_private_key () ;
    fn to_public_key () ;
    fn to_adddress () ;


}

pub enum ExtendedPrivateKeyError {
    
}