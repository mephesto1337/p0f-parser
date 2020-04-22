extern crate p0f_parser;
use p0f_parser::parsers::parse_line;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let path = "/var/tmp/p0f.log";
    let logfile = File::open(path)?;
    let reader = BufReader::new(logfile);

    for (_num, line) in reader.lines().enumerate() {
        let mut l = line.unwrap();
        let r = parse_line(&mut l);
        println!("{:?}", r);
    }
    Ok(())
}
