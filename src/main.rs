use clap::{Command, arg, Arg};
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
        .subcommand(
            Command::new("hash-object")
                .about("Get the hash of a file object") 
                .arg(
                    Arg::new("file")
                        .help("Name of file")
                        .required(true)
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => init(),
        Some(("hash-object", sub_matches)) => hash_object(
            sub_matches.get_one::<String>("file").unwrap()
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
        data::hash_object(std::fs::read(file).unwrap()).unwrap()
    );
    Ok(())
}

fn main() {
    parse_args().unwrap()
}
