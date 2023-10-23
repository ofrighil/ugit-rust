use std::io::{self, Write};

use clap::{Command, arg, Arg};
use ugit_rust::{data, base};

fn parse_args() -> Result<(), std::io::Error> {
    // let matches = command!()
    let matches = Command::new("ugit")
        .subcommand_required(true)
        .subcommand(
            Command::new("init")
                .about("Initialize new git repository")
                .arg(arg!([NAME]))
        )
        .subcommand(
            Command::new("hash-object")
                .about("Get the hash of a file object") 
                .arg(
                    Arg::new("file")
                        .help("Name of file")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("cat-file")
                .about("Prints the file by object")
                .arg(
                    Arg::new("object")
                        .help("Object")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("write-tree")
                .about("Writes the tree to ugit")
                .arg(
                    Arg::new("directory")
                        .help("Directory")
                        .default_value(".")
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => init(),
        Some(("hash-object", sub_matches)) => hash_object(
            sub_matches.get_one::<String>("file").unwrap()
        ), 
        Some(("cat-file", sub_matches)) => cat_file(
            sub_matches.get_one::<String>("object").unwrap()
        ),
        Some(("write-tree", sub_matches)) => write_tree(
            sub_matches.get_one::<String>("directory").unwrap()
        ),
        _ => unreachable!("No subcommand"), 
    }
}

fn init() -> Result<(), std::io::Error> {
    data::init()?;
    println!(
        "Initialized empty ugit repository in {}/{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        data::GIT_DIR
    );
    Ok(())
}

fn hash_object(file: &str) -> Result<(), std::io::Error> {
    println!(
        "{}",
        data::hash_object(
            std::fs::read(file).unwrap(), data::ObjectType::Blob
        ).unwrap()
    );
    Ok(())
}

fn cat_file(object: &str) -> Result<(), std::io::Error> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(&data::get_object(object, data::ObjectType::Blob))?;
    Ok(())
}

fn write_tree(directory: &str) -> Result<(), std::io::Error> {
    base::write_tree(directory);
    Ok(())
}

fn main() {
    parse_args().unwrap()
}
