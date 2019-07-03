use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => Err("No input file specified"),
    }

    let file = File::open(filename)?;
    for line in BufReader::new(file).lines() {
    }

    Ok(())
}
