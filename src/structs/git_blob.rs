use std::rc::Rc;
use crate::structs::git_object::GitObject;
use crate::structs::GitObjectType;
use crate::utils::hash;

#[derive(Debug)]
pub struct GitBlob {
    content: Rc<String>,
    hash: Rc<String>
}

impl GitBlob {
    pub fn new(content: &str) -> Self {
        let ctn_string = content.to_string();

        Self {
            hash: Rc::new(hash::from_string(&ctn_string)),
            content: Rc::new(content.to_string()),
        }
    }
}

impl GitObject for GitBlob {
    fn content(&self) -> Rc<String> { Rc::clone(&self.content) }
    fn hash(&self) -> Rc<String> { Rc::clone(&self.hash) }
    fn git_type(&self) -> GitObjectType { GitObjectType::Blob }
}
