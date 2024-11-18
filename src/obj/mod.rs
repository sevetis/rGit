use sha1::{Sha1, Digest};
use anyhow::Result;

mod blob;
mod tree;
mod commit;

use blob::Blob;
use tree::Tree;
use commit::Commit;

#[derive(PartialEq)]
pub enum Type {
    Blob,
    Tree,
    Commit,
    Tag,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Blob =>   write!(f, "blob"),
            Type::Tree =>   write!(f, "tree"),
            Type::Commit => write!(f, "commit"),
            Type::Tag =>    write!(f, "tag"),
        }
    }
}


pub fn new_obj(raw_data: Vec<u8>) -> Result<Box<dyn Obj>> {
    match raw_data {
        _ if raw_data.starts_with(b"blob") =>   Blob::new(raw_data),
        _ if raw_data.starts_with(b"tree") =>   Tree::new(raw_data),
        _ if raw_data.starts_with(b"commit") => Commit::new(raw_data),
        _ => Err(anyhow::anyhow!("invalid object"))
    }
}


pub trait Obj {
    /// Obj to string
    fn to_string(&self) -> Result<String>;

    /// Obj type
    fn obj_type(&self) -> Type;

    /// Obj size
    fn size(&self) -> usize;

    /// Obj content
    fn content(&self) -> &Vec<u8>;

    /// Hash a Obj, Return hash code with corresponding content
    fn hash(&self) -> Result<(String, Vec<u8>)> {
        let header = format!("{} {}\0", self.obj_type(), self.size());
        let mut content = Vec::with_capacity(header.len() + self.size());
        content.extend_from_slice(header.as_bytes());
        content.extend_from_slice(self.content());
        let hex_sha = {
            let mut hasher = Sha1::new();
            hasher.update(&content);
            format!("{:x}", hasher.finalize())
        };
        Ok((hex_sha, content))
    }

    /// Return the hash code of the parent object of current object
    fn parent(&self) -> Option<String>;
}