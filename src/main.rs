// parse p0f log file
use std::fs::File;
use std::io::{prelude::*, BufReader};
use clap::{Arg, App};



fn main() {
    let args = App::new("P0f log parser")
        .version("0.1.0")
        .author("Jrmy Grmnprz <jgrmnprz@gmail.com>")
        .about("Parse p0f log files")
        .arg(Arg::with_name("file")
                 .short("f")
                 .long("file")
                 .required(true)
                 .takes_value(true)
                 .help("P0f log to parse"))
        .get_matches();

    let logfile = args.value_of("file").unwrap();
    println!("The file passed is: {}", logfile);

    let f = File::open(logfile).expect("Unable to open file");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{:?}", line);
    }
}
