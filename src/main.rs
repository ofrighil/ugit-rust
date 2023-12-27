use std::io::{self, Write};

use clap::{Arg, Command};
use ugit_rust::{base, data};

fn parse_args() -> Result<(), std::io::Error> {
    // let matches = command!()
    let matches = Command::new("ugit")
        .subcommand_required(true)
        .subcommand(Command::new("init").about("Initialize new git repository"))
        .subcommand(
            Command::new("hash-object")
                .about("Get the hash of a file object")
                .arg(Arg::new("file").help("Name of file").required(true)),
        )
        .subcommand(
            Command::new("cat-file")
                .about("Prints the file by object")
                .arg(Arg::new("object").help("Object").required(true)),
        )
        .subcommand(
            Command::new("write-tree")
                .about("Writes the tree to ugit")
                .arg(Arg::new("directory").help("Directory").default_value(".")),
        )
        .subcommand(
            Command::new("read-tree")
                .about("Reads the tree object")
                .arg(Arg::new("tree").help("Tree Object").required(true)),
        )
        .subcommand(
            Command::new("commit").about("Commits the changes").arg(
                Arg::new("message")
                    .short('m')
                    .help("Commit message")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("log")
                .about("Print commit information")
                .arg(Arg::new("oid").help("The object ID")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => init(),
        Some(("hash-object", sub_matches)) => {
            hash_object(sub_matches.get_one::<String>("file").unwrap())
        }
        Some(("cat-file", sub_matches)) => {
            cat_file(sub_matches.get_one::<String>("object").unwrap())
        }
        Some(("write-tree", sub_matches)) => {
            write_tree(sub_matches.get_one::<String>("directory").unwrap())
        }
        Some(("read-tree", sub_matches)) => {
            read_tree(sub_matches.get_one::<String>("tree").unwrap())
        }
        Some(("commit", sub_matches)) => commit(sub_matches.get_one::<String>("message").unwrap()),
        Some(("log", sub_matches)) => log(sub_matches.get_one::<String>("oid").cloned()),
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
        data::hash_object(&std::fs::read(file).unwrap(), data::ObjectType::Blob).unwrap()
    );
    Ok(())
}

fn cat_file(object: &str) -> Result<(), std::io::Error> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(&data::get_object(object, data::ObjectType::Blob))?;
    Ok(())
}

fn write_tree(directory: &str) -> Result<(), std::io::Error> {
    println!("{}", base::write_tree(std::path::Path::new(directory)));
    Ok(())
}

fn read_tree(tree: &str) -> Result<(), std::io::Error> {
    base::read_tree(tree)?;
    Ok(())
}

fn commit(message: &str) -> Result<(), std::io::Error> {
    println!("{}", base::commit(message)?);
    Ok(())
}

fn log(input_oid: Option<String>) -> Result<(), std::io::Error> {
    let mut oid = input_oid.map_or(data::get_HEAD(), |ioid| Some(ioid));

    while let Some(o) = oid {
        let commit: base::Commit = base::get_commit(&o.replace("\"", ""));

        println!("commit {}", o);
        println!("{}", commit.message);
        println!();

        if let Some(p) = commit.parent {
            oid = Some(p.replace("parent ", ""));
        } else {
            oid = None
        }
    }

    Ok(())
}

fn main() {
    parse_args().unwrap()
}
