#![allow(unused_imports)]
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use anyhow::{Result, Context};
use sha1::{Sha1, Digest};
use std::io::{Read, Write};
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
    pub fn new(obj_sha: String) -> Result<Self> {
        let raw_data = read_obj(&obj_sha)?;
        let obj_t = match raw_data {
            _ if raw_data.starts_with(b"blob") =>   Type::Blob,
            _ if raw_data.starts_with(b"tree") =>   Type::Tree,
            _ if raw_data.starts_with(b"commit") => Type::Commit,
            _ if raw_data.starts_with(b"tag") =>    Type::Tag,
            _ => return Err(anyhow::anyhow!("invalid object")),
        };

        if let Some(idx) = raw_data.iter().position(|&x| x == 0) {
            Ok(Self {
                obj_type: obj_t,
                content: raw_data[idx + 1..].to_vec(),
            })
        } else {
            Err(anyhow::anyhow!("corrupted object"))
        }
    }

    pub fn print(&self) -> Result<()> {
        match self.obj_type {
            Type::Blob => print_blob_obj(&self.content)?,
            Type::Tree => print_tree_obj(&self.content)?,
            _ => panic!("Unimplemented!"),
        };

        Ok(())
    }
}

fn read_obj(obj_sha: &str) -> Result<Vec<u8>> {
    const SHA1_LENGTH: usize = 40;
    if obj_sha.len() != SHA1_LENGTH {
        return Err(anyhow::anyhow!("Invalid object {}", obj_sha));
    }

    let obj_path = format!(".git/objects/{}/{}", &obj_sha[..2], &obj_sha[2..]);
    let obj_file = fs::File::open(obj_path)
        .with_context(|| format!(
            "Invalid object {}",
            obj_sha
        ))?;

    let mut decoder = ZlibDecoder::new(obj_file);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

// fn compress(input_path: &str, output_path: &str) -> Result<()> {
//     let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());

//     let blob = create_blob(input_path)?;
//     encoder.write_all(&blob)?;
//     let compressed = encoder.finish()?;

//     let mut output = fs::File::create(output_path)?;
//     output.write_all(&compressed)?;
//     Ok(())
// }


// pub fn create_blob(file_path: &str) -> Result<Vec<u8>> {
//     let mut file = fs::File::open(file_path)?;
//     let mut content = Vec::new();
//     file.read_to_end(&mut content)?;

//     let prefix = format!("blob {}\0", content.len());
//     let prefix_bytes = prefix.as_bytes();

//     let mut blob = Vec::with_capacity(
//         prefix_bytes.len() + content.len()
//     );

//     blob.extend_from_slice(prefix_bytes);
//     blob.extend(content);
//     Ok(blob)
// }

// pub fn blob_sha1(file: &str) -> Result<String> {
//     let mut hasher = Sha1::new();

//     let blob = create_blob(file)?;
//     hasher.update(blob);

//     let result = hasher.finalize();
//     let hex_code = format!("{:x}", result);
//     Ok(hex_code)
// }

pub fn print_blob_obj(content: &Vec<u8>) -> Result<()> {
    print!("{}", String::from_utf8(content.clone())?);
    Ok(())
}

pub fn print_tree_obj(data: &Vec<u8>) -> Result<()> {
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
