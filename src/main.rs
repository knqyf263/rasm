extern crate rasm;

use rasm::run;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("not enough arguments");
    }

    let (file_name, func_name, args) = (&args[1], &args[2], &args[3..]);
    run(file_name, func_name, args)
}
