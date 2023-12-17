use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::env::current_dir;
use std::fmt::format;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::rc::Rc;

use clap::{Args, Parser, Subcommand, ValueEnum};
use sha2::{Sha256, Digest};
use sha2::digest::Output;

use std::io::prelude::*;
use flate2::Compression;
use flate2::write::{ZlibEncoder, ZlibDecoder};

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

struct Storage {
    root: PathBuf
}

trait GitObject {
    fn content(&self) -> Rc<String>;
    fn hash(&self) -> Rc<String>;
}

#[derive(Debug)]
struct GitBlob {
    content: Rc<String>,
    hash: Rc<String>
}

#[derive(Debug)]
struct GitTree {
    refs: Vec<GitObjectRef>
}

#[derive(Debug)]
enum GitObjectRefType { Blob, Tree }

#[derive(Debug)]
struct GitObjectRef {
    // 100644 blob 2f781156939ad540b2434d012446154321e41e03	example_file.txt
    permissions: u16,
    ref_type: GitObjectRefType,
    hash: String,
    content: String
}

impl GitObjectRef {
    fn as_line(&self) -> String {
        format!("{} {:?} {} {}", self.permissions, self.ref_type, self.hash, self.content)
    }
}

impl GitBlob {
    fn new(content: String) -> Self {
        Self {
            hash: Rc::new(calc_hash(&content)),
            content: Rc::new(content),
        }
    }
}

impl GitObject for GitBlob {
    fn content(&self) -> Rc<String> { Rc::clone(&self.content) }
    fn hash(&self) -> Rc<String> { Rc::clone(&self.hash) }
}

impl GitObject for GitTree {
    fn content(&self) -> Rc<String> {
        let value =
            self.refs
                .iter()
                .map(|r| r.as_line())
                .collect::<Vec<_>>()
                .join("\n");

        Rc::new(value)
    }

    fn hash(&self) -> Rc<String> { Rc::new(calc_hash(&self.content())) }
}

impl Storage {
    fn new(path: PathBuf) -> Self {
        Self { root: path.join(".rinit") }
    }

    fn objects_path(&self) -> PathBuf { self.root.join("objects") }
    fn info_path(&self) -> PathBuf { self.objects_path().join("info") }
    fn pack_path(&self) -> PathBuf { self.objects_path().join("pack") }

    fn init(&self) {
        create_dir(&self.root);
        create_dir(&self.root.join("objects/info"));
        create_dir(&self.root.join("objects/pack"));
    }

    fn persist_object(&self, object: &dyn GitObject) {
        let hash = object.hash();
        let (catalog, index) = hash.split_at(2);

        let obj_dir = self.objects_path().join(catalog);

        create_dir(&obj_dir);
        create_file(&obj_dir.join(index), &object.content());
    }

    fn read_object(&self, hash: &String) -> GitBlob {
        let (catalog, index) = hash.split_at(2);
        let obj_path = self.objects_path().join(catalog).join(index);

        GitBlob::new(read_file(obj_path))
    }
}

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
        }
    }
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

fn create_file(file_path: &PathBuf, content: &str) {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(content.as_ref())
        .expect("Can't compress content");

    let compressed_bytes = e.finish().unwrap();
    let mut file = fs::File::create(file_path)
        .expect("Can't create file");

    file.write_all(&*compressed_bytes)
        .expect("Can't write compressed content to file");
}

fn read_file(path: PathBuf) -> String {
    let data = fs::read(path).unwrap();
    let mut writer = Vec::new();
    let mut z = ZlibDecoder::new(writer);

    z.write_all(&data[..]).unwrap();
    writer = z.finish().unwrap();

    String::from_utf8(writer).expect("String parsing error")
}

fn calc_hash(content: &String) -> String {
    let mut hasher = Sha256::new();

    hasher.update(content);
    format!("{:X}", hasher.finalize())
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
