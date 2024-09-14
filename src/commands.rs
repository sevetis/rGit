use flate2::read::ZlibDecoder;
use anyhow::{Result, Context};
use clap::Subcommand;
use std::path::Path;
use std::io::Read;
use std::fs;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Init {
        #[arg(default_value = "")]
        path: String
    },
    CatFile {
        #[arg(long = None, short = 'p', required = true)]
        p: bool,
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
            let data = read_blob(&object)?;
            let content = data.split('\0')
                .nth(1)
                .with_context(|| format!("Corrupted object {}", object))?;
            print!("{}", content);
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


fn read_blob(object: &str) -> Result<String> {
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
