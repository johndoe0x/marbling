use crate::address::BitcoinAddressFormat;

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

    fn return_address_prefix_from_network(address_type: &BitcoinAddressFormat, network_type: &str) -> Vec<u8>;

    fn return_network_from_address_prefix(prefix: &[u8],network_type: &str) -> Result< Self, AddressError>;

    fn return_private_key_wif_prefix(network_type: &str) -> u8;

    fn return_network_from_private_key_wif_prefix(prefix: u8,network_type: &str)-> Result<Self,PrivateKeyError>;
    
    fn return_extended_private_key_version_from_network(address_type: &BitcoinAddressFormat,network_type: &str) -> Result<Vec<u8>,ExtendedPrivateKeyError>;

    fn return_network_from_extended_private_key_version(prefix: &[u8],network_type: &str) -> Result< Self,ExtendedPrivateKeyError>;

    fn return_extended_public_key_version_from_network(address_type: &BitcoinAddressFormat,network_type: &str) -> Result< Vec<u8>,ExtendedPublickKeyError>;

    fn return_network_from_extended_public_key_version(prefix: &[u8],network_type: &str) -> Result< Self,ExtendedPublickKeyError>;
}

pub struct BitcoinNetwork;

pub enum NetworkType{
    // mainnet
    MainNet,
    //Testnet
    TestNet,
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "{}",
            match self {
                NetworkType::MainNet => "mainnet",
                NetworkType::TestNet => "testnet",
            }
        )
    }
}

impl Network for BitcoinNetwork {}

impl BitcoinNetwork {

    fn return_address_prefix_from_network(address_format: &BitcoinAddressFormat, network: &str) ->Vec<u8> { 
        if network == "mainnet" {
            match address_format {
                BitcoinAddressFormat::P2PKH => vec![0x00],
                BitcoinAddressFormat::P2WSH => vec![0x00],
                BitcoinAddressFormat::P2SH_P2WPKH => todo!(),
                BitcoinAddressFormat::Bech32 => todo!(),
                BitcoinAddressFormat::P2TR => todo!(),
            }
        }else if network == "testnet" {
            match address_format {
                BitcoinAddressFormat::P2PKH => todo!(),
                BitcoinAddressFormat::P2WSH => todo!(),
                BitcoinAddressFormat::P2SH_P2WPKH => todo!(),
                BitcoinAddressFormat::Bech32 => todo!(),
                BitcoinAddressFormat::P2TR => todo!(),
            }
        }
    }

    fn return_network_from_address_prefix(prefix : &[u8], network_type: &str) -> Result<Self, AddressError> { 

    }


}