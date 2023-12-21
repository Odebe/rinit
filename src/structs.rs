use crate::utils::{calc_hash};
use std::rc::Rc;

pub(crate) trait GitObject {
    fn content(&self) -> Rc<String>;
    fn hash(&self) -> Rc<String>;
}

#[derive(Debug)]
pub(crate) struct GitBlob {
    content: Rc<String>,
    hash: Rc<String>
}

#[derive(Debug)]
pub(crate) struct GitTree {
    refs: Vec<GitObjectRef>
}

#[derive(Debug)]
enum GitObjectRefType { Blob, Tree }

#[derive(Debug)]
pub(crate) struct GitObjectRef {
    // 100644 blob 2f781156939ad540b2434d012446154321e41e03	example_file.txt
    permissions: u32,
    ref_type: GitObjectRefType,
    hash: String,
    content: String
}

impl GitObjectRef {
    fn as_line(&self) -> String {
        format!("{} {:?} {} {}", self.permissions, self.ref_type, self.hash, self.content)
    }
}

impl GitBlob {
    pub(crate) fn new(content: String) -> Self {
        Self {
            hash: Rc::new(calc_hash(&content)),
            content: Rc::new(content),
        }
    }
}

impl GitObject for GitBlob {
    fn content(&self) -> Rc<String> { Rc::clone(&self.content) }
    fn hash(&self) -> Rc<String> { Rc::clone(&self.hash) }
}

impl GitObject for GitTree {
    fn content(&self) -> Rc<String> {
        let value =
            self.refs
                .iter()
                .map(|r| r.as_line())
                .collect::<Vec<_>>()
                .join("\n");

        Rc::new(value)
    }

    fn hash(&self) -> Rc<String> { Rc::new(calc_hash(&self.content())) }
}
