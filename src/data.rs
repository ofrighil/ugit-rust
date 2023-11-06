use std::{
    fs,
    io::{self, BufRead, Write},
    path,
};

use sha1::{Digest, Sha1};

pub const GIT_DIR: &str = ".ugit";

#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Blob,
    Commit,
    Tree,
}

impl ObjectType {
    pub fn as_string(&self) -> &'static str {
        match self {
            ObjectType::Commit => "commit",
            ObjectType::Blob => "blob",
            ObjectType::Tree => "tree",
        }
    }

    pub fn from_string(s: &str) -> ObjectType {
        match s {
            "commit" => ObjectType::Commit,
            "blob" => ObjectType::Blob,
            "tree" => ObjectType::Tree,
            _ => panic!(),
        }
    }

    fn as_bytes(&self) -> &'static [u8] {
        self.as_string().as_bytes()
    }
}

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(format!("{}/objects", GIT_DIR))
}

#[allow(non_snake_case)]
pub fn set_HEAD(oid: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(format!("{}/HEAD", GIT_DIR))?;
    file.write_all(oid.as_bytes())?;
    Ok(())
}

#[allow(non_snake_case)]
pub fn get_HEAD() -> Option<String> {
    let HEAD = format!("{}/HEAD", GIT_DIR);
    if path::Path::new(&HEAD).try_exists().unwrap() {
        io::BufReader::new(fs::File::open(&HEAD).unwrap())
            .lines()
            .take(1)
            .next()
            .unwrap()
            .ok()
    } else {
        None
    }
}

pub fn hash_object(data: &[u8], otype: ObjectType) -> std::io::Result<String> {
    let saved_data = [&otype.as_bytes(), &[0u8].as_slice(), data].concat();

    let mut hasher = Sha1::new();
    hasher.update(&saved_data);
    let result = hasher.finalize();

    let string_representation = result
        .iter()
        .map(|h| format!("{:x?}", h))
        .collect::<Vec<String>>()
        .join("");

    let mut file = fs::File::create(format!("{}/objects/{}", GIT_DIR, string_representation))?;
    file.write_all(&saved_data)?;

    Ok(format!("{:x?}", string_representation))
}

pub fn get_object(object: &str, expected: ObjectType) -> Vec<u8> {
    let content = fs::read(format!("{}/objects/{}", GIT_DIR, object)).unwrap();

    let saved_data = content.split(|&b| b == 0u8).collect::<Vec<_>>();

    let actual = std::str::from_utf8(saved_data[0]);
    if let Ok(res) = actual {
        assert!(
            ObjectType::from_string(res) == expected,
            "Expected {}, got {}",
            expected.as_string(),
            res
        );
    }

    saved_data[1].to_vec()
}
