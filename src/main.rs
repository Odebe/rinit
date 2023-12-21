mod storage;
mod structs;
mod utils;
mod commands;

use clap::Parser;

use crate::utils::{get_current_dir, read_stdin};
use crate::storage::{Storage};
use crate::structs::*;
use crate::commands::*;

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
            do_update_index_command(storage, args)
        }
    }
}

fn do_update_index_command(storage: Storage, args: UpdateIndexArgs) {
    // TODO
}

fn do_init_command(storage: Storage) {
    storage.init();
    println!("Initialized empty rInit repository in {:?}", storage.root);
}

fn do_hash_object_command(storage: Storage, args: HashObjectArgs) {
    let content = read_stdin();
    let object = GitBlob::new(content);

    if args.write == true { storage.persist_object(&object); }

    println!("{:?}", object);
}

fn do_cat_file_command(storage: Storage, args: CatFileArgs) {
    let object = storage.read_object(&args.hash.unwrap());

    println!("{:?}", object);
}
