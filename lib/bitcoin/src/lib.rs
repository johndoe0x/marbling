#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unused_extern_crates, dead_code)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate failure;

pub mod address;
pub use self::address::*;

pub mod network;
pub use self::network::*;