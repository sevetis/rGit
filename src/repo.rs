use std::path::{Path, PathBuf};
use anyhow::{Result};
use std::fs;

pub struct Repo {
    git_dir: PathBuf,
}

impl Repo {
    pub fn new(path: String) -> Result<Self> {
        let path = Path::new(&path);
        let abs_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };

        Ok(Repo {
            git_dir: abs_path.join(".git"),
        })
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
}
