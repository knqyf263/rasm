use crate::binary::decoder::Decoder;
use crate::structure::values::Byte;
use std::error::Error;
use std::io::Read;

impl Decoder for Byte {
    fn decode<R>(reader: &mut R) -> Result<(usize, Box<Self>), Box<dyn Error>>
    where
        R: Read,
    {
        let mut buf = [0; 1];
        reader.read_exact(&mut buf)?;
        Ok((1, Box::new(Byte(buf[0]))))
    }
}
