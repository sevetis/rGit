use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::Path;
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

}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.cmd {
        Commands::Init{path} => init(&path)?,
    }

    Ok(())
}

fn init(path: &str) -> Result<()> {
    let path = Path::new(path);
    let full_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    let final_path = full_path.join(".git");
    
    let reinit = final_path.exists();
    if reinit {
        fs::remove_dir_all(&final_path)?;
    }

    fs::create_dir_all(&final_path)?;
    create_git_struct(&final_path)?;

    if reinit {
        println!("Reinitialized existing Git repository in {}", final_path.display());
    } else {
        println!("Initialized empty Git repository in {}", final_path.display());
    }
    Ok(())
}

fn create_git_struct(path: &Path) -> Result<()> {
    fs::create_dir(path.join("objects"))?;
    fs::create_dir(path.join("refs"))?;
    fs::write(path.join("HEAD"), "ref: refs/heads/main\n")?;
    Ok(())
}
