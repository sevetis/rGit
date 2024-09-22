use anyhow::{Result, Context};
use sha1::{Sha1, Digest};
use std::io::Read;
use std::fs;
use hex;

pub fn create_blob(file_path: &str) -> Result<Vec<u8>> {
    let mut file = fs::File::open(file_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let prefix = format!("blob {}\0", content.len());
    let prefix_bytes = prefix.as_bytes();

    let mut blob = Vec::with_capacity(
        prefix_bytes.len() + content.len()
    );

    blob.extend_from_slice(prefix_bytes);
    blob.extend(content);
    Ok(blob)
}

pub fn blob_sha1(file: &str) -> Result<String> {
    let mut hasher = Sha1::new();

    let blob = create_blob(file)?;
    hasher.update(blob);

    let result = hasher.finalize();
    let hex_code = format!("{:x}", result);
    Ok(hex_code)
}

pub fn print_blob_obj(data: Vec<u8>) -> Result<()> {
    let data = String::from_utf8(data)?;
    let content = data.split('\0')
        .nth(1)
        .context("Corrupted blob object")?;
    println!("{}", content);
    Ok(())
}


pub fn print_tree_obj(data: Vec<u8>, name_only: bool) -> Result<()> {
    const B_SHA1_LEN: usize = 20;
    if &data[..4] != b"tree" {
        return Err(anyhow::anyhow!("Not a tree object"));
    }

    // skip header
    let mut idx = data.iter()
        .position(|&x| x == 0)
        .unwrap() + 1;
    
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
        if !name_only {
            println!(
                "{:0>6} {} {}\t{}",
                mode,
                obj_type,
                hex::encode(&sha_bytes),
                name
            );
        } else {
            println!("{}", name);
        }
    }

    Ok(())
}
