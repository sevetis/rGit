use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use anyhow::{Result, Context};
use std::path::Path;
use std::io::Read;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
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

fn main() -> Result<()> {
    let args = Args::parse();
    match args.cmd {
        Commands::Init { path } => cmd_init(&path)?,
        Commands::CatFile { .. } => cmd_cat_file(args.cmd)?,
    }

    Ok(())
}

fn cmd_init(path: &str) -> Result<()> {
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

fn create_git_struct(path: &Path) -> Result<()> {
    fs::create_dir(path.join("objects"))?;
    fs::create_dir(path.join("refs"))?;
    fs::write(path.join("HEAD"), "ref: refs/heads/main\n")?;
    Ok(())
}


fn cmd_cat_file(args: Commands) -> Result<()> {
    match args {
        Commands::CatFile { object, .. } => {
            let data = read_blob(&object)?;
            let content = data.split('\0')
                .nth(1)
                .with_context(|| format!("Currpted object {}", object))?;
            print!("{}", content);
        },
        _ => {}
    }
    Ok(())
}

fn read_blob(object: &str) -> Result<String> {
    let obj_path = format!(
        ".git/objects/{}/{}",
        &object[..2],
        &object[2..]
    );

    let file = fs::File::open(obj_path)
        .with_context(|| format!(
            "Not a valid object {}",
            object
        ))?;

    let mut decoder = ZlibDecoder::new(file);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed)?;

    Ok(decompressed)
}
