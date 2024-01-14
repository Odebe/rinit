pub mod hash_object;
pub mod update_index;
pub mod write_tree;
pub mod cat_file;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "rinit")]
#[command(about = "A fictional versioning CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init { },
    HashObject(HashObjectArgs),
    WriteTree(WriteTreeArgs),
    CatFile(CatFileArgs),
    UpdateIndex(UpdateIndexArgs),
}

#[derive(Debug, Args)]
pub struct UpdateIndexArgs {
    #[arg(long, default_value_t = false)]
    pub add: bool,
    #[arg(long, default_value_t = false)]
    pub cacheinfo: bool,

    pub mode: Option<u32>,
    pub sha1: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Args)]
pub struct WriteTreeArgs {
}

#[derive(Debug, Args)]
pub struct HashObjectArgs {
    #[arg(short, long, default_value_t = false)]
    pub stdin: bool,
    #[arg(short, default_value_t = false)]
    pub write: bool,

    #[arg(last = true)]
    pub filepath: Option<String>
}

#[derive(Debug, Args)]
pub struct CatFileArgs {
    #[arg(short, default_value_t = false)]
    pub p: bool,

    pub hash: Option<String>
}