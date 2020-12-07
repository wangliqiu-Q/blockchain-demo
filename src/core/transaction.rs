use crate::utils::coder;
use serde::{Deserialize, Serialize};

/// 交易记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub hash: [u8; 32],
    pub from: [u8; 32],
    pub to: [u8; 32],
    /// 交易金额量
    pub amount: u64,
    /// 佣金
    pub fee: u64,
    /// Account.nonce
    pub nonce: u64,
    pub sign: String,
}

impl Transaction {
    pub fn new(
        from: [u8; 32],
        to: [u8; 32],
        amount: u64,
        fee: u64,
        nonce: u64,
        sign: String,
    ) -> Self {
        let mut tx = Transaction {
            // set_hash
            hash: [0; 32],
            from,
            to,
            amount,
            fee,
            nonce,
            sign,
        };
        tx.set_hash();

        tx
    }

    pub fn set_hash(&mut self) {
        let tx = coder::serialize(&self);
        let mut hash = coder::get_hash(&tx);

        self.hash = hash;
    }

    /// coinbase
    pub fn is_coinbase(&self) -> bool {
        (self.from == [0; 32]) && (self.to != [0; 32])
    }
}
