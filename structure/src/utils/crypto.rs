use crate::no_std::*;
use ripemd160::Ripemd160;
use sha2::{Digest, Sha256};

pub fn checksum(data: &[u8]) -> Vec<u8> {
    Sha256::digest(&Sha256::digest(data)).to_vec()
}

pub fn hash160(bytes: &[u8]) -> Vec<u8> {
    Ripemd160::digest(&Sha256::digest(bytes)).to_vec()
}