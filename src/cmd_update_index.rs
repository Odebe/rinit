use crate::commands::UpdateIndexArgs;
use crate::storage::Storage;
use crate::structs::{GitBlob, GitIndexEntry};
use crate::utils::files::read_file;

// git update-index --add --cacheinfo 100644 83baae61804e65cc73a7201a7252750c76066a30 Cargo.lock
pub fn call(storage: Storage, args: UpdateIndexArgs) {
    if args.add == true { add_entry(storage, args) }
}

fn add_entry(storage: Storage, args: UpdateIndexArgs) {
    let mut index = storage.read_index();
    let UpdateIndexArgs { mode, sha1, path, add, .. } = args;
    let entry = GitIndexEntry::from_path(path.unwrap(), mode, sha1);

    println!("{:?}", entry);

    index.add_entry(entry);
    storage.save_index(&index);
}
