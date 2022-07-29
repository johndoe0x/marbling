use crate::format::BitcoinFormat;
use crate::network::BitcoinNetwork;
use marbling_structure::no_std::*;
use marbling_structure::*;

use core::{
    fmt,
    str::FromStr
};
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord,  Hash, Serialize)]

pub struct Mainnet;

impl Network for Mainnet {
    const NAME: &'static str = "mainnet";
}

impl BitcoinNetwork for Mainnet {
    const HD_COIN_TYPE: ChildIndex = ChildIndex::Hardened(0);

    fn to_address_prefix(format: &BitcoinFormat) -> Vec<u8> {
        match format {
            BitcoinFormat::P2PKH => vec![0x00],
            Bitcoin
        }
    }
}

