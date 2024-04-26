use std::{path::Path, fs::{create_dir_all, read_dir, remove_file, OpenOptions, File}, io::{self, Seek, SeekFrom, Read, Write}};

use super::{block_id::BlockId, page::Page, file_mgr::FileMgr};

pub struct FileMgrImpl {
    db_directory: String,
    block_size: usize,
    is_new: bool,
}

impl FileMgrImpl {
    pub fn new(db_directory: &str, block_size: usize) -> io::Result<Self> {
        let is_new = !Path::new(db_directory).exists();
        if is_new {
            create_dir_all(db_directory)?;
        }
        for entry in read_dir(db_directory)? {
            let path = entry?.path();
            if path.starts_with("temp") {
                remove_file(path)?;
            }
        }

        Ok(Self {
            db_directory: db_directory.to_string(),
            block_size,
            is_new
        })
    }

    pub const fn is_new(&self) -> bool {
        self.is_new
    }

    fn get_file(&self, filename: &str) -> io::Result<File> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(Path::new(&self.db_directory).join(filename))
    }

    fn seek_file(&self, f: &mut File, blk: &BlockId) -> io::Result<u64> {
        let pos = blk.number() as u64 * self.block_size as u64;
        f.seek(SeekFrom::Start(pos))?;
        Ok(pos)
    }
}

impl FileMgr for FileMgrImpl {
    fn read(&self, blk: &BlockId, p: &mut Page) -> io::Result<()> {
        let mut f = self.get_file(blk.file_name())?;
        let pos = self.seek_file(&mut f, blk)?;
        if f.metadata()?.len() >= pos + p.contents().len() as u64 {
            f.read_exact(p.contents_mut())?;
        }
        // TODO Add else clause to imitate Java implementation?
        // else {
        //     f.read_to_end(p.contents())?;
        // } 

        Ok(())
    }

    fn write(&self, blk: &BlockId, p: &mut Page) -> io::Result<()> {
        let mut f = self.get_file(blk.file_name())?;
        self.seek_file(&mut f, blk)?;
        f.write_all(p.contents())?;

        Ok(())
    }

    fn append(&self, filename: &str) -> io::Result<BlockId> {
        let new_blk_num = self.length(filename)?;
        let blk = BlockId::new(filename, new_blk_num as i32);
        let b = vec![0u8; self.block_size];

        let mut f = self.get_file(filename)?;
        self.seek_file(&mut f, &blk)?;
        f.write_all(&b)?;

        Ok(blk)
    }

    fn length(&self, filename: &str) -> io::Result<usize> {
        let metadata = self.get_file(filename)?.metadata()?;
        Ok(metadata.len() as usize / self.block_size)
    }

    fn block_size(&self) -> usize {
        self.block_size
    }
}
