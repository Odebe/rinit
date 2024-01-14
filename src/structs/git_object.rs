use std::fmt::Debug;
use std::rc::Rc;
use crate::structs::GitObjectType;

pub trait GitObject:Debug {
    fn content(&self) -> Rc<String>;
    fn hash(&self) -> Rc<String>;
    fn git_type(&self) -> GitObjectType;
}
