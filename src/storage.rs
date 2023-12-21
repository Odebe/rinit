use std::path::{Path, PathBuf};
use flate2::Compression;
use flate2::write::{ZlibEncoder, ZlibDecoder};
use std::fs;
use std::io::{Read, Write};

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

    pub(crate) fn init(&self) {
        create_dir(&self.root);
        create_dir(&self.root.join("objects/info"));
        create_dir(&self.root.join("objects/pack"));
    }

    pub(crate) fn persist_object(&self, object: &dyn GitObject) {
        let hash = object.hash();
        let (catalog, index) = hash.split_at(2);

        let obj_dir = self.objects_path().join(catalog);

        create_dir(&obj_dir);
        create_file(&obj_dir.join(index), &object.content());
    }

    pub(crate) fn read_object(&self, hash: &String) -> GitBlob {
        let (catalog, index) = hash.split_at(2);
        let obj_path = self.objects_path().join(catalog).join(index);

        GitBlob::new(read_file(obj_path))
    }
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

fn create_dir(path: &Path) {
    fs::create_dir_all(path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

