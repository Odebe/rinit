use sha2::{Sha256, Digest};
use std::env::current_dir;
use std::io;
use std::io::Read;
use std::path::{PathBuf};

pub(crate) fn calc_hash(content: &String) -> String {
    let mut hasher = Sha256::new();

    hasher.update(content);
    format!("{:X}", hasher.finalize())
}

// TODO
pub(crate) fn get_current_dir() -> PathBuf {
    current_dir().unwrap()
}

pub(crate) fn read_stdin() -> String {
    let mut content = String::new();

    io::stdin()
        .read_to_string(&mut content)
        .expect("read_stdin: panic message");

    content
}
