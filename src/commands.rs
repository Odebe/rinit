use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "rinit")]
#[command(about = "A fictional versioning CLI", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Init { },
    HashObject(HashObjectArgs),
    CatFile(CatFileArgs),
    UpdateIndex(UpdateIndexArgs),
}

#[derive(Debug, Args)]
pub(crate) struct UpdateIndexArgs {
    #[arg(long, default_value_t = false)]
    pub(crate) add: bool,
    #[arg(long, default_value_t = false)]
    pub(crate) cacheinfo: bool,

    pub(crate) mode: Option<u32>,
    pub(crate) sha1: Option<String>,
    pub(crate) path: Option<String>,
}


#[derive(Debug, Args)]
pub(crate) struct HashObjectArgs {
    #[arg(short, long, default_value_t = false)]
    pub(crate) stdin: bool,
    #[arg(short, default_value_t = false)]
    pub(crate) write: bool
}

#[derive(Debug, Args)]
pub(crate) struct CatFileArgs {
    #[arg(short, default_value_t = false)]
    pub(crate) p: bool,

    pub(crate) hash: Option<String>
}