use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use anyhow::{Result, Context};
use std::io::{Read, Write};
use std::path::PathBuf;
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

    pub fn find_repo() -> Result<Self> {
        let mut cur_dir = fs::canonicalize(".")?;
        while let Some(parent_dir) = cur_dir.parent() {
            let target = cur_dir.join(".git");
            if fs::metadata(&target).is_ok() && target.is_dir() {
                return Ok(Repo { git_dir: target });
            }
            cur_dir = parent_dir.to_path_buf();
        }
        Err(anyhow::anyhow!("no rgit repository found"))
    }

    pub fn get_obj(&self, obj_sha: &str) -> Result<Obj> {
        let obj_path = self.git_dir
            .join("objects")
            .join(&obj_sha[..2])
            .join(&obj_sha[2..]);

        if fs::metadata(&obj_path).is_ok() && obj_path.is_file() {
            let raw_data = decompress(&obj_path.to_string_lossy().to_owned())?;
            Obj::new(raw_data)
        } else {
            Err(anyhow::anyhow!("invalid object"))
        }
    }

    pub fn write_obj(&self, obj_sha: &str, data: &Vec<u8>) -> Result<()> {
        let mut storing_path = self.git_dir
            .join("objects").
            join(&obj_sha[..2]);

        if !storing_path.exists() {
            fs::create_dir(&storing_path)?;
        }
        storing_path = storing_path.join(&obj_sha[2..]);
        compress(data, &storing_path.to_string_lossy().to_owned())?;
        Ok(())
    }

    pub fn head_ref(&self) -> Result<String> {
        let head = fs::read_to_string(self.git_dir.join("HEAD"))?;
        let ref_path = self.git_dir.join(&head[5..].trim_end());
        if ref_path.exists() {
            Ok(fs::read_to_string(ref_path)?.trim_end().to_string())
        } else {
            Err(anyhow::anyhow!("current branch does not have any commits yet"))
        }
    }

}

fn decompress(file_path: &str) -> Result<Vec<u8>> {
    let file = fs::File::open(file_path)
        .with_context(|| format!(
            "Open file failed",
        ))?;

    let mut decoder = ZlibDecoder::new(file);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

fn compress(data: &Vec<u8>, output_path: &str) -> Result<()> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    let compressed = encoder.finish()?;
    let mut output = fs::File::create(output_path)?;
    output.write_all(&compressed)?;
    Ok(())
}

