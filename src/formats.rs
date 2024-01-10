pub mod serialization {
    use crate::structs::{GitObject};

    pub fn call(object: &dyn GitObject) -> String {
        let content = object.content();
        let header = format!("{} {}", object.type_string(), content.len());

        format!("{}\0{}", header, content)
    }
}

pub mod deserialization {
    use crate::structs::{GitObject, GitBlob, GitTree};

    pub fn call(data: String) -> Box<dyn GitObject> {
        let parts: Vec<&str> = data.split("\0").take(2).collect();
        let object: Box<dyn GitObject> =
            match parts[..] {
                [header, content] => {
                    let header_parts: Vec<&str> = header.split(" ").take(2).collect();
                    match header_parts.as_slice() {
                        ["blob", _bytesize] => Box::new(GitBlob::new(content)),
                        ["tree", _bytesize] => Box::new(GitTree::new(content)),
                        _ => panic!("invalid git object header")
                    }
                }
                _ => panic!("invalid git object")
            };

        object
    }
}
