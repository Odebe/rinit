mod storage;
mod structs;
mod utils;
mod commands;
mod formats;
mod cmd_update_index;
mod cmd_hash_object;
mod cmd_write_tree;

use clap::Parser;

use crate::utils::files::{get_current_dir};
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
            cmd_hash_object::call(storage, args)
        },
        Commands::WriteTree(args) => {
            cmd_write_tree::call(storage, args)
        },
        Commands::CatFile(args) => {
            do_cat_file_command(storage, args)
        },
        Commands::UpdateIndex(args) => {
            if args.cacheinfo == true {
                cmd_update_index::call(storage, args);
            } else {
                println!("Only --cacheinfo implemented");
            }
        }
    }
}

fn do_init_command(storage: Storage) {
    storage.init();
    println!("Initialized empty rInit repository in {:?}", storage.root());
}

fn do_cat_file_command(storage: Storage, args: CatFileArgs) {
    let object = storage.read_object(&args.hash.unwrap());

    println!("{:?}", object);
}
