use std::{fs, io::Write};

use sha1::{Sha1, Digest};

pub const GIT_DIR: &str = ".ugit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(format!("{}/objects", GIT_DIR))
}

pub fn hash_object(data: Vec<u8>) -> std::io::Result<String> {
    let mut hasher = Sha1::new();
    hasher.update(&data);
    let result = hasher.finalize();

    let string_representation = result
        .iter()
        .map(|h| format!("{:x?}", h))
        .collect::<Vec<String>>()
        .join("");
    
    let mut file = fs::File::create(
        format!("{}/objects/{}", GIT_DIR, string_representation)
    )?;
    file.write_all(&data)?;

    Ok(format!("{:x?}", string_representation))
}

pub fn get_object(object: &str) -> Vec<u8> {
    fs::read(format!("{}/objects/{}", GIT_DIR, object)).unwrap()
}
