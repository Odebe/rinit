use std::fmt::Debug;
use crate::utils::{hash};
use std::rc::Rc;
use crate::structs::GitObjectRefType::Blob;
use std::fs;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use bitflags::bitflags;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct GitBlob {
    content: Rc<String>,
    hash: Rc<String>
}

#[derive(Debug)]
pub(crate) struct GitTree {
    refs: Vec<GitObjectRef>
}

pub trait GitObject:Debug {
    fn content(&self) -> Rc<String>;
    fn hash(&self) -> Rc<String>;
    fn type_string(&self) -> String;
}

#[derive(Debug)]
enum GitObjectRefType { Blob, Tree }

#[derive(Debug)]
pub(crate) struct GitObjectRef {
    // 100644 blob 2f781156939ad540b2434d012446154321e41e03	example_file.txt
    permissions: u32,
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
    pub fn new(content: &str) -> Self {
        let ctn_string = content.to_string();

        Self {
            hash: Rc::new(hash::from_string(&ctn_string)),
            content: Rc::new(content.to_string()),
        }
    }
}

impl GitTree {
    pub(crate) fn new(content: &str) -> Self {
        Self { refs: vec![] }
    }
}

pub struct GitIndexEntry {
    pub ctime_seconds: u32,
    pub ctime_nanoseconds: u32,
    pub mtime_seconds: u32,
    pub mtime_nanoseconds: u32,
    pub dev: u32,
    pub ino: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u32,
    pub hash: [u8; 20],
    pub flags: Flags,
    pub path: String,
}

pub struct GitIndex {
    pub entries: Vec<GitIndexEntry>
}

impl GitIndex {
    pub fn add_entry(&mut self, entry: GitIndexEntry) {
        // TODO: sorting and stuff
        self.entries.push(entry);
    }
}

impl GitIndexEntry {
    pub fn from_path(
        path: impl Into<PathBuf>,
        mode: Option<u32>,
        sha1: Option<String>
    ) -> Self {
        let binding = path.into();

        let meta= fs::metadata(&binding).unwrap();
        let hash = sha1.unwrap_or_else(|| hash::from_path(&binding));
        let mode = mode.unwrap_or_else(|| meta.mode());

        let path_ = binding.as_path();
        let hash_bytes : [u8; 20] = hash.as_bytes()[0..20].try_into().unwrap();

        GitIndexEntry {
            ctime_seconds: meta.ctime() as u32,
            ctime_nanoseconds: meta.ctime_nsec() as u32,
            mtime_seconds: meta.mtime() as u32,
            mtime_nanoseconds: meta.mtime_nsec() as u32,
            dev: meta.dev() as u32,
            ino: meta.ino() as u32,
            mode,
            uid: meta.uid(),
            gid: meta.gid(),
            size: meta.size() as u32,
            hash: hash_bytes,
            flags: Flags::empty(),
            path: path_.display().to_string()
        }
    }
}

impl From<&GitIndexEntry> for GitObjectRef {
    fn from(item: &GitIndexEntry) -> Self {
        let file = fs::read(&item.path).unwrap();
        let content = String::from_utf8(file).expect("String parsing error");

        GitObjectRef {
            permissions: item.mode,
            ref_type: Blob,
            hash: hash::from_string(&content),
            content
        }
    }
}

bitflags! {
    /// Flags how they are serialized to a storage location
    #[derive(Copy, Clone, Debug)]
    pub struct Flags: u16 {
        /// A portion of a the flags that encodes the length of the path that follows.
        const PATH_LEN = 0x0fff;
        const STAGE_MASK = 0x3000;
        /// If set, there is more extended flags past this one
        const EXTENDED = 0x4000;
        /// If set, the entry be assumed to match with the version on the working tree, as a way to avoid `lstat()`  checks.
        const ASSUME_VALID = 0x8000;
    }
}

impl Flags {
    pub fn as_u16(&self) -> u16 {
        self.bits() as u16
    }
}

impl From<GitIndex> for GitTree {
    fn from(item: GitIndex) -> Self {
        let refs =
            item.entries
                .iter()
                .map(|e| e.into())
                .collect::<Vec<GitObjectRef>>();

        GitTree { refs }
    }
}

impl GitObject for GitBlob {
    fn content(&self) -> Rc<String> { Rc::clone(&self.content) }
    fn hash(&self) -> Rc<String> { Rc::clone(&self.hash) }
    fn type_string(&self) -> String { "blob".to_string() }
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

    fn hash(&self) -> Rc<String> { Rc::new(hash::from_string(&self.content())) }

    fn type_string(&self) -> String { "tree".to_string() }
}
