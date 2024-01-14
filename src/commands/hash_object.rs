use crate::commands::{HashObjectArgs};
use crate::storage::Storage;
use crate::structs::git_blob::GitBlob;
use crate::utils::files::{read_stdin};

pub fn call(storage: Storage, args: HashObjectArgs) {
    let content =
        if args.stdin == true {
            read_stdin()
        } else {
            storage.read_file(args.filepath.unwrap())
        };

    let object = GitBlob::new(content.as_str());

    if args.write == true { storage.persist_object(&object); }

    println!("{:?}", object);
}
