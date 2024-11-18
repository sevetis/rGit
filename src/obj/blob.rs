
use crate::obj::*;
pub struct Blob {
    content: Vec<u8>,
}

impl Blob {
    /// Create a new blob object
    pub fn new(raw_data: Vec<u8>) -> Result<Box<dyn Obj>> {
        if let Some(idx) = raw_data
            .iter()
            .position(|&x| x == 0)
        {
            return Ok(Box::new(Self {
                content: raw_data[idx + 1..].to_vec(),
            }));
        }
        Err(anyhow::anyhow!("invalid blob object"))
    }
}

impl Obj for Blob {
    fn obj_type(&self) -> Type {
        Type::Blob
    }

    fn size(&self) -> usize {
        self.content.len()
    }

    fn content(&self) -> &Vec<u8> {
        &self.content
    }

    fn to_string(&self) -> Result<String> {
        Ok(String::from_utf8_lossy(self.content()).into_owned())
    }

    fn parent(&self) -> Option<String> {
        None
    }
}