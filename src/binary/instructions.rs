use crate::binary::decoder::Decoder;
use crate::errors::RuntimeError;
use crate::leb128;
use crate::structure::instructions::Instr::{
    F32Const, F64Const, I32Add, I32Const, I32Mul, I32Sub, I64Const, LocalGet, LocalSet,
};
use crate::structure::instructions::{Expr, Instr};
use crate::structure::modules::LocalIdx;
use std::error::Error;
use std::io::Read;

impl Decoder for Expr {
    fn decode<R>(reader: &mut R) -> Result<(usize, Box<Self>), Box<dyn Error>>
    where
        R: Read,
    {
        let mut size: usize = 0;
        let mut expr: Vec<Instr> = vec![];
        loop {
            let mut opcode = [0; 1];
            reader.read_exact(&mut opcode)?;

            let (n, instr) = match opcode[0] {
                0x0b => break,

                // Variable Instructions: https://webassembly.github.io/spec/core/binary/instructions.html#variable-instructions
                0x20 => local_get(reader)?,
                0x21 => local_set(reader)?,

                // Numeric Instructions: https://webassembly.github.io/spec/core/binary/instructions.html#numeric-instructions
                0x41 => i32_const(reader)?,

                0x6a => i32_add()?,
                0x6b => i32_sub()?,
                0x6c => i32_mul()?,

                _ => Err(RuntimeError::InvalidWasmError(
                    "not implemented".to_string(),
                ))?,
            };
            expr.push(*instr);
            size += n;
        }

        Ok((size, Box::new(Expr(expr))))
    }
}

fn local_get<R>(reader: &mut R) -> Result<(usize, Box<Instr>), Box<dyn Error>>
where
    R: Read,
{
    let (n, localidx) = leb128::decode::u32(reader)?;
    Ok((0, Box::new(LocalGet(LocalIdx(localidx)))))
}

fn local_set<R>(reader: &mut R) -> Result<(usize, Box<Instr>), Box<dyn Error>>
where
    R: Read,
{
    let (n, localidx) = leb128::decode::u32(reader)?;
    Ok((0, Box::new(LocalSet(LocalIdx(localidx)))))
}

fn i32_const<R>(reader: &mut R) -> Result<(usize, Box<Instr>), Box<dyn Error>>
where
    R: Read,
{
    let (n, num) = leb128::decode::i32(reader)?;
    Ok((0, Box::new(I32Const(num))))
}

fn i32_add() -> Result<(usize, Box<Instr>), Box<dyn Error>> {
    Ok((0, Box::new(I32Add)))
}

fn i32_sub() -> Result<(usize, Box<Instr>), Box<dyn Error>> {
    Ok((0, Box::new(I32Sub)))
}

fn i32_mul() -> Result<(usize, Box<Instr>), Box<dyn Error>> {
    Ok((0, Box::new(I32Mul)))
}
