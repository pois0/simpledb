#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockId {
    file_name: String,
    blk_num: u32,
}

impl BlockId {
    pub fn new(file_name: &str, blk_num: u32) -> Self {
        BlockId {
            file_name: file_name.to_string(),
            blk_num
        }
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub const fn number(&self) -> u32 {
        self.blk_num
    }
}

