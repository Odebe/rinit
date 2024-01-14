use std::fmt;
use std::fmt::Display;

pub mod flags;
pub mod git_blob;
pub mod git_tree;
pub mod git_object;
pub mod git_index;

#[derive(Debug)]
pub enum GitObjectType { Blob, Tree }

impl Display for GitObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GitObjectType::Blob => write!(f, "blob"),
            GitObjectType::Tree => write!(f, "tree"),
        }
    }
}

impl GitObjectType {
    pub fn parse(data: &str) -> Self {
        match data {
            "blob" => GitObjectType::Blob,
            "tree" => GitObjectType::Tree,
            _ => panic!("not supported git object type")
        }
    }
}

