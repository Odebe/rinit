use crate::commands::UpdateIndexArgs;
use crate::storage::Storage;
use crate::structs::git_index::GitIndexEntry;

// git update-index --add --cacheinfo 100644 83baae61804e65cc73a7201a7252750c76066a30 Cargo.lock
pub fn call(storage: Storage, args: UpdateIndexArgs) {
    if args.add == true { add_entry(storage, args) }
}

fn add_entry(storage: Storage, args: UpdateIndexArgs) {
    let mut index = storage.read_index();
    let UpdateIndexArgs { mode, sha1, path, .. } = args;
    let entry = GitIndexEntry::from_path(path.unwrap(), mode, sha1);

    index.add_entry(entry);
    storage.save_index(&index);

    println!("{:?}", index);
}
