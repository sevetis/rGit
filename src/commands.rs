use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use anyhow::{Result, Context};
use clap::Subcommand;
use sha1::{Sha1, Digest};
use std::path::Path;
use std::io::{Read, Write};
use hex;
use std::fs;


#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Init {
        #[arg(default_value = "")]
        path: String
    },
    CatFile {
        #[arg(long = None, short = 'p', required = true)]
        pretty_print: bool,
        object: String,
    },
    HashObject {
        #[arg(short = 'w')]
        write: bool,
        file: String,
    },
    LsTree {
        #[arg(long = "name-only")]
        name_only: bool,
        object: String,
    },
}


pub fn init(path: &str) -> Result<()> {
    let path = Path::new(path);
    let full_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };

    let final_path = full_path.join(".git");
    if final_path.exists() {
        fs::remove_dir_all(&final_path)?;
    }

    fs::create_dir_all(&final_path)?;
    fs::create_dir(final_path.join("objects"))?;
    fs::create_dir(final_path.join("refs"))?;
    fs::write(
        final_path.join("HEAD"),
        "ref: refs/heads/main\n"
    )?;

    println!(
        "{} Git repository in {}",
        if final_path.exists() { "Reinitialized existing" }
        else { "Initialized empty" },
        final_path.display()
    );

    Ok(())
}

pub fn cat_file(args: Commands) -> Result<()> {
    if let Commands::CatFile { object, .. } = args {
        let blob = decompress(&object)?;
        let data = String::from_utf8(blob)?;
        let content = data.split('\0')
            .nth(1)
            .with_context(|| format!("Corrupted object {}", object))?;
        println!("{}", content);
    }

    Ok(())
}

pub fn hash_object(args: Commands) -> Result<()> {
    if let Commands::HashObject { write, file } = args {
        let object = blob_sha1(&file)?;
        println!("{}", object);

        if write {
            let mut path = format!(
                ".git/objects/{}/",
                &object[..2],    
            );
            let dir = Path::new(&path);
            if !dir.exists() {
                fs::create_dir_all(&path)?;
            }
            path.push_str(&object[2..]);
            compress(&file, &path)?;
        }
    }

    Ok(())
}

pub fn list_tree(args: Commands) -> Result<()> {
    if let Commands::LsTree { name_only, object } = args {
        let data = decompress(&object)?;
        print_tree_obj(data, name_only)?;
    }

    Ok(())
}



fn print_tree_obj(data: Vec<u8>, name_only: bool) -> Result<()> {
    const B_SHA1_LEN: usize = 20;
    if &data[..4] != b"tree" {
        return Err(anyhow::anyhow!("Not a tree object"));
    }

    // skip header
    let mut idx = 4;
    while data[idx + 1] != 0 {
        idx += 1;
    }
    
    while idx < data.len() {
        let mut mode = String::new();
        if data[idx] == b'4' { mode.push('0'); }
        while data[idx] != b' ' {
            mode.push(data[idx] as char);
            idx += 1;
        }
        idx += 1;

        let mut name = String::new();
        while data[idx] != 0 {
            name.push(data[idx] as char);
            idx += 1;
        }
        idx += 1;

        let sha_bytes = &data[idx..idx + B_SHA1_LEN];
        idx += B_SHA1_LEN;
    
        let obj_type = if mode == "040000" { "tree" } else { "blob" };
        if !name_only {
            println!("{} {} {}\t{}", mode, obj_type, hex::encode(&sha_bytes), name);
        } else {
            println!("{}", name);
        }
    }

    Ok(())
}

fn decompress(object: &str) -> Result<Vec<u8>> {
    const SHA1_LENGTH: usize = 40;
    if object.len() != SHA1_LENGTH {
        return Err(anyhow::anyhow!("Invalid object {}", object));
    }

    let obj_path = format!(
        ".git/objects/{}/{}",
        &object[..2],
        &object[2..]
    );

    let file = fs::File::open(obj_path)
        .with_context(|| format!(
            "Invalid object {}",
            object
        ))?;

    let mut decoder = ZlibDecoder::new(file);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)
        .context("Unable to read")?;
    Ok(decompressed)
}

fn compress(file: &str, output_path: &str) -> Result<()> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());

    let blob = create_blob(file)?;
    encoder.write_all(&blob)?;
    let compressed = encoder.finish()?;

    let mut output = fs::File::create(output_path)?;
    output.write_all(&compressed)?;
    Ok(())
}

fn create_blob(file_path: &str) -> Result<Vec<u8>> {
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

fn blob_sha1(file: &str) -> Result<String> {
    let mut hasher = Sha1::new();

    let blob = create_blob(file)?;
    hasher.update(blob);

    let result = hasher.finalize();
    let hex_code = format!("{:x}", result);
    Ok(hex_code)
}


