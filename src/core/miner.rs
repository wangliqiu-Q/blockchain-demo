use crate::core::block::Block;
use crate::core::blockchain::BlockChain;
use crate::core::pow::ProofOfWork;
use crate::core::transaction::Transaction;

pub struct Miner {
    address: [u8; 32],
}

impl Miner {
    pub fn new(address: [u8; 32]) -> Miner {
        Miner { address }
    }

    fn produce_block(
        vec_tx: Vec<Transaction>,
        pre_hash: [u8; 32],
        bits: u32,
        height: u64,
    ) -> Block {
        let mut block = Block::new(vec_tx, pre_hash, bits, height);
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }

    pub fn mine(
        &self,
        transactions: &mut Vec<Transaction>,
        pre_hash: [u8; 32],
        bits: u32,
        height: u64,
    ) -> Block {
        let mut vec_tx: Vec<Transaction> = Vec::new();
        let tx = Transaction::new([0; 32], self.address, 0, 0, 0, "coinbase".to_string());
        vec_tx.push(tx);
        vec_tx.append(transactions);

        // really, should check the bits need modify
        let bits = bits;

        Miner::produce_block(vec_tx, pre_hash, bits, height)
    }
}

pub struct Host {
    blockchain: BlockChain,
    miner: Miner,
}

const MINER_ADDRESS: [u8; 32] = [8; 32];

impl Host {
    pub fn new() -> Host {
        Host {
            blockchain: BlockChain::new_blockchain(),
            miner: Miner::new(MINER_ADDRESS),
        }
    }

    pub fn mining(&mut self, txs: &mut Vec<Transaction>) {
        let b = self.miner.mine(
            txs,
            self.blockchain.curr_hash,
            self.blockchain.curr_bits,
            self.blockchain.curr_height + 1,
        );

        self.blockchain.input_block(b).unwrap();
    }

    pub fn print(&self) {
        self.blockchain.print();
    }
}
