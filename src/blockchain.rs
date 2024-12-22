use parity_scale_codec::{Decode, Encode};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Encode, Decode)]
pub struct Block {
    block_number: u64,
    transactions: Vec<Transaction>,
}

#[derive(Encode, Decode, Debug, Default, Clone)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u128,
    nonce: u64,
    memo: String,
}

pub struct TransactionPool {
    pub pool: HashMap<String, Transaction>,
}

impl TransactionPool {
    pub fn init() -> Self {
        TransactionPool {
            pool: HashMap::new(),
        }
    }
    pub fn insert(&mut self, transaction: &Transaction) {
        let key = format!("{}-{}", transaction.sender, transaction.nonce);
        self.pool.insert(key, transaction.clone());
    }

    pub fn len(&self) -> usize {
        self.pool.len()
    }

    pub fn clean_for_block(&mut self) -> Vec<Transaction> {
        let result = self.pool.values().map(|x| x.clone()).collect::<Vec<_>>();
        self.pool.clear();
        result
    }
}

impl Transaction {
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.encode().as_slice());
        hasher.finalize().as_slice().to_vec()
    }

    pub fn from_map(map: &HashMap<String, String>) -> anyhow::Result<Transaction> {
        let sender = map
            .get("sender")
            .ok_or(anyhow::anyhow!("no sender"))?
            .to_owned();
        let recipient = map
            .get("recipient")
            .ok_or(anyhow::anyhow!("no recipient"))?
            .to_owned();
        let amount = map
            .get("amount")
            .ok_or(anyhow::anyhow!("no amount"))?
            .parse::<u128>()?;
        let nonce = map
            .get("nonce")
            .ok_or(anyhow::anyhow!("no nonce"))?
            .parse::<u64>()?;
        let memo = map
            .get("memo")
            .ok_or(anyhow::anyhow!("no memo"))?
            .to_owned();

        Ok(Transaction {
            sender,
            recipient,
            amount,
            nonce,
            memo,
        })
    }
}
