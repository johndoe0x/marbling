//! no-std io replacement
use crate::no_std::Vec;
use core::{cmp, fmt, mem};
use uint::uint_full_mul_reg;


pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error>;
}

impl<R: Read+?Sized> Read for &mut R {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        (**self).read(buf)
    }
    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        (**self).read_exact(buf)
    }
}

impl Read for &[u8] {
    #[inline]
    fn read(&mut self , buf: &mut [u8]) -> Result<usize, Error> {
        let amt = cmp::min(buf.len(),self.len());
        let (a,b) = self.split_at(amt);

        if amt ==1 {
            buf[0] = a[0];
        }else {
            buf[..amt].copy_from_slice(a);
        }
        *self = b;
        Ok(amt)

    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        if buf.len() > self.len() {
            return Err(Error);
        }
        let(a,b) = self.split_at(buf.len());

        if buf.len() ==1 {
            buf[0] = a[0];
        }else{
            buf.copy_from_slice(a);
        }

        *self = b;
        Ok(())
    }
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
}

impl<w: Write + ?Sized> Write for &mut W {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        (**self).write(buf)
    }
    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        (**self).write_all(buf)
    }
}

impl Write for &mut [u8] {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        todo!()
    }
    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        todo!()
    }
}

impl Write for Vec<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        todo!()
    }
    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        todo!()
    }
}
#[derive(Debug)]
pub struct Error;

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Error in no_std::io operation")
    }
}

