use std::io;

pub trait LogMgr {
    type IterResult;
    fn append(&mut self, log_rec: &[u8]) -> io::Result<usize>;
    fn flush(&mut self, lsn: usize) -> io::Result<()>;
    fn iter(&self) -> Self::IterResult;
}
