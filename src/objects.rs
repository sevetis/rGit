#![allow(unused_imports)]
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use anyhow::{Result, Context};
use sha1::{Sha1, Digest};
use std::io::{Read, Write};
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

    pub fn new(obj_path: String) -> Result<Self> {
        let raw_data = decompress(&obj_path)?;
        let obj_t = match raw_data {
            _ if raw_data.starts_with(b"blob") =>   Type::Blob,
            _ if raw_data.starts_with(b"tree") =>   Type::Tree,
            _ if raw_data.starts_with(b"commit") => Type::Commit,
            _ if raw_data.starts_with(b"tag") =>    Type::Tag,
            _ => return Err(anyhow::anyhow!("Invalid object")),
        };

        if let Some(idx) = raw_data.iter().position(|&x| x == 0) {
            Ok(Self {
                obj_type: obj_t,
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

    pub fn print(&self) -> Result<()> {
        match self.obj_type {
            Type::Blob => print_blob_obj(&self.content)?,
            Type::Tree => print_tree_obj(&self.content)?,
            _ => panic!("Unimplemented!"),
        };

        Ok(())
    }

    pub fn hash(&self, write: bool) -> Result<String> {
        let header = format!(
            "{} {}\0",
            &self.obj_type,
            &self.content.len()
        );
        let header = header.as_bytes();

        let mut result = Vec::with_capacity(
            &header.len() + &self.content.len()
        );
        result.extend_from_slice(header);
        result.extend(self.content.clone());

        let mut hasher = Sha1::new();
        hasher.update(result.clone());
        let hex_sha = format!("{:x}", hasher.finalize());

        if write {
            let mut path = format!(".git/objects/{}/", &hex_sha[..2]);
            let dir = Path::new(&path);
            if !dir.exists() {
                fs::create_dir_all(&path)?;
            }
            path.push_str(&hex_sha[2..]);
            compress(&result, &path)?;
        }

        Ok(hex_sha)
    }

}

fn decompress(file_path: &str) -> Result<Vec<u8>> {
    let file = fs::File::open(file_path)
        .with_context(|| format!(
            "Open file failed",
        ))?;

    let mut decoder = ZlibDecoder::new(file);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

fn compress(data: &Vec<u8>, output_path: &str) -> Result<()> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    let compressed = encoder.finish()?;
    let mut output = fs::File::create(output_path)?;
    output.write_all(&compressed)?;
    Ok(())
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
