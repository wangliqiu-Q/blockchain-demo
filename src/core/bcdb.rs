use crate::utils::key::MyKey;
use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use std::{env, fs};

pub struct BlockChainDb;

impl BlockChainDb {
    pub fn new_db(path: &str) -> Database<MyKey> {
        let mut dir = env::current_dir().unwrap();
        // 加 \path
        dir.push(path);
        println!("db location: {}", dir.display());

        let path_buf = dir.clone();
        // move
        fs::create_dir_all(dir).unwrap();

        let mut options = Options::new();
        options.create_if_missing = true;
        Database::open(path_buf.as_path(), options)
            .unwrap_or_else(|e| panic!("failed to open database: {:?}", e))
    }

    pub fn write_db(db: &mut Database<MyKey>, k: MyKey, v: &[u8]) {
        let opts = WriteOptions::new();
        db.put(opts, k, &v)
            .unwrap_or_else(|e| panic!("failed to write block to database: {:?}", e));
    }

    pub fn read_db(db: &mut Database<MyKey>, k: MyKey) -> Option<Vec<u8>> {
        let opts = ReadOptions::new();
        db.get(opts, k).unwrap_or_else(|e| {
            eprintln!("failed to read from database: {}", e);
            None
        })
    }
}
