#![allow(unused_imports)]
use flate2::Compression;
use anyhow::{Result, Context};
use sha1::{Sha1, Digest};
use std::path::Path;
use std::fs;
use hex;

#[derive(Copy, Clone, PartialEq)]
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

    pub fn to_string(&self) -> Result<String> {
        match self.obj_type {
            Type::Blob |
            Type::Commit => Ok(String::from_utf8_lossy(&self.content).into_owned()),
            Type::Tree =>   tree_to_string(&self.content),
            _ => todo!(),
        }
    }

    pub fn size(&self) -> usize {
        self.content.len()
    }

    pub fn obj_type(&self) -> Type {
        self.obj_type
    }

    // only for commit
    pub fn parent(&self) -> Result<Option<String>> {
        if let Type::Commit = self.obj_type {
            let content = std::str::from_utf8(&self.content)?;
            let lines: Vec<&str> = content.split("\n").collect();
            if lines[1].starts_with("parent") {
                let parent_sha = lines[1][7..].to_owned();
                Ok(Some(parent_sha))
            } else {
                Ok(None)
            }
        } else {
            Err(anyhow::anyhow!("invalid call"))
        }
    }

}

fn tree_to_string(data: &Vec<u8>) -> Result<String> {
    const B_SHA1_LEN: usize = 20;
    let cut = |data: &Vec<u8>, st: &mut usize, del: u8| -> Result<String> {
        let ed = data[(*st)..].iter()
            .position(|&x| x == del)
            .unwrap();
        let ret = String::from_utf8(data[*st..*st + ed].to_vec())?;
        *st += ed + 1;
        Ok(ret)
    };

    let mut idx = 0;
    let mut ret = String::new();
    while idx < data.len() {
        let mode = cut(data, &mut idx, b' ')?;
        let name = cut(data, &mut idx, 0)?;
        let hex_ = hex::encode(&data[idx..idx + B_SHA1_LEN]);
        let obj_type = if mode == "40000" { "tree" } else { "blob" };
        
        idx += B_SHA1_LEN;
        ret = format!(
            "{}{:0>6} {} {}\t{}\n",
            ret, mode, obj_type, hex_, name
        );
    }

    ret.pop();
    Ok(ret)
}



