
use crate::obj::*;

pub struct Tree {
    content: Vec<u8>,
}

impl Tree {
    /// Create a new tree object
    pub fn new(raw_data: Vec<u8>) -> Result<Box<dyn Obj>> {
        if let Some(idx) = raw_data
            .iter()
            .position(|&x| x == 0)
        {
            return Ok(Box::new(Self {
                content: raw_data[idx + 1..].to_vec(),
            }));
        }
        Err(anyhow::anyhow!("invalid tree object"))
    }
}


impl Obj for Tree {
    fn obj_type(&self) -> Type {
        Type::Tree
    }

    fn size(&self) -> usize {
        self.content.len()
    }

    fn content(&self) -> &Vec<u8> {
        &self.content
    }

    fn to_string(&self) -> Result<String> {
        let bin_sha_len = 20;
        let cut = |data: &Vec<u8>, st: &mut usize, del: u8| -> Result<String> {
            let ed = data[(*st)..].iter()
                .position(|&x| x == del)
                .unwrap();
            let ret = String::from_utf8(data[*st..*st + ed].to_vec())?;
            *st += ed + 1;
            Ok(ret)
        };

        let mut idx = 0;
        let mut ret = String::new();
        while idx < self.content.len() {
            let mode = cut(&self.content, &mut idx, b' ')?;
            let name = cut(&self.content, &mut idx, 0)?;
            let hex_ = hex::encode(&self.content[idx..idx + bin_sha_len]);
            let obj_type = match mode {
                _ if mode.starts_with("40") => "tree",
                _ if mode.starts_with("10") => "blob",
                _ if mode.starts_with("12") => "blob",
                _ if mode.starts_with("16") => "commit",
                _ => return Err(anyhow::anyhow!("unknown tree leaf mode"))
            };
            
            idx += bin_sha_len;
            ret = format!(
                "{}{:0>6} {} {}\t{}\n",
                ret, mode, obj_type, hex_, name
            );
        }

        ret.pop();
        Ok(ret)
    }

    fn parent(&self) -> Result<Option<String>> {
        Err(anyhow::anyhow!("no parent for tree object"))
    }
}