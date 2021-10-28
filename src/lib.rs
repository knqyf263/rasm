mod binary;
mod errors;
mod execution;
mod leb128;
mod structure;

use crate::execution::runtime::Store;
use crate::execution::stack::Val;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn run(file_name: &String, func_name: &String, args: &[String]) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(file_name)?);

    binary::modules::magic(&mut reader)?;
    binary::modules::version(&mut reader)?;
    let module = binary::modules::sections(&mut reader)?;
    let store = Store::new(&module)?;

    let args = args
        .iter()
        .map(|a| a.parse::<i32>().unwrap())
        .map(|a| Val::I32(a))
        .collect();

    let result = module.call(store, func_name, args)?;

    if let Val::I32(res) = result[0] {
        println!("result: {}", res);
    }

    Ok(())
}
