use std::io::{self, Write};

use clap::{Arg, Command};
use ugit_rust::{base, data};

fn parse_args() -> Result<(), std::io::Error> {
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
                .arg(Arg::new("oid").default_value("@").help("The object ID")),
        )
        .subcommand(
            Command::new("checkout")
                .about("Checkout by an object ID")
                .arg(Arg::new("oid").help("The object ID")),
        )
        .subcommand(
            Command::new("tag")
                .about("Checkout by an object ID")
                .arg(Arg::new("name").help("The tag name").required(true))
                .arg(Arg::new("oid").default_value("@").help("The object ID")),
        )
        .subcommand(
            Command::new("branch")
                .about("Create a new branch")
                .arg(Arg::new("name").help("The branch name").required(true))
                .arg(
                    Arg::new("start_point")
                        .default_value("@")
                        .help("The object ID"),
                ),
        )
        .subcommand(Command::new("k").about("Visualizer tool to draw all refs and commits"))
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
        Some(("log", sub_matches)) => log(sub_matches.get_one::<String>("oid").unwrap()),
        Some(("checkout", sub_matches)) => checkout(sub_matches.get_one::<String>("oid").unwrap()),
        Some(("tag", sub_matches)) => tag(
            sub_matches.get_one::<String>("name").unwrap(),
            sub_matches.get_one::<String>("oid").unwrap(),
        ),
        Some(("branch", submatches)) => branch(
            submatches.get_one::<String>("name").unwrap(),
            submatches.get_one::<String>("start_point").unwrap(),
        ),
        Some(("k", _)) => k(),
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
    stdout.write_all(&data::get_object(
        &base::get_oid(object),
        data::ObjectType::Blob,
    ))?;
    Ok(())
}

fn write_tree(directory: &str) -> Result<(), std::io::Error> {
    println!("{}", base::write_tree(std::path::Path::new(directory)));
    Ok(())
}

fn read_tree(tree: &str) -> Result<(), std::io::Error> {
    base::read_tree(&base::get_oid(tree))?;
    Ok(())
}

fn commit(message: &str) -> Result<(), std::io::Error> {
    println!("{}", base::commit(message)?);
    Ok(())
}

fn log(oid: &str) -> Result<(), std::io::Error> {
    for oid in base::commits_and_parents(vec![base::get_oid(oid)]).into_iter() {
        let commit = base::get_commit(&oid);
        println!("commit {}", oid);
        println!("{}\n", commit.message);
    }

    Ok(())
}

fn checkout(oid: &str) -> Result<(), std::io::Error> {
    base::checkout(base::get_oid(oid))?;
    Ok(())
}

fn tag(name: &str, oid: &str) -> Result<(), std::io::Error> {
    base::create_tag(name, base::get_oid(oid))?;
    Ok(())
}

fn branch(name: &str, start_point: &str) -> Result<(), std::io::Error> {
    let oid = base::get_oid(start_point);
    base::create_branch(name, oid.clone())?;
    println!("Branch {} created at {}", name, &oid[0..10]);
    Ok(())
}

fn k() -> Result<(), std::io::Error> {
    let mut dot = "digraph commits {\n".to_string();
    let mut oids = vec![];
    for r in data::refs().into_iter() {
        let o = base::get_oid(&r);
        dot.push_str(&format!("\"{}\" [shape=note]\n", &r));
        dot.push_str(&format!("\"{}\" -> \"{}\"\n", &r, &o));
        oids.push(o);
    }

    for oid in base::commits_and_parents(oids) {
        let commit = base::get_commit(&oid);
        dot.push_str(&format!(
            "\"{}\" [shape=box style=filled label=\"{}\"\n]",
            oid,
            &oid[0..10]
        ));
        if let Some(parent) = commit.parent {
            dot.push_str(&format!("\"{}\" -> \"{}\"\n", oid, parent));
        }
    }

    dot.push('}');
    println!("{}", dot);

    // Yeah, I'm not actually going to subprocess dot

    Ok(())
}

fn main() {
    parse_args().unwrap()
}
