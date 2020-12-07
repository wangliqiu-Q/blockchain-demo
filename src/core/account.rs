use crate::core::transaction::Transaction;
use crate::utils::coder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub nonce: u64,
    /// 剩余金额
    pub balance: u64,
    pub address: [u8; 32],
    pub hash: [u8; 32],
    /// 私钥
    private: [u8; 32],
}

impl Account {
    pub fn new(address: [u8; 32], private: [u8; 32]) -> Account {
        let mut account = Account {
            nonce: 0,
            balance: 0,
            address,
            // set_hash
            hash: [0; 32],
            private,
        };
        account.set_hash();

        account
    }

    fn set_hash(&mut self) {
        let account_data = coder::serialize(&self);
        let mut hash = coder::get_hash(&account_data);

        self.hash = hash;
    }

    pub fn send_to(&mut self, to: [u8; 32], amount: u64, fee: u64) -> Result<Transaction, String> {
        if amount + fee > self.balance {
            return Err("amount + fee > balance".to_string());
        }

        self.balance -= amount;
        self.balance -= fee;
        self.nonce += 1;
        self.set_hash();

        let tx = Transaction::new(
            self.address,
            to,
            amount,
            fee,
            self.nonce,
            "sign".to_string(),
        );

        Ok(tx)
    }
}
