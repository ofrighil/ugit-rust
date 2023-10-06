use std::{fs, env};

pub const GIT_DIR: &str = ".ugit";

pub fn init() -> std::io::Result<()> {
    fs::create_dir(GIT_DIR)?;
    println!(
        "Initialized empty ugit repository in {}/{}",
        env::current_dir().unwrap().to_str().unwrap(),
        GIT_DIR
    );
    Ok(())
}
