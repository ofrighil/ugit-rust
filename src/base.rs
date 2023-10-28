use std::{fs, path::PathBuf};

pub fn write_tree(directory: &str) {
    let dir = fs::read_dir(directory).unwrap();

    for entry in dir {
        let path = entry.unwrap().path();
        
        if is_ignored(&path) {
            continue
        }

        if path.is_file() {
            // todo!();
            println!("{}", path.to_str().unwrap());
        } else if path.is_dir() {
            write_tree(path.as_os_str().to_str().unwrap());
        }
    }
}

fn is_ignored(path: &PathBuf) -> bool {
    path.to_str().unwrap().contains(".ugit")
}
