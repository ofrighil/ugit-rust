use std::{error::Error, io::BufRead};

use clap::{command, Command, arg};
use ugit_rust::data;

fn parse_args() -> Result<(), std::io::Error> {
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
        Some(("init", _)) => {
            data::init()
        },
        _ => unreachable!("No subcommand"), 
    }
}

fn main() {
    parse_args().unwrap()
}
