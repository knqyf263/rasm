use crate::binary::decoder::Decoder;
use crate::errors::RuntimeError;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

// Vectors: https://webassembly.github.io/spec/core/binary/conventions.html#vectors
#[derive(Debug, Clone, PartialEq)]
pub struct Vectors<T>(pub Vec<T>)
where
    T: Decoder;

impl<T> Decoder for Vectors<T>
where
    T: Decoder,
{
    fn decode<R>(reader: &mut R) -> Result<(usize, Box<Self>), Box<dyn Error>>
    where
        R: Read,
    {
        let mut num = [0; 1];
        let n = reader.read(&mut num)?;
        if n == 0 {
            Err(RuntimeError::InvalidWasmError(
                "invalid vectors".to_string(),
            ))?;
        }

        let mut size = 1;
        let mut vecs: Vec<T> = vec![];
        for _ in 0..num[0] {
            let (n, res) = T::decode(reader)?;
            vecs.push(*res);
            size += n;
        }

        Ok((size, Box::new(Vectors(vecs))))
    }
}
