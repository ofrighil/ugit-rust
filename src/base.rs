use std::{fs, path::PathBuf};

use crate::data;

pub fn write_tree(directory: &str) {
    let dir = fs::read_dir(directory).unwrap();

    for entry in dir {
        let path = entry.unwrap().path();
        
        if is_ignored(&path) {
            continue
        }

        if path.is_symlink() {
            continue
        } else if path.is_file() {
            // todo!();
            let path_str: &str = path.to_str().unwrap();
            println!(
                "{}, {}",
                data::hash_object(
                    std::fs::read(path_str).unwrap(),
                    data::ObjectType::Blob
                ).unwrap(),
                path_str
            );
        } else if path.is_dir() {
            write_tree(path.as_os_str().to_str().unwrap());
        }
    }
}

fn is_ignored(path: &PathBuf) -> bool {
    path.to_str().unwrap().contains(".ugit")
}
