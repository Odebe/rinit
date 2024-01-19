use std::path::{PathBuf};
use crate::utils::files::{create_dir, create_file, read_object_file, read_file};
use crate::formats::{serialization, deserialization};
use crate::structs::git_index::GitIndex;
use crate::structs::git_object::GitObject;

pub struct Storage {
    pub working_root: PathBuf
}

impl Storage {
    pub fn new(path: PathBuf) -> Self {
        Self { working_root: path }
    }

    pub(crate) fn root(&self) -> PathBuf { self.working_root.join(".rinit") }
    fn objects_path(&self) -> PathBuf { self.root().join("objects") }
    fn info_path(&self) -> PathBuf { self.objects_path().join("info") }
    fn pack_path(&self) -> PathBuf { self.objects_path().join("pack") }
    fn index_path(&self) -> PathBuf { self.root().join("index") }

    pub fn object_path(&self, hash: &str) -> PathBuf {
        let (catalog, index) = hash.split_at(2);
        self.objects_path().join(catalog).join(index)
    }

    pub fn object_exists(&self, hash: &str) -> bool {
        self.object_path(hash).exists()
    }

    pub fn init(&self) {
        create_dir(&self.root());
        create_dir(&self.root().join("objects/info"));
        create_dir(&self.root().join("objects/pack"));
    }

    pub fn persist_object(&self, object: &dyn GitObject) {
        let hash = object.hash();
        let (catalog, _index) = hash.split_at(2);
        let obj_dir = self.objects_path().join(catalog);
        let file_path = self.object_path( &object.hash());
        let body = serialization::call(object);

        create_dir(&obj_dir);
        create_file(&file_path, &body);
    }

    pub fn read_object(&self, hash: &String) -> Box<dyn GitObject> {
        let (catalog, index) = hash.split_at(2);
        let obj_path = self.objects_path().join(catalog).join(index);

        deserialization::call(read_object_file(obj_path))
    }

    pub fn read_index(&self) -> GitIndex {
        let path = self.index_path();

        if path.exists() {
            GitIndex::from_path(&path)
        } else {
            GitIndex::empty()
        }
    }

    pub fn save_index(&self, index: &GitIndex) {
        index.persist(self.index_path());
    }

    pub fn read_file(&self, path: String) -> String {
        read_file(self.working_root.join(path))
    }
}