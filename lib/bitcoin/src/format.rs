use crate::network::BitcoinNetwork;
use marbling_structure::no_std::*;
use marbling_structure::{ AddressError, ExtendedPrivateKeyError, ExtendedPublickKeyError, Format};

use core::fmt;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]

pub enum BitcoinFormat {
    P2PKH, 
    P2WSH, 
    P2SH_P2WPKH,
    Bech32,
}

impl Format for BitcoinFormat {}

impl BitcoinFormat {
    pub fn to_address_prefix<N: BitcoinNetwork>(&self) -> Vec<u8> {
        N::to_address_prefix(self)
    }

    pub fn from_address_prefix(prefix: &[u8]) -> Result<Self, AddressError> {
        if prefix.len() <2 {
            return Err(AddressError::InvalidPrefix(prefix.to_vec()));
        }
        match (prefix[0], prefix[1]) {
            (0x00, _) | (0x6F, _) => Ok(BitcoinFormat::P2PKH),
            (0x05, _) | (0xC4, _) => Ok(BitcoinFormat::P2SH_P2WPKH),
            (0x62, _0x63) | (0x74, 0x62) => Ok(BitcoinFormat::P2WSH),
            _ => return Err(AddressError::InvalidPrefix(prefix.to_vec())),
        }
    }

    pub fn from_extended_private_key_version_bytes(prefix: &[u8]) -> Result<Self, ExtendedPrivateKeyError> {
        match prefix[0..4] {
            [0x04, 0x88, 0xAD, 0xE4] | [0x04, 0x35, 0x83, 0x94] => Ok(BitcoinFormat::P2PKH), 
            [0x04, 0x9D, 0x78, 0x78] | [0x04, 0x4A, 0x4E, 0x28] => Ok(BitcoinFormat::P2SH_p2WPKH), 
            _ => Err(ExtendedPrivateKeyError::InvalidVersionBytes(prefix.to_vec())),
        }
    }

    pub fn from_extended_public_key_version_bytes(prefix: &[u8]) -> Result<Self, ExtendedPublickKeyError> {
        match prefix[0..4] {
            [0x04, 0x88, 0xB2, 0x1E] | [0x04, 0x35, 0x87, 0xCF] => Ok(BitcoinFormat::P2PKH), 
            [0x04, 0x9D, 0x7C, 0xB2] | [0x04, 0x4A, 0x4E, 0x62] => Ok(BitcoinFormat::P2SH_p2WPKH), 
            _ => Err(ExtendedPublickKeyError::InvalidVersionBytes(prefix.to_vec())),
        }
    }
}

impl fmt::Display for BitcoinFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BitcoinFormat::P2PKH => write!(f, "P2PKH"),
            BitcoinFormat::P2WSH => write!(f, "P2WSH"),
            BitcoinFormat::P2SH_P2WPKH => write!(f, "P2SH_P2WPKH"),
            BitcoinFormat::Bech32 => write!(f, "Bech32"),
        }
    }
}