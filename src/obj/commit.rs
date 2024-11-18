
use crate::obj::*;

pub struct Commit {
    content: Vec<u8>,
    parent: Option<String>,
}

impl Commit {
    /// Create a new commit object
    pub fn new(raw_data: Vec<u8>) -> Result<Box<dyn Obj>> {
        if let Some(idx) = raw_data
            .iter()
            .position(|&x| x == 0)
        {
            Self::parse_commit(raw_data[idx + 1..].to_vec())
        } else {
            Err(anyhow::anyhow!("invalid commit object"))
        }
    }

    /// helper function for new function
    fn parse_commit(content: Vec<u8>) -> Result<Box<dyn Obj>> {
        let lines: Vec<String> = content
            .split(|&del| del == b'\n')
            .map(|line| String::from_utf8_lossy(line).to_string())
            .collect();

        let mut parent = None;
        lines.iter().for_each(|line| {
            let idx = line.find(' ').unwrap();
            let (label, stuff) = line.split_at(idx);
            if label == "parent" {
                parent = Some(stuff.to_string());
            }
        });

        return Ok(Box::new(Self {
            content,
            parent
        }));
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
    fn parent(&self) -> Option<String> {
        self.parent.clone()
    }
}