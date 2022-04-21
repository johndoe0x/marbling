use crate::format::Format;
use crate::private_key::{PrivateKey,PrivateKeyError};
use crate::public_key::{PublicKey,PublicKeyError};
use crate::no_std::*;

use core::{
    fmt::{Debug,Display},
    hash::Hash,
    str::FromStr,
};

pub trait Address : 'static+ Clone+ Debug+ Display+ FromStr+ Hash+ PartialEq+ Eq+ Ord+ Send+ Sized+ Sync {
    type Format: Format;
    type PrivateKey: PrivateKey;
    type PublicKey: PublicKey;
}

