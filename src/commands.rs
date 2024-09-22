use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use anyhow::{Result, Context};
use clap::Subcommand;
use std::path::Path;
use std::io::{Read, Write};
use std::fs;

use crate::objects::*;


#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Init {
        #[arg(default_value = "")]
        path: String
    },
    Add {
        files: Vec<String>  
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
    WriteTree,
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

pub fn add(args: Commands) -> Result<()> {

    Ok(())
}

pub fn cat_file(args: Commands) -> Result<()> {
    if let Commands::CatFile { object, .. } = args {
        let obj = decompress(&object)?;
        if &obj[..4] == b"tree" {
            print_tree_obj(obj, false)?;
        } else if &obj[..4] == b"blob" {
            print_blob_obj(obj)?;
        }
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

pub fn write_tree() -> Result<()> {

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


