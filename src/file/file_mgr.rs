use std::io;

use super::{page::Page, block_id::BlockId};

pub trait FileMgr {
    fn read(&self, blk: &BlockId, p: &mut Page) -> io::Result<()>;
    fn write(&self, blk: &BlockId, p: &mut Page) -> io::Result<()>;
    fn append(&self, filename: &str) -> io::Result<BlockId>;
    fn length(&self, filename: &str) -> io::Result<usize>;
    fn block_size(&self) -> usize;
}
