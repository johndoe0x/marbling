use marbling_structure::no_std::*;
use marbling_structure::{AddressError, ExtendedPrivateKeyError, ExtendedPublickKeyError, Format};
use crate::network::BitcoinNetwork;
use core::fmt;
use serde::Serialize;

#[derive(Serialize,Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]

pub enum BitcoinAddressFormat {
    P2PKH,

    P2WSH,

    P2SH_P2WPKH,

    Bech32,

    P2TR
    // Need to add DKG in here??
}

impl Format for BitcoinAddressFormat{}

impl BitcoinAddressFormat {
    pub fn return_address_prefix_from_network< Network:BitcoinNetwork>(&self)-> Vec<u8> {
        Network::return_address_prefix_from_network(self);
    }

    pub fn return_addres_format_from_address_prefix( prefix: &[])
}

