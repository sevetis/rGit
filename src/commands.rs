use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use anyhow::{Result, Context};
use clap::Subcommand;
use sha1::{Sha1, Digest};
use std::path::Path;
use std::io::{Read, Write};
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
    }
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
    create_git_struct(&final_path)?;

    println!(
        "{} Git repository in {}",
        if final_path.exists() { "Reinitialized existing" }
        else { "Initialized empty" },
        final_path.display()
    );

    Ok(())
}

pub fn cat_file(args: Commands) -> Result<()> {
    match args {
        Commands::CatFile { object, .. } => {
            let data = decompress(&object)?;
            let content = data.split('\0')
                .nth(1)
                .with_context(|| format!("Corrupted object {}", object))?;
            print!("{}", content);
        },
        _ => {}
    }
    Ok(())
}

pub fn hash_object(args: Commands) -> Result<()> {
    match args {
        Commands::HashObject { write, file } => {
            let object = compute_sha1(&file)?;
            print!("{}", object);

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
        },
        _ => {}
    }
    Ok(())
}


fn create_git_struct(path: &Path) -> Result<()> {
    fs::create_dir(path.join("objects"))?;
    fs::create_dir(path.join("refs"))?;
    fs::write(
        path.join("HEAD"),
        "ref: refs/heads/main\n"
    )?;
    Ok(())
}

fn decompress(object: &str) -> Result<String> {
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
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed)?;

    Ok(decompressed)
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

fn compute_sha1(file: &str) -> Result<String> {
    let blob = create_blob(file)?;

    let mut hasher = Sha1::new();
    hasher.update(blob);
    let result = hasher.finalize();

    let hex = format!("{:x}", result);
    Ok(hex)
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


