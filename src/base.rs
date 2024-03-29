use std::{collections::HashSet, fs, io::Write, path::Path, path::PathBuf};

use crate::data;

#[derive(Debug)]
struct Entry {
    otype: data::ObjectType,
    oid: String,
    file: PathBuf,
}

impl Entry {
    fn format(&self) -> String {
        format!(
            "{} {} {}",
            self.otype.as_string(),
            self.oid,
            self.file.to_str().unwrap()
        )
        .replace("\"", "")
    }
}

#[derive(Debug)]
pub struct Commit {
    pub tree: String,
    pub parent: Option<String>,
    pub message: String,
}

pub fn commit(message: &str) -> std::io::Result<String> {
    let mut commit_message = Vec::new();
    commit_message.push(format!("tree {}", write_tree(Path::new("."))));
    if let Some(parent_value) = data::get_ref("HEAD") {
        commit_message.push(format!("parent {}", parent_value.value));
    }
    commit_message.push("".to_string());
    commit_message.push(message.to_string());

    let oid = data::hash_object(
        commit_message.join("\n").replace("\"", "").as_bytes(),
        data::ObjectType::Commit,
    )?;

    data::update_ref(
        "HEAD",
        data::RefValue {
            symbolic: false,
            value: oid.clone(),
        },
    )?;

    Ok(oid)
}

pub fn get_commit(oid: &str) -> Commit {
    let commit = data::get_object(oid, data::ObjectType::Commit);
    let message = std::str::from_utf8(&commit).unwrap().to_string();

    let mut history = message.split("\n");

    let tree = history
        .next()
        .unwrap()
        .to_string()
        .split(" ")
        .last()
        .unwrap()
        .to_string();
    let parent = match history.next().unwrap() {
        "" => None,
        p => Some(p.to_string()),
    };

    Commit {
        tree,
        parent,
        message,
    }
}

pub fn commits_and_parents(mut oids: Vec<String>) -> Vec<String> {
    let mut ordered_oids = vec![];
    let mut visited = HashSet::new();
    while !oids.is_empty() {
        let oid = oids.pop().unwrap();
        if !visited.contains(&oid) {
            visited.insert(oid.clone());
            ordered_oids.push(oid.clone());
            let commit = get_commit(&oid);
            if let Some(parent) = commit.parent {
                oids.push(parent.split(" ").last().unwrap().to_string());
            }
        }
    }

    ordered_oids
}

pub fn get_oid(name: &str) -> String {
    let name = {
        if name == "@" {
            "HEAD"
        } else {
            name
        }
    };

    // Name is a ref
    let ref_names = [
        format!("{}", name),
        format!("refs/{}", name),
        format!("refs/tags/{}", name),
        format!("refs/heads/{}", name),
    ];
    for ref_name in ref_names {
        if let Some(value) = data::get_ref(&ref_name, false) {
            return value.value.to_string();
        }
    }

    // Name is an OID
    for character in name.chars() {
        if !character.is_ascii_hexdigit() {
            panic!("Unknown name {}", name)
        }
    }
    if name.len() != 40 {
        panic!("Unknown name {}", name);
    }
    name.to_string()
}

fn is_ignored(path: &Path) -> bool {
    path.to_str().unwrap().contains(".ugit")
}

pub fn write_tree(directory: &Path) -> String {
    let dir = fs::read_dir(directory).unwrap();

    let mut entries: Vec<Entry> = vec![];

    for entry in dir {
        let path = entry.unwrap().path().to_owned();

        if is_ignored(&path) {
            continue;
        }

        if path.is_symlink() {
            continue;
        } else if path.is_file() {
            let path_str = path.to_str().unwrap();
            let oid = data::hash_object(&std::fs::read(path_str).unwrap(), data::ObjectType::Blob)
                .unwrap();
            entries.push(Entry {
                otype: data::ObjectType::Blob,
                oid,
                file: path,
            });
        } else if path.is_dir() {
            entries.push(Entry {
                otype: data::ObjectType::Tree,
                oid: write_tree(&path),
                file: path,
            });
        }
    }

    let tree = entries
        .iter()
        .map(|entry| entry.format())
        .collect::<Vec<String>>()
        .join("\n");

    data::hash_object(tree.as_bytes(), data::ObjectType::Tree).unwrap()
}

fn tree_entries(oid: &str) -> std::io::Result<Vec<Entry>> {
    let mut all_entries: Vec<Entry> = vec![];

    let tree = data::get_object(oid, data::ObjectType::Tree);
    let entries = std::str::from_utf8(&tree)
        .unwrap()
        .split("\n")
        .map(|s| {
            let mut t = s.split_whitespace().into_iter();
            let otype = data::ObjectType::from_string(t.next().unwrap());
            let oid = t.next().unwrap().to_owned();
            let file = Path::new(t.next().unwrap()).to_owned();

            Entry { otype, oid, file }
        })
        .collect::<Vec<Entry>>();

    for entry in entries {
        match entry.otype {
            data::ObjectType::Blob => all_entries.push(entry),
            data::ObjectType::Commit => continue,
            data::ObjectType::Tree => all_entries.extend(tree_entries(&entry.oid)?),
        }
    }

    Ok(all_entries)
}

fn empty_current_directory(directory: &Path) {
    for path in fs::read_dir(directory).unwrap() {
        let path = path.unwrap();
        let ftype = path.file_type().unwrap();
        if is_ignored(&path.path()) || ftype.is_symlink() {
            continue;
        } else if ftype.is_file() {
            fs::remove_file(path.path()).unwrap();
        } else if ftype.is_dir() {
            fs::remove_dir_all(path.path()).unwrap();
        }
    }
}

pub fn read_tree(tree_oid: &str) -> std::io::Result<()> {
    empty_current_directory(Path::new("."));
    for entry in tree_entries(tree_oid)? {
        if let Some(directory) = Path::new(&entry.file).parent() {
            fs::create_dir_all(&directory).unwrap();
        }
        let mut file = fs::File::create(&entry.file)?;
        file.write_all(&data::get_object(&entry.oid, entry.otype))?;
    }
    Ok(())
}

pub fn checkout(oid: String) -> std::io::Result<()> {
    let commit = get_commit(&oid);
    read_tree(&commit.tree)?;
    data::update_ref(
        "HEAD",
        data::RefValue {
            symbolic: false,
            value: oid,
        },
    )?;

    Ok(())
}

pub fn create_tag(name: &str, oid: String) -> std::io::Result<()> {
    let ref_name = &format!("refs/tags/{}", name);
    data::update_ref(
        ref_name,
        data::RefValue {
            symbolic: false,
            value: oid,
        },
    )?;

    Ok(())
}

pub fn create_branch(name: &str, oid: String) -> std::io::Result<()> {
    data::update_ref(
        &format!("refs/heads/{}", name),
        data::RefValue {
            symbolic: false,
            value: oid,
        },
    )?;

    Ok(())
}
