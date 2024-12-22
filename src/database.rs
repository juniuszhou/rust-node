use crate::blockchain::Transaction;
use anyhow;
use parity_scale_codec::{Decode, Encode};
use rocksdb::{Options, DB};

const CURRENT_BLOCK_HEIGHT_KEY: &[u8] = b"current block height";

pub struct Database {
    pub db: DB,
}

impl Database {
    pub fn open(db_path: &str) -> anyhow::Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true); // Create the database if it doesn't exist

        // Open the database
        let db = DB::open(&opts, db_path)?;
        Ok(Database { db })
    }

    pub fn write(&self, key: &[u8], value: &[u8]) -> anyhow::Result<()> {
        self.db.put(key, value)?;
        Ok(())
    }

    pub fn read(&self, key: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
        self.db
            .get(key)
            .map_err(|err| anyhow::anyhow!(format!("read db error as {}", err.into_string())))
    }

    pub fn write_transaction(&self, transaction: &Transaction) -> anyhow::Result<()> {
        self.write(
            transaction.hash().as_slice(),
            transaction.encode().as_slice(),
        )
    }

    pub fn get_block_number(&self) -> u64 {
        match self.read(CURRENT_BLOCK_HEIGHT_KEY).unwrap() {
            Some(value) => u64::decode(&mut value.as_slice()).unwrap_or(0),
            None => 0_u64,
        }
    }

    pub fn set_block_number(&self, block_number: u64) -> anyhow::Result<()> {
        self.write(CURRENT_BLOCK_HEIGHT_KEY, block_number.encode().as_slice())
    }
}

#[tokio::test]
async fn test_db_open() {
    #[derive(Encode, Decode)]
    struct Data {
        pub a: String,
        pub b: i64,
    }
    let database = Database::open("test").unwrap();
    let block_number = database.get_block_number();
    assert_eq!(block_number, 0);
    database.set_block_number(100).unwrap();
    assert_eq!(database.get_block_number(), 100);
    let key = b"key";
    let value = b"value";
    database.write(key, value).unwrap();
    database.read(key).unwrap();

    let data = Data {
        a: "a".to_string(),
        b: 10,
    };
    database.write(key, data.encode().as_slice()).unwrap();

    let decoded_data = database.read(key).unwrap().unwrap();
    let result: Data = Data::decode(&mut decoded_data.as_slice()).unwrap();
    assert_eq!(result.b, 10);
}
