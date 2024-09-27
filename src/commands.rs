#![allow(unused_variables)]

use anyhow::{Result};
use clap::Subcommand;

use crate::repo::Repo;
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
    CatFile {
        #[arg(long = None, short = 'p', required = true)]
        pretty_print: bool,
        obj_sha: String,
    },
    HashObject {
        #[arg(short = 'w')]
        write: bool,
        file: String,
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
    if let Commands::Init { path } = args {
        let repo = Repo::new(path)?;
        repo.init()?;
    }
    Ok(())
}

pub fn add(args: Commands) -> Result<()> {

    Ok(())
}

pub fn commit(args: Commands) -> Result<()> {

    Ok(())
}

pub fn status(args: Commands) -> Result<()> {

    Ok(())
}

pub fn log(args: Commands) -> Result<()> {
    
    Ok(())
}

pub fn rm(args: Commands) -> Result<()> {

    Ok(())
}

pub fn checkout(args: Commands) -> Result<()> {

    Ok(())
}

pub fn check_ignore(args: Commands) -> Result<()> {

    Ok(())
}

pub fn cat_file(args: Commands) -> Result<()> {
    if let Commands::CatFile { obj_sha, .. } = args {
        let obj = Obj::new(obj_sha)?;
        obj.print()?;
    }
    Ok(())
}

pub fn hash_object(args: Commands) -> Result<()> {
    // if let Commands::HashObject { write, file } = args {
    //     let object = blob_sha1(&file)?;
    //     println!("{}", object);

    //     if write {
    //         let mut path = format!(
    //             ".git/objects/{}/",
    //             &object[..2],    
    //         );
    //         let dir = Path::new(&path);
    //         if !dir.exists() {
    //             fs::create_dir_all(&path)?;
    //         }
    //         path.push_str(&object[2..]);
    //         compress(&file, &path)?;
    //     }
    // }

    Ok(())
}

pub fn list_tree(args: Commands) -> Result<()> {
    // if let Commands::LsTree { obj_sha } = args {
    //     let obj = Obj::new(obj_sha)?;
    //     print_tree_obj(&obj.content, name_only)?;
    // }
    Ok(())
}

pub fn write_tree(args: Commands) -> Result<()> {

    Ok(())
}

pub fn rev_parse(args: Commands) -> Result<()> {

    Ok(())
}

pub fn show_ref(args: Commands) -> Result<()> {

    Ok(())
}

pub fn tag(args: Commands) -> Result<()> {

    Ok(())
}



