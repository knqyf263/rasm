use crate::binary::conventions::Vectors;
use crate::binary::decoder::Decoder;
use crate::binary::modules::Locals;
use crate::errors::RuntimeError::InvalidWasmError;
use crate::leb128;
use crate::structure::instructions::Expr;
use crate::structure::modules::{Func, TypeIdx};
use crate::structure::types::{FuncType, NumType, ResultType, ValType};
use std::error::Error;
use std::fs::{read, File};
use std::io::Read;

impl Decoder for FuncType {
    fn decode<R>(reader: &mut R) -> Result<(usize, Box<Self>), Box<dyn Error>>
    where
        R: Read,
    {
        let mut type_id = [0; 1];
        reader.read_exact(&mut type_id)?;
        if type_id[0] != 0x60 {
            Err(InvalidWasmError("invalid func type".to_string()))?;
        }
        println!("type_id: {}", type_id[0]);

        let (n1, params) = ResultType::decode(reader)?;
        let (n2, results) = ResultType::decode(reader)?;
        println!("params: {:?}", params);
        println!("results: {:?}", results);
        Ok((n1 + n2, Box::new(FuncType(*params, *results))))
    }
}

impl Decoder for ResultType {
    fn decode<R>(reader: &mut R) -> Result<(usize, Box<Self>), Box<dyn Error>>
    where
        R: Read,
    {
        let (n, res) = Vectors::<ValType>::decode(reader)?;
        Ok((n, Box::new(ResultType((*res).0))))
    }
}

impl Decoder for ValType {
    fn decode<R>(reader: &mut R) -> Result<(usize, Box<Self>), Box<dyn Error>>
    where
        R: Read,
    {
        let mut valtype = [0; 1];
        reader.read_exact(&mut valtype)?;

        let numtype = match valtype[0] {
            0x7f => NumType::I32,
            0x7e => NumType::I64,
            0x7d => NumType::F32,
            0x7c => NumType::F64,
            _ => Err(InvalidWasmError(
                format!("unknown number type {}", valtype[0]).to_string(),
            ))?,
        };

        Ok((0, Box::new(ValType::NumType(numtype))))
    }
}

impl Decoder for TypeIdx {
    fn decode<R>(reader: &mut R) -> Result<(usize, Box<Self>), Box<dyn Error>>
    where
        R: Read,
    {
        let (n, typeidx) = leb128::decode::u32(reader)?;
        Ok((n, Box::new(TypeIdx(typeidx))))
    }
}

impl Decoder for Func {
    fn decode<R>(reader: &mut R) -> Result<(usize, Box<Self>), Box<dyn Error>>
    where
        R: Read,
    {
        let (n1, size) = leb128::decode::u32(reader)?;

        let (n2, t) = Vectors::<Locals>::decode(reader)?;

        let mut locals = vec![];
        for l in (*t).0 {
            let (n, valtype) = (l.0, l.1);
            for _ in 0..n {
                locals.push(valtype.clone())
            }
        }
        println!("locals: {:?}", locals);

        let (n3, expr) = Expr::decode(reader)?;
        println!("expr: {:?}", expr);

        let func = Func {
            type_: TypeIdx(0),
            locals,
            body: *expr,
        };

        Ok((n1 + n2 + n3, Box::new(func)))
    }
}
