use std::{fs, path::PathBuf};

use crate::data;

pub fn write_tree(directory: &str) -> String {
    let dir = fs::read_dir(directory).unwrap();

    let mut entries: Vec<(String, String, data::ObjectType)> = vec!();

    for entry in dir {
        let path = entry.unwrap().path();
        
        if is_ignored(&path) {
            continue
        }

        if path.is_symlink() {
            continue
        } else if path.is_file() {
            let path_str = path.to_str().unwrap();
            let oid = data::hash_object(
                &std::fs::read(path_str).unwrap(),
                data::ObjectType::Blob
            ).unwrap();
            entries.push(
                (
                    path.to_str().unwrap().to_owned(),
                    oid,
                    data::ObjectType::Tree
                )
            );
        } else if path.is_dir() {
            let oid = write_tree(path.as_os_str().to_str().unwrap());
            entries.push(
                (
                    path.to_str().unwrap().to_owned(),
                    oid,
                    data::ObjectType::Tree
                )
            );
        }
    }

    let tree = entries
        .iter()
        .map(
            |(name, oid, t)| format!("{} {} {}", name, oid, t.as_string())
        )
        .collect::<Vec<String>>()
        .join("\n");

    data::hash_object(tree.as_bytes(), data::ObjectType::Tree).unwrap()
}

fn is_ignored(path: &PathBuf) -> bool {
    path.to_str().unwrap().contains(".ugit")
}
