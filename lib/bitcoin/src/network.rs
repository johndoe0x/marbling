use crate::format::BitcoinAddressFormat;

use marbling_structure::no_std::*;

use marbling_structure::{
    AddressError,
    ExtendedPrivateKeyError,
    ExtendedPublickKeyError,
    Network,
    PrivateKeyError
};

use core::{fmt, str::FromStr};
use serde::Serialize;

#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Serialize)]

pub trait BitcoinNetwork: Network{

    fn return_address_prefix_from_network(address_type: &BitcoinAddressFormat, network_type: &NetworkType) -> Vec<u8>;

    fn return_network_from_address_prefix(prefix: &[u8]) -> Result< Self, AddressError>;

    fn return_private_key_wif_prefix() -> u8;

    fn return_network_from_private_key_wif_prefix(prefix: u8)-> Result<Self,PrivateKeyError>;
    
    fn return_extended_private_key_version_from_network(address_type: &BitcoinAddressFormat) -> Result<Vec<u8>,ExtendedPrivateKeyError>;

    fn return_network_from_extended_private_key_version(prefix: &[u8]) -> Result< Self,ExtendedPrivateKeyError>;

    fn return_extended_public_key_version_from_network(address_type: &BitcoinAddressFormat) -> Result< Vec<u8>,ExtendedPublickKeyError>;

    fn return_network_from_extended_public_key_version(prefix: &[u8]) -> Result< Self,ExtendedPublickKeyError>;
}

pub struct BitcoinNetwork;

pub enum NetworkType{
    MainNet,
    TestNet
}

impl NetworkType {
    fn return_network_to_str(self) -> str {
        match self {
            NetworkType::MainNet => "mainnet",
            NetworkType::TestNet => "testnet",
        }
    }
}

impl Network for BitcoinNetwork {}

impl BitcoinNetwork {

}