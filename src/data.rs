use std::fs;

use sha1::{Sha1, Digest};

pub const GIT_DIR: &str = ".ugit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(format!("{}/objects", GIT_DIR))
}

pub fn hash_object(data: Vec<u8>) -> std::io::Result<String> {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();

    let string_representation = result
        .iter()
        .map(|h| format!("{:x?}", h))
        .collect::<Vec<String>>()
        .join("");
    
    fs::File::create(
        format!("{}/objects/{}", GIT_DIR, string_representation)
    )?;

    Ok(format!("{:x?}", string_representation))
}
