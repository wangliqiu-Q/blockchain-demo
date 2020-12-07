///
/// https://en.bitcoin.it/wiki/Block_hashing_algorithm
/// 交易列表附加在区块头后面，其中的第一笔交易是coinbase交易，这是一笔为了让矿工获得奖励及手续费的特殊交易。
///
///
use crate::core::transaction::Transaction;
use crate::utils::coder;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    /// 区块在链中的长度位置
    pub height: u64,
    pub time: i64,
    /// transactions data merkle root hash
    pub tx_hash: [u8; 32],
    /// pre_header_hash
    pub pre_hash: [u8; 32],
    /// Current target in compact format
    pub bits: u32,
    /// increment after a hash is tried
    pub nonce: u32,
    /// after all transaction, the merkle hash of all account state
    pub state_root: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    /// header_hash
    pub hash: [u8; 32],
    /// transactions data
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Merkle Tree 算法
    /// https://en.wikipedia.org/wiki/Merkle_tree
    ///
    /// /note/attach/Merkle_Tree.svg
    /// - 平衡二叉树：将数据块计算的哈希值两两配对，如果是奇数个数，最后一个自己与自己配对。
    /// - 任何底层数据块的变化，最终都会传导到根哈希。
    /// 若n为数据块的个数，则空间存储复杂度：O(n)，计算复杂度：O(n)，检索那个数据块错误的复杂度：O(log2n)，
    /// 所以主要是用于区块链数据校验。
    ///
    /// 以太坊用的是 Merkle Patricia Tree  https://blog.csdn.net/tianlongtc/article/details/80418923
    ///   
    fn merkle_root(mut vec_hash: Vec<[u8; 32]>) -> [u8; 32] {
        let mut size = vec_hash.len();
        if size == 0 {
            return [0; 32];
        }

        let mut j = 0usize;
        while size > 1 {
            let mut i1 = 0usize;
            while i1 < size {
                let mut i2 = i1 + 1;
                // 如果是奇数个数，最后一个自己与自己配对。
                if i2 == size {
                    i2 = i1;
                }
                let merge = (vec_hash[i1 + j], vec_hash[i2 + j]);
                let se = coder::serialize(&merge);
                let mut hash = coder::get_hash(&se);
                // 为了之后 j += size;
                vec_hash.push(hash);
                i1 += 2;
            }

            j += size;
            size = (size + 1) / 2;
        }

        match vec_hash.pop() {
            Some(root_hash) => return root_hash,
            None => panic!("vec_hash is empty!"),
        }
    }

    pub fn new(
        vec_tx: Vec<Transaction>,
        pre_hash: [u8; 32],
        bits: u32,
        height: u64,
    ) -> Block {
        let mut vec_hash = vec_tx.iter().map(|tx| tx.hash).collect::<Vec<[u8; 32]>>();

        Block {
            header: BlockHeader {
                height,
                time: Utc::now().timestamp(),
                tx_hash: Self::merkle_root(vec_hash),
                pre_hash,
                bits,
                nonce: 0,
                state_root: [0; 32],
            },
            hash: [0; 32],
            transactions: vec_tx,
        }
    }
}
