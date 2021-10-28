use crate::binary::conventions::Vectors;
use crate::binary::decoder::Decoder;
use crate::errors::RuntimeError;
use crate::leb128;
use crate::structure::modules::ExportDesc::Mem;
use crate::structure::modules::{
    Export, ExportDesc, Func, FuncIdx, GlobalIdx, MemIdx, Module, TableIdx, TypeIdx,
};
use crate::structure::types::{FuncType, ValType};
use crate::structure::values::{Byte, Name};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn magic(reader: &mut BufReader<File>) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; 4];
    let magic: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
    let n = reader.read(&mut buf)?;
    if n == 0 || buf != magic {
        Err(RuntimeError::InvalidWasmError("invalid magic".to_string()))?;
    }

    Ok(())
}

pub fn version(reader: &mut BufReader<File>) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; 4];
    let magic: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
    let n = reader.read(&mut buf)?;
    if n == 0 || buf != magic {
        Err(RuntimeError::InvalidWasmError(
            "invalid version".to_string(),
        ))?;
    }

    Ok(())
}

pub fn sections(reader: &mut BufReader<File>) -> Result<Module, Box<dyn Error>> {
    let mut module: Module = Default::default();
    let mut func_indices = vec![];
    loop {
        let mut section_id = [0; 1];
        let n = reader.read(&mut section_id)?;
        if n == 0 {
            return Ok(module);
        }

        let (n, section_size) = leb128::decode::u32(reader)?;

        match section_id[0] {
            1 => {
                println!("=== Type Section (size: {}) ===", section_size);
                let (_, types) = Vectors::<FuncType>::decode(reader)?;
                module.types = (*types).0;

                println!("types: {:?}\n", module.types);
            }
            3 => {
                println!("=== Function Section (size: {}) ===", section_size);
                let (_, res) = Vectors::<TypeIdx>::decode(reader)?;
                func_indices = (*res).0;

                println!("function indices: {:?}\n", func_indices);
            }
            7 => {
                println!("=== Export Section (size: {}) ===", section_size);
                let (_, exports) = Vectors::<Export>::decode(reader)?;
                module.exports = (*exports).0;

                println!("exports: {:?}\n", module.exports);
            }
            10 => {
                println!("=== Code Section (size: {}) ===", section_size);
                let (_, funcs) = Vectors::<Func>::decode(reader)?;
                let mut funcs = (*funcs).0;
                for (i, mut func) in funcs.iter_mut().enumerate() {
                    let typeidx = func_indices.get(i).ok_or("unknown index")?;
                    func.type_ = *typeidx;
                }
                module.funcs = funcs;

                println!("funcs: {:?}\n", module.funcs)
            }
            _ => Err(RuntimeError::InvalidWasmError(
                "not implemented".to_string(),
            ))?,
        };
    }
}

// Code Section: https://webassembly.github.io/spec/core/binary/modules.html#binary-local
pub struct Locals(pub u32, pub ValType);

impl Decoder for Locals {
    fn decode<R>(reader: &mut R) -> Result<(usize, Box<Self>), Box<dyn Error>>
    where
        R: Read,
    {
        // Decode a u32 count
        let (n1, local_count) = leb128::decode::u32(reader)?;

        // Decode a value type
        let (n2, t) = ValType::decode(reader)?;

        Ok((n1 + n2, Box::new(Locals(local_count, *t))))
    }
}

impl Decoder for Export {
    fn decode<R>(reader: &mut R) -> Result<(usize, Box<Self>), Box<dyn Error>>
    where
        R: Read,
    {
        let (n1, name) = Vectors::<Byte>::decode(reader)?;
        let name = (*name).0;
        let name = String::from_utf8(name.iter().map(|b| b.0).collect())?;

        let mut buf = [0; 1];
        reader.read_exact(&mut buf)?;
        let (n2, desc) = match buf[0] {
            0x00 => {
                let (n2, funcidx) = leb128::decode::u32(reader)?;
                (n2, ExportDesc::Func(FuncIdx(funcidx)))
            }
            0x01 => {
                let (n2, tableidx) = leb128::decode::u32(reader)?;
                (n2, ExportDesc::Table(TableIdx(tableidx)))
            }
            0x02 => {
                let (n2, memidx) = leb128::decode::u32(reader)?;
                (n2, ExportDesc::Mem(MemIdx(memidx)))
            }
            0x03 => {
                let (n2, globalidx) = leb128::decode::u32(reader)?;
                (n2, ExportDesc::Global(GlobalIdx(globalidx)))
            }
            _ => {
                return Err(Box::new(RuntimeError::InvalidWasmError(
                    "unknown export descriptor".to_string(),
                )))
            }
        };

        Ok((
            n1 + n2 + 1,
            Box::new(Export {
                name: Name(name),
                desc,
            }),
        ))
    }
}
