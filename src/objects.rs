#![allow(unused_imports)]
use flate2::Compression;
use anyhow::{Result, Context};
use sha1::{Sha1, Digest};
use std::path::Path;
use std::fs;
use hex;

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

pub struct Obj {
    obj_type: Type,
    content: Vec<u8>,
}

impl Obj {
    pub fn new(raw_data: Vec<u8>) -> Result<Self> {
        let obj_type = match raw_data {
            _ if raw_data.starts_with(b"blob") =>   Type::Blob,
            _ if raw_data.starts_with(b"tree") =>   Type::Tree,
            _ if raw_data.starts_with(b"commit") => Type::Commit,
            _ if raw_data.starts_with(b"tag") =>    Type::Tag,
            _ => return Err(anyhow::anyhow!("Invalid object")),
        };

        if let Some(idx) = raw_data.iter().position(|&x| x == 0) {
            Ok(Self {
                obj_type,
                content: raw_data[idx + 1..].to_vec(),
            })
        } else {
            Err(anyhow::anyhow!("Corrupted object"))
        }
    }

    pub fn new_blob(file_path: String) -> Result<Self> {
        Ok(Self {
            obj_type: Type::Blob,
            content: fs::read(file_path)?,
        })
    }

    pub fn hash(&self) -> Result<(String, Vec<u8>)> {
        let header = format!("{} {}\0", &self.obj_type, &self.content.len());
        let mut result = Vec::with_capacity(header.len() + self.content.len());
        result.extend_from_slice(header.as_bytes());
        result.extend_from_slice(&self.content);

        let hex_sha = {
            let mut hasher = Sha1::new();
            hasher.update(&result);
            format!("{:x}", hasher.finalize())
        };

        Ok((hex_sha, result))
    }

    pub fn print(&self) -> Result<()> {
        match self.obj_type {
            Type::Blob => print_blob_obj(&self.content)?,
            Type::Tree => print_tree_obj(&self.content)?,
            _ => todo!(),
        };

        Ok(())
    }

}


fn print_blob_obj(content: &Vec<u8>) -> Result<()> {
    print!("{}", String::from_utf8(content.clone())?);
    Ok(())
}

fn print_tree_obj(data: &Vec<u8>) -> Result<()> {
    const B_SHA1_LEN: usize = 20;

    let mut idx = 0;
    while idx < data.len() {
        let mode_end = data[idx..].iter()
            .position(|&x| x == b' ')
            .unwrap();
        let mode = String::from_utf8_lossy(&data[idx..idx + mode_end]);
        idx += mode_end + 1;

        let name_end = data[idx..].iter()
            .position(|&x| x == 0)
            .unwrap();
        let name = String::from_utf8_lossy(&data[idx..idx + name_end]);
        idx += name_end + 1;

        let sha_bytes = &data[idx..idx + B_SHA1_LEN];
        idx += B_SHA1_LEN;
    
        let obj_type = if mode == "40000" { "tree" } else { "blob" };

        println!(
            "{:0>6} {} {}\t{}",
            mode,
            obj_type,
            hex::encode(&sha_bytes),
            name
        );

    }

    Ok(())
}
