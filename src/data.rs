use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
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

pub fn update_ref(ref_name: &str, oid: &str) -> std::io::Result<()> {
    let ref_path = format!("{}/{}", GIT_DIR, ref_name);
    fs::create_dir_all(Path::new(&ref_path).parent().unwrap())?;
    let mut file = fs::File::create(ref_path)?;
    file.write_all(oid.as_bytes())?;
    Ok(())
}

pub fn get_ref(ref_name: &str) -> Option<String> {
    let ref_path = format!("{}/{}", GIT_DIR, ref_name);
    if Path::new(&ref_path).try_exists().unwrap() {
        let value = io::BufReader::new(fs::File::open(&ref_path).unwrap())
            .lines()
            .take(1)
            .next()
            .unwrap()
            .ok();

        if let Some(r) = value.clone() {
            if r.starts_with("ref:") {
                get_ref(r.split(':').nth(1).unwrap())
            } else {
                value
            }
        } else {
            value
        }
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

    Ok(format!("{:x?}", string_representation.replace("\"", "")))
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

fn get_refs(directory: &Path) -> Vec<String> {
    let dir = fs::read_dir(directory).unwrap();

    let mut entries: Vec<String> = vec![];

    for entry in dir {
        let path = entry.unwrap().path().to_owned();
        if path.is_file() {
            entries.push(path.file_stem().unwrap().to_str().unwrap().to_string());
        } else {
            entries.extend(get_refs(&path));
        }
    }

    entries
}

pub fn refs() -> Vec<String> {
    let mut ref_list = vec!["HEAD".to_string()];
    ref_list.extend(get_refs(&Path::new(&format!("{}/refs", GIT_DIR))));

    ref_list
}
