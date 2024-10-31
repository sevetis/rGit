use clap::Parser;
use anyhow::Result;

pub mod obj;

mod repo;
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
        Commands::Init {..} =>        commands::init(args.cmd)?,
        Commands::Add {..} =>         commands::add(args.cmd)?,
        Commands::Commit {..} =>      commands::commit(args.cmd)?,
        Commands::Status {..} =>      commands::status(args.cmd)?,
        Commands::Log {..} =>         commands::log(args.cmd)?,
        Commands::Rm {..} =>          commands::rm(args.cmd)?,
        Commands::Checkout {..} =>    commands::checkout(args.cmd)?,
        Commands::CheckIgnore {..} => commands::check_ignore(args.cmd)?,
        Commands::CatFile {..} =>     commands::cat_file(args.cmd)?,
        Commands::HashObject {..} =>  commands::hash_object(args.cmd)?,
        Commands::LsTree {..} =>      commands::list_tree(args.cmd)?,
        Commands::WriteTree {..} =>   commands::write_tree(args.cmd)?,
        Commands::RevParse {..} =>    commands::rev_parse(args.cmd)?,
        Commands::ShowRef {..} =>     commands::show_ref(args.cmd)?,
        Commands::Tag {..} =>         commands::tag(args.cmd)?,
    }

    Ok(())
}

