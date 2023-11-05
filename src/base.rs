use std::{fs, io::Write, path::Path, path::PathBuf};

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

fn is_ignored(path: &PathBuf) -> bool {
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
            data::ObjectType::Tree => all_entries.extend(tree_entries(&entry.oid)?),
        }
    }

    Ok(all_entries)
}

pub fn read_tree(tree_oid: &str) -> std::io::Result<()> {
    for entry in tree_entries(tree_oid)? {
        if let Some(directory) = Path::new(&entry.file).parent() {
            fs::create_dir_all(&directory).unwrap();
        }
        let mut file = fs::File::create(&entry.file)?;
        file.write_all(&data::get_object(&entry.oid, entry.otype))?;
    }
    Ok(())
}
