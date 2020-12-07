use crate::core::bcdb::BlockChainDb;
use crate::core::block::Block;
use crate::core::pow::ProofOfWork;
use crate::core::transaction::Transaction;
use crate::utils::coder;
use crate::utils::key::MyKey;
use crate::utils::key::U256;
use leveldb::database::Database;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct BlockChain {
    /// 仅仅用来 print
    block_index: Mutex<HashMap<[u8; 32], Block>>,
    blocks_db: Box<Database<MyKey>>,
    /// genesis块 hash 程序里写死
    pub genesis_hash: [u8; 32],
    pub curr_hash: [u8; 32],
    pub curr_bits: u32,
    /// 链长度
    pub curr_height: u64,
}

// const DIFFICULTY_1_TARGET: u32 = 0x1d00ffff;
/// 为了 pow 快速计算，暂时用这个数值
const DIFFICULTY_1_TARGET: u32 = 0x2100FFFF;

impl BlockChain {
    fn write_block(db: &mut Database<MyKey>, b: &Block) {
        let k = MyKey {
            val: U256::from(b.hash),
        };
        let v = coder::serialize(&b);

        BlockChainDb::write_db(db, k, &v);
    }

    /// k -> tail, v -> b.hash
    /// write the end block hash to database
    fn write_tail(db: &mut Database<MyKey>, b: &Block) {
        let k = MyKey {
            val: U256::from("tail".as_bytes()),
        };
        let v = coder::serialize(&b.hash);
        BlockChainDb::write_db(db, k, &v);
    }

    pub fn input_block(&mut self, b: Block) -> Result<(), String> {
        Self::write_block(&mut self.blocks_db, &b);
        // write tail
        if b.header.height > self.curr_height {
            Self::write_tail(&mut self.blocks_db, &b);
            self.curr_hash = b.hash;
            self.curr_bits = b.header.bits;
            self.curr_height = b.header.height;

            // 再判断是否需要回朔
        }

        // TODO 无限添加 内存爆炸
        Self::update_map(&mut self.block_index, b.clone());

        Ok(())
    }

    fn get_genesis_block() -> Block {
        let tx = Transaction::new([0; 32], [0; 32], 0, 0, 0, "This is genesis".to_string());
        let mut b = Block::new(vec![tx], [0; 32], DIFFICULTY_1_TARGET, 0);
        let data = ProofOfWork::block_header_se(&mut b, 0);
        b.hash = coder::get_hash(&data);

        b
    }

    fn update_map(map: &mut Mutex<HashMap<[u8; 32], Block>>, block: Block) {
        let mut map = map.lock().unwrap();
        map.insert(block.hash, block);
    }

    pub fn new_blockchain() -> BlockChain {
        let mut db = BlockChainDb::new_db("blockchain_db");
        let genesis = Self::get_genesis_block();
        Self::write_block(&mut db, &genesis);
        Self::write_tail(&mut db, &genesis);

        let mut map = Mutex::new(HashMap::new());
        Self::update_map(&mut map, genesis.clone());
        let genesis_hash = genesis.hash;

        BlockChain {
            block_index: map,
            genesis_hash,
            curr_bits: DIFFICULTY_1_TARGET,
            blocks_db: Box::new(db),
            curr_hash: genesis_hash,
            curr_height: 0,
        }
    }

    pub fn print(&self) {
        let mut hash = self.curr_hash;
        let mut blocks: Vec<Block> = Vec::new();

        let map = self.block_index.lock().unwrap();
        loop {
            if let Some(b) = map.get(&hash) {
                hash = b.header.pre_hash;
                blocks.push(b.clone());
            } else {
                panic!("found block error");
            }

            if hash == self.genesis_hash {
                break;
            }
        }

        blocks.reverse();
        for b in blocks {
            println!("--------------------------------------------------------------------------");
            println!("{:?}\n", b);
        }
    }
}
