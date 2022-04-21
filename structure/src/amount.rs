use crate::no_std::*;
use core::{ fmt::{Debug,Display}, hash::Hash,};

pub trait amount: Copy+Clone+Debug+Display+Send+Sync+'static+ Eq+Ord+Sized+Hash {
}

#[derive(Debug,Fail)]
pub enum  AmountError {
    #[fail(display= "Amount : {} is exeeds {}", _0, _1)]
    AmountOutBounds(String,String),

    #[fail(display = "{} : {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display= "invalid amount: {}", _0)]
    InvalidAmount(String)
}