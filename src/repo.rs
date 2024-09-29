use std::path::{PathBuf};
use anyhow::{Result};
use std::fs;

use crate::objects::Obj;

pub struct Repo {
    git_dir: PathBuf,
}

impl Repo {
    pub fn new(path: String) -> Result<Self> {
        let path = fs::canonicalize(&path)?;
        let git_dir = path.join(".git");
        Ok(Repo { git_dir })
    }

    pub fn init(&self) -> Result<()> {
        let is_overwrite = self.git_dir.exists();
        if is_overwrite {
            fs::remove_dir_all(&self.git_dir)?;
        }

        fs::create_dir_all(&self.git_dir)?;
        fs::create_dir(self.git_dir.join("objects"))?;
        fs::create_dir(self.git_dir.join("refs"))?;

        fs::write(
            self.git_dir.join("HEAD"),
            "ref: refs/heads/main\n"
        )?;
        fs::write(
            self.git_dir.join("description"),
            "Unnamed repository; edit this file 'description' \
            to name the repository\n"
        )?;

        println!(
            "{} rGit repository in {}",
            if is_overwrite { "Reinitialized existing" } 
            else { "Initialized empty" },
            self.git_dir.display()
        );

        Ok(())
    }

    pub fn get_obj(&self, obj_sha: String) -> Result<Obj> {
        let obj_path = self.git_dir
            .join("objects")
            .join(&obj_sha[..2])
            .join(&obj_sha[2..]);

        if fs::metadata(&obj_path).is_ok() && obj_path.is_file() {
            Obj::new(obj_path.to_string_lossy().into_owned())
        } else {
            Err(anyhow::anyhow!("Invalid Object"))
        }
    }
}

pub fn find_repo(path: &str) -> Result<Option<Repo>> {
    let mut cur_dir = fs::canonicalize(&path)?;
    while let Some(parent_dir) = cur_dir.parent() {
        let target = cur_dir.join(".git");
        if fs::metadata(&target).is_ok() && target.is_dir() {
            return Ok(Some(
                Repo {
                    git_dir: target,
                }
            ));
        }
        cur_dir = parent_dir.to_path_buf();
    }
    Ok(None)
}
