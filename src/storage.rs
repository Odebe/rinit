use std::path::{PathBuf};
use std::fs;
use std::io::{self, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::utils::files::{create_dir, create_file, read_file};
use crate::structs::*;

pub(crate) struct Storage {
    pub(crate) root: PathBuf
}

impl Storage {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { root: path.join(".rinit") }
    }

    fn objects_path(&self) -> PathBuf { self.root.join("objects") }
    fn info_path(&self) -> PathBuf { self.objects_path().join("info") }
    fn pack_path(&self) -> PathBuf { self.objects_path().join("pack") }
    fn index_path(&self) -> PathBuf { self.root.join("index") }

    pub(crate) fn init(&self) {
        create_dir(&self.root);
        create_dir(&self.root.join("objects/info"));
        create_dir(&self.root.join("objects/pack"));
    }

    pub(crate) fn persist_object(&self, object: &dyn GitObject) {
        let hash = object.hash();
        let (catalog, index) = hash.split_at(2);
        let obj_dir = self.objects_path().join(catalog);
        let file_path = obj_dir.join(index);

        let content = object.content();
        let header = format!("{} {}", "blob", content.len());
        let body = format!("{}\0{}", header, content);

        create_dir(&obj_dir);
        create_file(&file_path, &body);
    }

    pub(crate) fn read_object(&self, hash: &String) -> GitBlob {
        let (catalog, index) = hash.split_at(2);
        let obj_path = self.objects_path().join(catalog).join(index);

        GitBlob::new(read_file(obj_path))
    }

    pub(crate) fn read_index(&self) -> GitIndex {
        let path = self.index_path();

        if path.exists() {
            parse_git_index(&path).unwrap()
        } else {
            GitIndex { entries: Vec::new() }
        }
    }

    pub(crate) fn save_index(&self, index: &GitIndex) {
        write_git_index(&self.index_path(), index).unwrap();
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

const GIT_INDEX_HEADER: &[u8; 4] = b"DIRC";
const GIT_INDEX_VERSIONS: u32 = 2;

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

        if flags.contains(Flags::EXTENDED) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not supported entry flag (EXTENDED)",
            ));
        };

        let path =
            if flags.contains(Flags::PATH_LEN) {
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