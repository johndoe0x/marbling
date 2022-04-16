use crate::no_std::*;

#[cfg_attr(test,macro_use)]
pub mod crypto;

pub fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join("")
}