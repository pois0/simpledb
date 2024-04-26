use std::{sync::Arc, io};

use crate::file::{page::Page, block_id::BlockId, file_mgr::FileMgr};

use super::log_mgr::LogMgr;

pub struct LogMgrImpl<FM> {
    fm: Arc<FM>,
    log_file: String,
    log_page: Page,
    current_blk: BlockId,
    latest_lsn: usize,
    last_saved_lsn: usize
}

fn append_new_block<FM: FileMgr>(fm: Arc<FM>, log_file: &str, log_page: &mut Page) -> io::Result<BlockId> {
    let blk = fm.append(log_file)?;
    log_page.set_int(0, fm.block_size() as i32);
    fm.write(&blk, log_page)?;
    Ok(blk)
}

impl <FM: FileMgr> LogMgrImpl<FM> {
    pub fn new(fm: Arc<FM>, log_file: &str) -> io::Result<Self> {
        let mut log_page = Page::new(fm.block_size());
        let log_size = fm.length(log_file)?;
        let current_blk = if log_size == 0 {
            append_new_block(fm.clone(), log_file, &mut log_page)?
        } else {
            let current_blk = BlockId::new(log_file, log_size as u32 - 1);
            fm.read(&current_blk, &mut log_page)?;
            current_blk
        };

        Ok(Self {
            fm,
            log_file: log_file.to_string(),
            log_page,
            current_blk,
            latest_lsn: 0,
            last_saved_lsn: 0
        })
    }

    fn flush_inner(&mut self) -> io::Result<()> {
        self.fm.write(&self.current_blk, &mut self.log_page)?;
        self.last_saved_lsn = self.latest_lsn;
        Ok(())
    }

    fn append_new_block(&mut self) -> io::Result<BlockId> {
        append_new_block(self.fm.clone(), &self.log_file, &mut self.log_page)
    }
}

impl <FM: FileMgr> LogMgr for LogMgrImpl<FM> {
    type IterResult = io::Result<LogMgrImplIter<FM>>;

    fn append(&mut self, log_rec: &[u8]) -> io::Result<usize> {
        let mut boundary = self.log_page.get_int(0) as usize;
        let bytes_needed = log_rec.len() + 4;
        if boundary - bytes_needed < 4 {
            self.flush_inner()?;
            self.current_blk = self.append_new_block()?;
            boundary = self.log_page.get_int(0) as usize;
        }
        let recpos = boundary - bytes_needed;
        self.log_page.set_bytes(recpos, log_rec);
        self.log_page.set_int(0, recpos as i32);
        self.latest_lsn += 1;

        Ok(self.latest_lsn)
    }

    fn flush(&mut self, lsn: usize) -> io::Result<()> {
        if lsn >= self.latest_lsn {
            self.flush_inner()?;
        }

        Ok(())
    }

    fn iter(&self) -> Self::IterResult {
        LogMgrImplIter::new(self.fm.clone(), self.current_blk.clone())
    }
}

pub struct LogMgrImplIter<FM> {
    fm: Arc<FM>,
    blk: BlockId,
    page: Page,
    current_pos: usize,
    boundary: usize
}

impl <FM: FileMgr> LogMgrImplIter<FM> {
    pub fn new(fm: Arc<FM>, blk: BlockId) -> io::Result<Self> {
        let page = Page::new(fm.block_size());
        let mut res = Self {
            fm,
            blk,
            page,
            current_pos: 0,
            boundary: 0,
        };
        res.move_to_block()?;

        Ok(res)
    }

    pub fn move_to_block(&mut self) -> io::Result<()> {
        self.fm.read(&self.blk, &mut self.page)?;
        self.boundary = self.page.get_int(0) as usize;
        self.current_pos = self.boundary;

        Ok(())
    }
}

impl <FM: FileMgr> Iterator for LogMgrImplIter<FM> {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos == self.fm.block_size() {
            self.blk = BlockId::new(self.blk.file_name(), self.blk.number() - 1);
            if let Err(e) = self.move_to_block() {
                return Some(Err(e));
            }
        }

        let rec = self.page.get_bytes(self.current_pos).to_vec();
        self.current_pos+= 4 + rec.len();
        Some(Ok(rec))
    }
}
