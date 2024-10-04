#![allow(unused_variables)]

use anyhow::Result;
use clap::{Subcommand, ArgGroup};

use crate::repo::*;
use crate::objects::*;

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
        obj_sha: String,
    },
    WriteTree,
    RevParse {

    },
    ShowRef {

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
    todo!();
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
    if let Commands::CatFile { obj_sha, pretty_print, obj_size, obj_type } = args {
        if let Some(repo) = Repo::find_repo(".")? {
            let obj = repo.get_obj(&obj_sha)?;
            if pretty_print { obj.print()?; }
            else if obj_size { println!("{}", obj.size()?); }
            else if obj_type { println!("{}", obj.obj_type()?); }
        } else {
            return Err(anyhow::anyhow!("No rgit repository found!"));
        }
    }
    Ok(())
}

pub fn hash_object(args: Commands) -> Result<()> {
    if let Commands::HashObject { file_path, write } = args {
        let obj = Obj::new_blob(file_path)?;
        let (hex_sha, hashed) = obj.hash()?;
        if write {
            if let Some(repo) = Repo::find_repo(".")? {
                repo.write_obj(&hex_sha, &hashed)?;
            } else {
                return Err(anyhow::anyhow!("No rgit repository found!"));
            }
        }
        println!("{}", hex_sha);
    }
    Ok(())
}

pub fn list_tree(args: Commands) -> Result<()> {
    cat_file(args)
}

pub fn write_tree(args: Commands) -> Result<()> {
    todo!();
}

pub fn rev_parse(args: Commands) -> Result<()> {
    todo!();
}

pub fn show_ref(args: Commands) -> Result<()> {
    todo!();
}

pub fn tag(args: Commands) -> Result<()> {
    todo!();
}



