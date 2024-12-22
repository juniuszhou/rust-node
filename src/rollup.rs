use crate::Transaction;

pub struct RollupSubmitter {
    pub client: String,
}

impl RollupSubmitter {
    pub fn new(url: &str) -> Self {
        RollupSubmitter {
            client: url.to_string(),
        }
    }

    // simulate the rollup
    pub fn submit(&self, block_number: u64, transactions: Vec<Transaction>) -> anyhow::Result<()> {
        log::info!(
            "Successfully submit rollup to scroll network including {} transactions in block {} via client {}",
            transactions.len(),
            block_number,
            &self.client
        );
        Ok(())
    }
}
