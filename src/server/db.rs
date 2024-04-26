use std::{sync::Arc, io};

use crate::file::{file_mgr::FileMgr, file_mgr_impl::FileMgrImpl};


pub struct SimpleDB<FM> {
    fm: Arc<FM>
}

impl <FM: FileMgr> SimpleDB<FM> {
    #[allow(dead_code)]
    pub fn file_mgr(&self) -> Arc<FM> {
        self.fm.clone()
    }
}

impl SimpleDB<FileMgrImpl> {
    pub fn with_params(dir_name: &str, block_size: usize, buff_size: usize) -> io::Result<Self> {
        let fm = Arc::new(FileMgrImpl::new(dir_name, block_size)?);
        Ok(Self {
            fm
        })
    }

}
