use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

use crate::structs::flags::Flags;
use crate::utils::hash;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct GitIndex {
    pub entries: Vec<GitIndexEntry>
}

const GIT_INDEX_HEADER: &[u8; 4] = b"DIRC";
const GIT_INDEX_VERSIONS: u32 = 2;

impl GitIndex {
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        parse_git_index(&path.into()).unwrap()
    }

    pub fn empty() -> Self {
        Self { entries: vec![] }
    }

    pub fn add_entry(&mut self, entry: GitIndexEntry) {
        // TODO: sorting and stuff
        self.entries.push(entry);
    }

    pub fn persist(&self, path: impl Into<PathBuf>) {
        write_git_index(&path.into(), self).unwrap();
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

        let mut flags = Flags::empty();
        let len = binding.as_path().to_str().unwrap().len();
        flags = Flags::from_bits_retain(len as u16 & 0x0fff);

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
            flags,
            path: path_.display().to_string()
        }
    }
}

fn parse_entry_path<R: Read>(reader: &mut R) -> io::Result<String> {
    let mut string = String::new();
    loop {
        let byte = reader.read_u8()?;
        if byte == 0 {
            break;
        }
        string.push(byte as char);
    }

    Ok(string)
}

fn write_entry_path<W: Write>(writer: &mut W, s: &str) -> io::Result<()> {
    writer.write_all(s.as_bytes())?;
    writer.write_u8(0)?;
    Ok(())
}

fn write_git_index_entry<W: Write>(writer: &mut W, entry: &GitIndexEntry) -> io::Result<()> {
    writer.write_u32::<BigEndian>(entry.ctime_seconds)?;
    writer.write_u32::<BigEndian>(entry.ctime_nanoseconds)?;
    writer.write_u32::<BigEndian>(entry.mtime_seconds)?;
    writer.write_u32::<BigEndian>(entry.mtime_nanoseconds)?;
    writer.write_u32::<BigEndian>(entry.dev)?;
    writer.write_u32::<BigEndian>(entry.ino)?;
    writer.write_u32::<BigEndian>(entry.mode)?;
    writer.write_u32::<BigEndian>(entry.uid)?;
    writer.write_u32::<BigEndian>(entry.gid)?;
    writer.write_u32::<BigEndian>(entry.size)?;
    writer.write_all(&entry.hash)?;
    writer.write_u16::<BigEndian>(entry.flags.as_u16())?;
    write_entry_path(writer, &entry.path)?;

    Ok(())
}

fn write_git_index(file_path: &PathBuf, index: &GitIndex) -> io::Result<()> {
    let mut file = fs::File::create(file_path)?;

    // Write the header
    file.write_all(GIT_INDEX_HEADER)?;
    file.write_u32::<BigEndian>(GIT_INDEX_VERSIONS)?; // Index version
    file.write_u32::<BigEndian>(index.entries.len() as u32)?; // Entry count

    // Write entries
    for entry in &index.entries {
        write_git_index_entry(&mut file, entry)?;
    }

    Ok(())
}

fn parse_git_index(file_path: &PathBuf) -> io::Result<GitIndex> {
    let mut file = fs::File::open(file_path)?;
    let mut header = [0u8; 4];
    file.read_exact(&mut header)?;

    if &header != GIT_INDEX_HEADER {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not a valid Git index file",
        ));
    }

    let version = file.read_u32::<BigEndian>()?;

    if version != GIT_INDEX_VERSIONS {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not a valid Git index file version",
        ));
    }

    let entry_count = file.read_u32::<BigEndian>()?;

    let mut entries = Vec::with_capacity(entry_count as usize);
    for _ in 0..entry_count {
        let ctime_seconds = file.read_u32::<BigEndian>()?;
        let ctime_nanoseconds = file.read_u32::<BigEndian>()?;
        let mtime_seconds = file.read_u32::<BigEndian>()?;
        let mtime_nanoseconds = file.read_u32::<BigEndian>()?;
        let dev = file.read_u32::<BigEndian>()?;
        let ino = file.read_u32::<BigEndian>()?;
        let mode = file.read_u32::<BigEndian>()?;
        let uid = file.read_u32::<BigEndian>()?;
        let gid = file.read_u32::<BigEndian>()?;
        let size = file.read_u32::<BigEndian>()?;

        let mut hash = [0u8; 20];
        file.read_exact(&mut hash)?;

        let raw_flags = file.read_u16::<BigEndian>()?;
        let flags = Flags::from_bits_retain(raw_flags);

        if flags.intersects(Flags::EXTENDED) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not supported entry flag (EXTENDED)",
            ));
        };

        let path =
            if flags.intersects(Flags::PATH_LEN) {
                parse_entry_path(&mut file)?
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not supported entry path size",
                ));
            };

        let entry = GitIndexEntry {
            ctime_seconds,
            ctime_nanoseconds,
            mtime_seconds,
            mtime_nanoseconds,
            dev,
            ino,
            mode,
            uid,
            gid,
            size,
            hash,
            flags,
            path,
        };

        entries.push(entry);
    }

    Ok(GitIndex { entries })
}