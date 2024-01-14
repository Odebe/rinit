mod storage;
mod utils;
mod formats;
mod commands;
mod structs;

use clap::Parser;

use crate::utils::files::{get_current_dir};
use crate::storage::{Storage};
use crate::commands::{Commands, Cli};

fn main() {
    let args = Cli::parse();
    let storage = Storage::new(get_current_dir());

    match args.command {
        Commands::Init { } => {
            storage.init();

            println!("Initialized empty rInit repository in {:?}", storage.root());
        },
        Commands::HashObject(args) => {
            commands::hash_object::call(storage, args)
        },
        Commands::WriteTree(args) => {
            commands::write_tree::call(storage, args)
        },
        Commands::CatFile(args) => {
            commands::cat_file::call(storage, args)
        },
        Commands::UpdateIndex(args) => {
            if args.cacheinfo == true {
                commands::update_index::call(storage, args);
            } else {
                println!("Only --cacheinfo implemented");
            }
        }
    }
}
