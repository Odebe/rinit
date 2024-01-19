use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use crate::formats::tree;
use crate::structs::git_index::{GitIndex, GitIndexEntry};
use crate::structs::git_object::GitObject;
use crate::structs::GitObjectType;
use crate::utils::hash;

#[derive(Debug)]
pub struct GitObjectRef {
    // 100644 blob 2f781156939ad540b2434d012446154321e41e03	example_file.txt
    pub permissions: u32,
    pub ref_type: GitObjectType,
    pub hash: String,
    pub content: String
}

#[derive(Debug)]
pub struct GitTree {
    pub refs: Vec<GitObjectRef>
}

impl GitTree {
    pub fn new(content: &str) -> Self {
        tree::parse(content)
    }
}

impl GitObject for GitTree {
    fn content(&self) -> Rc<String> {
        let value =
            self.refs
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
                .join("\n");

        Rc::new(value)
    }
    fn hash(&self) -> Rc<String> { Rc::new(hash::from_string(&self.content())) }
    fn git_type(&self) -> GitObjectType { GitObjectType::Tree }
}

impl Display for GitObjectRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.permissions, self.ref_type, self.hash, self.content)
    }
}

impl From<&GitIndexEntry> for GitObjectRef {
    fn from(item: &GitIndexEntry) -> Self {
        GitObjectRef {
            permissions: item.mode,
            ref_type: GitObjectType::Blob,
            hash: String::from_utf8(Vec::from(item.hash)).unwrap(),
            content: item.path.to_string()
        }
    }
}

impl From<GitIndex> for GitTree {
    fn from(item: GitIndex) -> Self {
        let refs =
            item.entries
                .iter()
                .map(|e| e.into())
                .collect::<Vec<GitObjectRef>>();

        GitTree { refs }
    }
}
