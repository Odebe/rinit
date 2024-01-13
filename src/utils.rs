pub mod hash {
    use std::path::{PathBuf};
    use crate::utils::files;
    use sha2::{Sha256, Digest};

    pub fn from_path(path: impl Into<PathBuf>) -> String {
        let content = files::read_object_file(path.into());
        from_string(&content)
    }

    pub fn from_string(content: &String) -> String {
        let mut hasher = Sha256::new();

        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }
}

pub mod files {
    use std::env::current_dir;
    use std::path::{Path, PathBuf};
    use std::fs;
    use std::io::{self, Read, Write};
    use flate2::write::{ZlibEncoder, ZlibDecoder};
    use flate2::Compression;

    pub fn create_file(file_path: &PathBuf, content: &str) {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(content.as_ref())
            .expect("Can't compress content");

        let compressed_bytes = e.finish().unwrap();
        let mut file = fs::File::create(file_path)
            .expect("Can't create file");

        file.write_all(&*compressed_bytes)
            .expect("Can't write compressed content to file");
    }

    pub fn create_dir(path: &Path) {
        fs::create_dir_all(path).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    pub fn read_object_file(path: PathBuf) -> String {
        let data = fs::read(path).unwrap();
        let mut writer = Vec::new();
        let mut z = ZlibDecoder::new(writer);

        z.write_all(&data[..]).unwrap();
        writer = z.finish().unwrap();

        String::from_utf8(writer).expect("String parsing error")
    }

    // TODO
    pub fn get_current_dir() -> PathBuf {
        current_dir().unwrap()
    }

    pub fn read_stdin() -> String {
        let mut content = String::new();

        io::stdin()
            .read_to_string(&mut content)
            .expect("read_stdin: panic message");

        content
    }

    pub fn read_file(path: impl Into<PathBuf> + AsRef<Path>) -> String {
        let data = fs::read(path).unwrap();
        String::from_utf8(data).unwrap()
    }
}
