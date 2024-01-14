use crate::commands::{CatFileArgs};
use crate::storage::Storage;

pub fn call(storage: Storage, args: CatFileArgs) {
    let object = storage.read_object(&args.hash.unwrap());

    println!("{:?}", object);
}
