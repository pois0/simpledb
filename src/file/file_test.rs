#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{server::db::SimpleDB, file::{block_id::BlockId, page::Page, file_mgr::FileMgr}};

    const DATABASE_NAME: &str = "filetest";
    struct Context {}

    impl Drop for Context {
        fn drop(&mut self) {
            fs::remove_dir_all(DATABASE_NAME).unwrap();
        }
    }

    #[test]
    fn file_test() {
        let db = SimpleDB::with_params(DATABASE_NAME, 400, 8).unwrap();
        let fm = db.file_mgr();

        let blk = BlockId::new("testfile", 2);
        let mut p1 = Page::new(fm.block_size());
        let pos1 = 88;
        let pos1_str = "abcdefghijklm";
        p1.set_string(pos1, pos1_str);
        let size = Page::max_length(pos1_str.len());
        let pos2 = pos1 + size;
        let pos2_int = 345;
        p1.set_int(pos2, pos2_int);
        fm.write(&blk, &mut p1).unwrap();
    }
}
