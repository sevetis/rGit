use clap::Parser;
use anyhow::Result;

mod objects;
mod commands;
use commands::Commands;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}


fn main() -> Result<()> {
    let args = Args::parse();
    match args.cmd {
        Commands::Init { path } => commands::init(&path)?,
        Commands::Add { .. } => commands::add(args.cmd)?,
        Commands::CatFile { .. } => commands::cat_file(args.cmd)?,
        Commands::HashObject { .. } => commands::hash_object(args.cmd)?,
        Commands::LsTree { .. } => commands::list_tree(args.cmd)?,
        Commands::WriteTree => commands::write_tree()?,
    }

    Ok(())
}

