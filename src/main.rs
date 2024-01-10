mod storage;
mod structs;
mod utils;
mod commands;
mod formats;

use clap::Parser;

use crate::utils::files::{get_current_dir, read_stdin};
use crate::storage::{Storage};
use crate::structs::*;
use crate::commands::*;
use std::fs;

fn main() {
    let args = Cli::parse();
    let storage = Storage::new(get_current_dir());

    match args.command {
        Commands::Init { } => {
            do_init_command(storage)
        },
        Commands::HashObject(args) => {
            do_hash_object_command(storage, args)
        },
        Commands::CatFile(args) => {
            do_cat_file_command(storage, args)
        },
        Commands::UpdateIndex(args) => {
            if args.cacheinfo == true {
                do_update_index_command(storage, args)
            } else {
                println!("Only --cacheinfo implemented");
            }
        }
    }
}

// git update-index --add --cacheinfo 100644 83baae61804e65cc73a7201a7252750c76066a30 Cargo.lock
fn do_update_index_command(storage: Storage, args: UpdateIndexArgs) {
    let mut index = storage.read_index();
    let UpdateIndexArgs { mode, sha1, path, .. } = args;
    let entry = GitIndexEntry::from_path(path.unwrap(), mode, sha1);

    index.add_entry(entry);
    storage.save_index(&index);
}

fn do_init_command(storage: Storage) {
    storage.init();
    println!("Initialized empty rInit repository in {:?}", storage.root);
}

fn do_hash_object_command(storage: Storage, args: HashObjectArgs) {
    let content = read_stdin();
    let object = GitBlob::new(content.as_str());

    if args.write == true { storage.persist_object(&object); }

    println!("{:?}", object);
}

fn do_cat_file_command(storage: Storage, args: CatFileArgs) {
    let object = storage.read_object(&args.hash.unwrap());

    println!("{:?}", object);
}
