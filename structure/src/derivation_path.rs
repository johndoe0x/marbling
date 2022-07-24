use crate::no_std::*;
use core::{
    fmt,
    fmt::{Debug,Display},
    str::FromStr,};


pub trait DerivationPath: Clone + Debug + Display + FromStr + Send + Sync  + 'static + Eq + Sized {
    fn to_vec(&self) -> Result<Vec<ChildIndex>, DerivationPathError>;

    fn from_vec(path: &Vec<ChildIndex>) -> Result<Self, DerivationPathError>;
    
}

#[derive(Debug, Fail, PartialEq, Eq)]

pub enum  DerivationPathError {
    #[fail(display = "expected BIP32 path")]
    ExpectedBIP32Path,

    #[fail(display = "expected BIP44 path")]
    ExpectedBIP44Path,

    #[fail(display = "expected BIP49 path")]
    ExpectedBIP49Path,

    #[fail(display = "expected valid Ethereum derivation path")]
    ExpectedValidEthereumDerivationPath,

    #[fail(display = "expected ZIP32")]
    ExpectedZIP32Path,
    
    #[fail(display = "expected hardend path")]
    ExpectedHardenedPath,

    #[fail(display = "expected normal path")]
    ExpectedNormalPath,

    #[fail(display = "invalid child number: {}", _0)]
    InvalidChildNumber(u32),

    #[fail(display = "Child number format is wrong")]
    InvalidChildNumberFormat,

    #[fail(display = "Derivation path is wrong: {}", _0)]
    InvalidDerivationPath(String),
}

// Represents a child index for a derivation path
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChildIndex {
     // A non-hardened index: Normal(n) == n in path notation
     Normal(u32),
     // A hardened index: Hardened(n) == n + (1<<31)==n in path notation
     Hardened(u32),
}

impl ChildIndex{
    pub fn normal(index: u32) -> Result<Self, DerivationPathError> {
        if index &(1<<31) == 0 {
            Ok(ChildIndex::Normal(index))
        } else {
            Err(DerivationPathError::InvalidChildNumber(index))
        }        
    }

    pub fn hardened(index: u32) -> Result<Self, DerivationPathError> {
        if index & (1 <<31) ==  0 {
            Ok(ChildIndex::Hardened(index))
        }else {
            Err(DerivationPathError::InvalidChildNumber(index))
        }
    }

    pub fn is_normal(&self) -> bool{
        !self.is_hardened()
    }

    pub fn is_hardened(&self) -> bool{
        match *self{
            ChildIndex::Hardened(_) => true,
            ChildIndex::Normal(_) => false,
        }
    }

    pub fn to_index(&self)-> u32 {
        match self {
            &ChildIndex::Hardened(i) => i +(1 <<31),
            &ChildIndex::Normal(i) =>i,
        }
    }
}

impl From<u32> for ChildIndex {
    fn from(number: u32)->Self{
        if number & (1<<31)!=0 {
            ChildIndex::Hardened(number ^ (1 << 31))
        }else {
            ChildIndex::Normal(number)
        }
    }
}

impl From<ChildIndex> for u32 {
    fn from(index: ChildIndex) -> Self {
        match index {
            ChildIndex::Hardened(number) => number,
            ChildIndex::Normal(number) => number | (1<<31),
        }
    }
}

impl FromStr for ChildIndex {
    type Err = DerivationPathError;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        Ok(match inp.chars().last().map_or(false, |l|l=='\''|| l=='h'){
            true => Self::hardened(
                inp[0..inp.len()-1]
                .parse()
                .map_err(|_| DerivationPathError::InvalidChildNumberFormat)?,
            )?,
            false => Self::normal(inp.parse().map_err(|_| DerivationPathError::InvalidChildNumberFormat)?)?,
        })
    }
}

impl fmt::Display for ChildIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self  {
            ChildIndex::Hardened(number) => write!(f, "{}'", number),
            ChildIndex::Normal(number) => write!(f, "{}'", number),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn normal() {
        for i in 0..1 <<31 {
            assert_eq!(ChildIndex::Normal(i), ChildIndex::normal(i).unwrap());
        }

        for i in 1<<31..core::u32::MAX  {
            assert_eq!(Err(DerivationPathError::InvalidChildNumber(i)), ChildIndex::normal(i));
        }
    }

    #[test]
    fn hardened() {
        for i in 0..1 <<31 {
            assert_eq!(ChildIndex::Hardened(i), ChildIndex::hardened(i).unwrap());
        }
        for i in 1<<31..core::u32::MAX  {
            assert_eq!(Err(DerivationPathError::InvalidChildNumber(i)), ChildIndex::hardened(i));
        }
    }

    #[test]
    fn is_normal() {
        for  i in 0..1<< 31 {
            assert!(ChildIndex::Normal(i).is_normal());
            assert!(!ChildIndex::Hardened(i).is_normal());
        }
    }

    #[test]
    fn to_index() {
        for i in 0..1<<31 {
            assert_eq!(i, ChildIndex::Normal(i).to_index());
            assert_eq!(i | (1<<31), ChildIndex::Hardened(i).to_index());
        }
    }

    #[test]
    fn from() {
        const TRESHOLD: u32 = 1<<31;
        for i in 0..core::u32::MAX{
            match i < TRESHOLD {
                true => assert_eq!(ChildIndex::Normal(i), ChildIndex::from(i)),
                false => assert_eq!(ChildIndex::Hardened(i ^ 1 << 31), ChildIndex::from(i)),
            }
                
            }
        }
    
    #[test]
    fn from_str() {
        for i in (0..1 << 31  ).step_by(1 << 10 ) {
            assert_eq!(ChildIndex::Normal(i), ChildIndex::from_str(&format!("{}'", i)).unwrap());
            assert_eq!(
                ChildIndex::Hardened(i),
                ChildIndex::from_str(&format!("{}\'", i)).unwrap()
            );
            assert_eq!(
                ChildIndex::Hardened(i),
                ChildIndex::from_str(&format!("{}h", i)).unwrap()
            );
        }
    }

    #[test]
    fn to_string() {
        for i in (0..1 <<31 ).step_by(1<<10) {
            assert_eq!(format!("{}'", i), ChildIndex::Normal(i).to_string());
            assert_eq!(format!("{}\'", i), ChildIndex::Hardened(i).to_string());
        }
    }
}


