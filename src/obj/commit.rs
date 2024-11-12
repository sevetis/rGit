
use crate::obj::*;

pub struct Commit {
    content: Vec<u8>,
}

impl Commit {
    /// Create a new commit object
    pub fn new(raw_data: Vec<u8>) -> Result<Box<dyn Obj>> {
        if let Some(idx) = raw_data
            .iter()
            .position(|&x| x == 0)
        {
            return Ok(Box::new(Self {
                content: raw_data[idx + 1..].to_vec(),
            }));
        }
        Err(anyhow::anyhow!("invalid commit object"))
    }
}

impl Obj for Commit {
    fn obj_type(&self) -> Type {
        Type::Commit
    }

    fn size(&self) -> usize {
        self.content.len()
    }

    fn content(&self) -> &Vec<u8> {
        &self.content
    }

    fn to_string(&self) -> Result<String> {
        Ok(String::from_utf8_lossy(&self.content).into_owned())
    }

    // TODO: refine function
    fn parent(&self) -> Result<Option<String>> {
        let content = self.to_string()?;
        let lines: Vec<&str> = content.split('\n').collect();
        if lines[1].starts_with("parent") {
            let parent_sha = lines[1][7..].to_owned();
            Ok(Some(parent_sha))
        } else {
            Ok(None)
        }
    }
}