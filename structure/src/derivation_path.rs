use crate::no_std::*;
use core::{fmt,fmt::{Debug,Display},str::FromStr};

// TODO: In this file need to add explain what is the Derivation Path.
pub trait DerivationPath: Clone +Display+Debug+FromStr+Send+Sync+`static + Eq +Sized {
    fn to_vec(&self) -> Result<Vec<ChildIndex>, DerivationPath>;

    fn from_vec(path: &Vec<ChildIndex>) -> Result<Self, DerivationPathError>;
    
}

#[derive(Debug, Fail, PartialEq, Eq)]

pub enum  DerivationPathError {
    #[fail(display = "Invalid derivation path: {}", _0)]
    InvalidDerivationPath(String),

    

    
}