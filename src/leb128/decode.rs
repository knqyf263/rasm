// Ported from https://github.com/gimli-rs/leb128
use std::io;
use thiserror::Error as TError;

pub const CONTINUATION_BIT: u8 = 1 << 7;
pub const SIGN_BIT: u8 = 1 << 6;

pub fn low_bits_of_byte(byte: u8) -> u8 {
    byte & !CONTINUATION_BIT
}

#[derive(Debug, TError)]
pub enum Error {
    /// There was an underlying IO error.
    #[error("IO error: {0}")]
    IoError(io::Error),
    /// The number being read is larger than can be represented.
    #[error("leb128 overflow")]
    Overflow,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

/// Read an unsigned LEB128-encoded number from the `std::io::Read` stream
/// `r`.
///
pub fn u32<R>(r: &mut R) -> Result<(usize, u32), Error>
where
    R: io::Read,
{
    let mut result: u32 = 0;
    let mut shift = 0;
    let mut size: usize = 0;
    let mut byte;

    loop {
        let mut buf = [0];
        r.read_exact(&mut buf)?;
        size += 1;

        byte = buf[0];
        if shift == 31 && byte != 0x00 && byte != 0x01 {
            while byte & CONTINUATION_BIT != 0 {
                r.read_exact(&mut buf)?;
            }
            return Err(Error::Overflow);
        }

        let low_bits = low_bits_of_byte(byte) as u32;
        result |= low_bits << shift;

        if buf[0] & CONTINUATION_BIT == 0 {
            return Ok((size, result));
        }

        shift += 7;
    }
}

/// Read a signed LEB128-encoded number from the `std::io::Read` stream `r`.
///
/// On success, return the number.
pub fn i32<R>(r: &mut R) -> Result<(usize, i32), Error>
where
    R: ?Sized + io::Read,
{
    let mut result: i32 = 0;
    let mut shift = 0;
    let mut size: usize = 0;
    let mut byte;

    loop {
        let mut buf = [0];
        r.read_exact(&mut buf)?;
        size += 1;

        byte = buf[0];
        if shift == 63 && byte != 0x00 && byte != 0x7f {
            while buf[0] & CONTINUATION_BIT != 0 {
                r.read_exact(&mut buf)?;
            }
            return Err(Error::Overflow);
        }

        let low_bits = low_bits_of_byte(byte) as i32;
        result |= low_bits << shift;
        shift += 7;

        if byte & CONTINUATION_BIT == 0 {
            break;
        }
    }

    if shift < 32 && (SIGN_BIT & byte) == SIGN_BIT {
        // Sign extend the result.
        result |= !0 << shift;
    }

    Ok((size, result))
}
