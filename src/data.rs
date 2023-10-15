use std::{fs, env};

pub const GIT_DIR: &str = ".ugit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(format!("{}/objects", GIT_DIR))
}
