use std::{error::Error, io::BufRead};

use clap::{command, Command, arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

fn parse_args() {
    // let matches = command!()
    let matches = Command::new("ugit")
        .subcommand_required(true)
        .subcommand(
            Command::new("init")
                .about("Initialize new git repository")
                .arg(arg!([NAME]))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            println!("Hello World");
        },
        _ => unreachable!("No subcommand"), 
    }
}

fn main() {
    // println!("Hello, world!");
    parse_args();
}
