use anyhow;

const ROLLUP_SIZE: usize = 2;

use crate::{
    blockchain::{Transaction, TransactionPool},
    database::Database,
};

pub struct State {
    current_block_number: u64,
    transaction_pool: TransactionPool,
}

pub struct RustNode {
    state: State,
    db: Database,
}

impl RustNode {
    pub fn init(db_path: &str) -> Self {
        let db = Database::open(db_path).unwrap();
        let current_block_number = db.get_block_number();
        let transaction_pool = TransactionPool::init();
        let state = State {
            current_block_number,
            transaction_pool,
        };

        RustNode { state, db }
    }

    pub fn rollup(&mut self) -> anyhow::Result<(u64, Vec<Transaction>)> {
        let transactions = self.state.transaction_pool.clean_for_block();
        self.state.current_block_number += 1;
        self.db.set_block_number(self.state.current_block_number)?;
        Ok((self.state.current_block_number - 1, transactions))
    }

    // add transaction and check if the size for rollup
    pub fn add_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> anyhow::Result<Option<(u64, Vec<Transaction>)>> {
        self.db.write_transaction(transaction)?;
        self.state.transaction_pool.insert(transaction);
        let len = self.state.transaction_pool.len();
        if len >= ROLLUP_SIZE {
            let result = self.rollup()?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}
