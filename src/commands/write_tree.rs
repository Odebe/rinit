use crate::commands::{WriteTreeArgs};
use crate::storage::Storage;
use crate::structs::git_object::GitObject;
use crate::structs::git_tree::GitTree;

pub fn call(storage: Storage, _args: WriteTreeArgs) {
    let tree: GitTree = storage.read_index().into();
    storage.persist_object(&tree);

    println!("{}", tree.hash());
}
