use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use p0f_parser::parse_line;

fn main() -> Result<(), io::Error> {
    let path = env::args().skip(1).next().unwrap_or("./p0f.log".to_owned());
    let logfile = File::open(path)?;
    let reader = BufReader::new(logfile);

    for line in reader.lines() {
        let line = line?;
        match parse_line(&line) {
            Ok((_rest, p0f)) => println!("{:?}", p0f),
            Err(ref e) => match e {
                nom::Err::Failure(f) => {
                    return Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", f)));
                }
                nom::Err::Error(e) => {
                    eprintln!("Error: {:?} at {}", e, line);
                }
                nom::Err::Incomplete(n) => {
                    eprintln!("Incomplete line, need {:?}", n);
                }
            },
        }
    }
    Ok(())
}
