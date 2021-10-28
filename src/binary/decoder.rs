use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

pub trait Decoder {
    fn decode<R>(reader: &mut R) -> Result<(usize, Box<Self>), Box<dyn Error>>
    where
        R: Read;
}
