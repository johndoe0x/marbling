use marbling_structure::no_std::*;
use marbling_structure::{Amount, AmountError};

use core::fmt;
use serde::Serialize;


const COIN: i64 = 1_0000_0000;

const MAX_COINS: i64 = 21_000_000 * COIN;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]

pub struct BitcoinAmount(pub i64);

pub enum Denomination {
    Satoshi,
    MicroBit, 
    MilliBit,
    CentBit, 
    DeciBit, 
    Bitcoin,
}

impl Denomination {
    fn precision(self)
}
