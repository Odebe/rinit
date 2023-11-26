use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::env::current_dir;
use std::fmt::format;
use std::fs;
use std::io;
use std::io::{Read, Write};

use clap::{Args, Parser, Subcommand, ValueEnum};
use sha2::{Sha256, Digest};
use sha2::digest::Output;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "rinit")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init { },
    HashObject(HashObjectArgs),
    CatFile(CatFileArgs)
}

#[derive(Debug, Args)]
struct HashObjectArgs {
    #[arg(short, long, default_value_t = false)]
    stdin: bool,
    #[arg(short, default_value_t = false)]
    write: bool
}

#[derive(Debug, Args)]
struct CatFileArgs {
    #[arg(short, default_value_t = false)]
    p: bool,

    hash: Option<String>
}

fn main() {
    let args = Cli::parse();
    let current_dir = get_current_dir();

    match args.command {
        Commands::Init { } => {
            do_init_command(current_dir)
        },
        Commands::HashObject(_) => {
            do_hash_object_command(current_dir)
        },
        Commands::CatFile(args) => {
            do_cat_file_command(current_dir, args)
        }
    }
}

fn do_init_command(path: PathBuf) {
    let rinit_path = path.join(".rinit");

    create_dir(&rinit_path);
    create_dir(&rinit_path.join("objects/info"));
    create_dir(&rinit_path.join("objects/pack"));

    println!("Initialized empty rInit repository in {:?}", path);
}

fn do_hash_object_command(path: PathBuf) {
    let content = read_stdin();
    let hash = calc_content_hash(&content);

    persist_object(&content, &hash, path);

    println!("{}", hash);
}

fn do_cat_file_command(path: PathBuf, args: CatFileArgs) {
    let content = read_object(path, args.hash.unwrap());

    println!("{}", content);
}

fn read_object(path: PathBuf, hash: String) -> String {
    let (catalog, index) = hash.split_at(2);
    let rinit_path = path.join(".rinit");
    let obj_path =
        rinit_path
            .join("objects/info/")
            .join(catalog)
            .join(index);

    read_file(obj_path)
}

fn persist_object(content: &String, hash: &String, path: PathBuf) {
    let (catalog, index) = hash.split_at(2);

    let rinit_path = path.join(".rinit");
    let obj_dir =
        rinit_path
            .join("objects/info/")
            .join(catalog);

    create_dir(&obj_dir);
    create_obj_file(&obj_dir.join(index), content);
}

fn create_obj_file(file_path: &PathBuf, content: &String) {
    let mut file = fs::File::create(file_path).expect("calc_content_hash: panic message");

    file.write_all(content.as_ref()).expect("TODO: panic message");
}

fn calc_content_hash(content: &String) -> String {
    let mut hasher = Sha256::new();

    hasher.update(content);
    format!("{:X}", hasher.finalize())
}

fn read_file(path: PathBuf) -> String {
    fs::read_to_string(path).unwrap()
}

fn read_stdin() -> String {
    let mut content = String::new();

    io::stdin()
        .read_to_string(&mut content)
        .expect("read_stdin: panic message");

    content
}

fn create_dir(path: &Path) {
    fs::create_dir_all(path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

// TODO
fn get_current_dir() -> PathBuf { current_dir().unwrap() }

