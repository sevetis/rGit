#![allow(unused_variables)]

use std::fs;

use anyhow::Result;
use clap::{Subcommand, ArgGroup};

use crate::repo::*;
use crate::obj::*;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Init {
        #[arg(default_value = ".")]
        repo_path: String
    },

    Add {
        files: Vec<String>  
    },

    Commit {

    },

    Status {

    },

    Log {
        
    },

    Rm {

    },

    Checkout {

    },

    CheckIgnore {

    },

    #[command(group(
        ArgGroup::new("flags")
            .required(true)
            .args(&["pretty_print", "obj_type", "obj_size"])
            .multiple(false)
    ))]
    CatFile {
        obj_sha: String,
        #[arg(long = None, short = 'p')]
        pretty_print: bool,
        #[arg(long = None, short = 't')]
        obj_type: bool,
        #[arg(long = None, short = 's')]
        obj_size: bool,
    },

    HashObject {
        file_path: String,
        #[arg(short = 'w')]
        write: bool,
    },

    LsTree {
        tree_sha: String,
    },

    WriteTree,

    RevParse {

    },

    ShowRef {
        #[arg(long = None, short = 's')]
        hash: bool,
    },

    Tag {

    }
}


pub fn init(args: Commands) -> Result<()> {
    if let Commands::Init { repo_path } = args {
        let repo = Repo::new(repo_path)?;
        repo.init()?;
    }
    Ok(())
}

pub fn add(args: Commands) -> Result<()> {
    todo!();
}

pub fn commit(args: Commands) -> Result<()> {
    todo!();
}

pub fn status(args: Commands) -> Result<()> {
    todo!();
}

pub fn log(args: Commands) -> Result<()> {
    if let Commands::Log{} = args {
        let repo = Repo::find_repo()?;
        let mut cur_commit = repo.get_obj(&repo.head_ref()?)?;
        print!("{}", cur_commit.to_string()?);
        while let Some(parent) = cur_commit.parent()? {
            cur_commit = repo.get_obj(&parent)?;
            println!("--------------------------------------------------");
            print!("{}", cur_commit.to_string()?);
        }
    }
    Ok(())
}

pub fn rm(args: Commands) -> Result<()> {
    todo!();
}

pub fn checkout(args: Commands) -> Result<()> {
    todo!();
}

pub fn check_ignore(args: Commands) -> Result<()> {
    todo!();
}

pub fn cat_file(args: Commands) -> Result<()> {
    if let Commands::CatFile{ obj_sha, pretty_print, obj_size, obj_type } = args {
        let repo = Repo::find_repo()?;
        let obj = repo.get_obj(&obj_sha)?;
        let to_print = if pretty_print {
            obj.to_string()?
        } else if obj_size {
            format!("{}", obj.size())
        } else {
            format!("{}", obj.obj_type())
        };
        println!("{}", to_print);
    }
    Ok(())
}

pub fn hash_object(args: Commands) -> Result<()> {
    if let Commands::HashObject{ file_path, write } = args {
        let obj = new_obj(fs::read(file_path)?)?;
        let (hex_sha, content) = obj.hash()?;
        if write {
            let repo = Repo::find_repo()?;
            repo.write_obj(&hex_sha, &content)?;
        }
        println!("{}", hex_sha);
    }
    Ok(())
}

pub fn list_tree(args: Commands) -> Result<()> {
    if let Commands::LsTree{ tree_sha } = args {
        let repo = Repo::find_repo()?;
        let tree = repo.get_obj(&tree_sha)?;
        if tree.obj_type() != Type::Tree {
            return Err(anyhow::anyhow!("not a tree object"));
        }
        println!("{}", tree.to_string()?);
    }
    Ok(())
}

pub fn write_tree(args: Commands) -> Result<()> {
    todo!();
}

pub fn rev_parse(args: Commands) -> Result<()> {
    todo!();
}

pub fn show_ref(args: Commands) -> Result<()> {
    if let Commands::ShowRef { hash } = args {
        let repo = Repo::find_repo()?;
        for ref_path in repo.all_refs()?.iter() {
            if let Some(sha) = repo.get_ref((*ref_path).clone())? {
                if hash {
                    println!("{}", sha);
                } else {
                    println!("{} {}", sha, ref_path);
                }
            }
        }
    }
    Ok(())
}

pub fn tag(args: Commands) -> Result<()> {
    todo!();
}



