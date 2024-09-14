use clap::Parser;
use anyhow::Result;

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
        Commands::CatFile { .. } => commands::cat_file(args.cmd)?,
    }

    Ok(())
}
