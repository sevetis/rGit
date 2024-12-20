use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use anyhow::{Result, Context};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::fs;

use crate::obj::*;

pub struct Repo {
    git_dir: PathBuf,
}

impl Repo {
    /// Create a new repo struct
    pub fn new(path: String) -> Result<Self> {
        let path = fs::canonicalize(&path)?;
        let git_dir = path.join(".git");
        Ok(Repo { git_dir })
    }

    /// Init or reinit a repo
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

    /// Recursively find a repo from present working dir
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

    /// Get a object with given obj hash code 
    pub fn get_obj(&self, obj_sha: &str) -> Result<Box<dyn Obj>> {
        if obj_sha.len() == 40 {
            let obj_path = self.git_dir
                .join("objects")
                .join(&obj_sha[..2])
                .join(&obj_sha[2..]);
            if fs::metadata(&obj_path).is_ok() && obj_path.is_file() {
                let raw_data = decompress(&obj_path.to_string_lossy().to_owned())?;
                return new_obj(raw_data);
            }
        } else if obj_sha.len() >= 4 {
            let mut obj_path = self.git_dir
                .join("objects")
                .join(&obj_sha[..2]);
            let prefix = &obj_sha[2..];
            for entry in fs::read_dir(&obj_path)? {
                let filename = entry?.file_name();
                if filename.to_string_lossy().starts_with(&prefix) {
                    obj_path = obj_path.join(filename);
                    let raw_data = decompress(&obj_path.to_string_lossy())?;
                    return new_obj(raw_data);
                }
            }
        }
        Err(anyhow::anyhow!("invalid object"))
    }

    /// Create a object with given hash code and data in current repo
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

    /// Get the head reference of current branch
    pub fn head_ref(&self) -> Result<String> {
        Ok(self.get_ref("HEAD".into())?.unwrap())
    }

    /// Get the hash code of final ref found from given ref_path
    pub fn get_ref(&self, ref_path: String) -> Result<Option<String>> {
        let ref_path = self.git_dir.join(ref_path);
        if ref_path.is_file() {
            let data = fs::read(ref_path)?;
            if data.starts_with(&b"ref:".to_vec()[..]) {
                return self.get_ref(
                    std::str::from_utf8(&data[5..])?.trim_end().to_string()
                )
            } else {
                return Ok(Some(String::from_utf8(data)?.trim_end().to_string()));
            }
        }
        Ok(None)
    }

    /// Get all references
    pub fn all_refs(&self) -> Result<Vec<String>> {
        Ok(self.all_refs_(
            self.git_dir.join("refs")
        )?)
    }

    /// helper function of `all_refs`
    fn all_refs_(&self, dir: PathBuf) -> Result<Vec<String>> {
        let mut contents = vec![];
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_t = entry.file_type()?;
            if file_t.is_dir() {
                contents.extend((self.all_refs_(entry.path()))?);
            } else if file_t.is_file() {
                contents.push(entry.path().strip_prefix(&self.git_dir)?.to_string_lossy().into_owned());
            }
        }
        Ok(contents)
    }

}

/// Decompress a file
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

/// Compress data to the given path
fn compress(data: &Vec<u8>, output_path: &str) -> Result<()> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    let compressed = encoder.finish()?;
    let mut output = fs::File::create(output_path)?;
    output.write_all(&compressed)?;
    Ok(())
}


