pub mod serialization {
    use crate::structs::git_object::GitObject;

    pub fn call(object: &dyn GitObject) -> String {
        let content = object.content();
        let header = format!("{} {}", object.git_type().to_string(), content.len());

        format!("{}\0{}", header, content)
    }
}

pub mod deserialization {
    use crate::formats::tree;
    use crate::structs::git_blob::GitBlob;
    use crate::structs::git_object::GitObject;

    pub fn call(data: String) -> Box<dyn GitObject> {
        let parts: Vec<&str> = data.split("\0").take(2).collect();
        let object: Box<dyn GitObject> =
            match parts[..] {
                [header, content] => {
                    let header_parts: Vec<&str> = header.split(" ").take(2).collect();
                    match header_parts.as_slice() {
                        ["blob", _bytesize] => Box::new(GitBlob::new(content)),
                        ["tree", _bytesize] => Box::new(tree::parse(content)),
                        _ => panic!("invalid git object header")
                    }
                }
                _ => panic!("invalid git object")
            };

        object
    }
}

pub mod tree {
    use crate::formats::object_ref;
    use crate::structs::git_tree::GitTree;

    pub fn parse(data: &str) -> GitTree {
        GitTree { refs: data.lines().map(object_ref::parse).collect() }
    }
}

pub mod object_ref {
    use crate::structs::git_tree::GitObjectRef;
    use crate::structs::GitObjectType;

    // 100644 blob 2f781156939ad540b2434d012446154321e41e03	example_file.txt
    pub fn parse(line: &str) -> GitObjectRef {
        let parts: Vec<&str> = line.split(" ").collect();
        let fragments: [&str; 4] = parts[0..=3].try_into().unwrap();
        let raw_ref_type = fragments[1].parse::<String>().unwrap();

        GitObjectRef {
            permissions: fragments[0].parse().unwrap(),
            ref_type: GitObjectType::parse(&raw_ref_type.as_str()),
            hash: fragments[2].parse().unwrap(),
            content: fragments[3].parse().unwrap(),
        }
    }
}
