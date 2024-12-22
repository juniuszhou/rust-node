# document for simple rust node

**"Rust-based Simple Rollup Node Prototype Implementation"**

In this task, you will implement a prototype node in Rust that simulates the "rollup" mechanism used by Ethereum-based L2 scaling solutions. The node will accumulate transactions in a local database and, under certain conditions, generate "rollup blocks" that bundle these transactions. Once a rollup block is generated, the node will post it to a designated Rollup Contract on the Scroll network. Additionally, you should implement a simple P2P network mechanism allowing multiple nodes to synchronize and share transactions. While scalability considerations are not strictly required, providing them can lead to a higher evaluation.

Please build a minimal system that meets the following basic requirements.

**Mandatory Requirements**:

1. **Node Features**
   - **Transaction Reception**:
     The node should provide an API or CLI command to accept incoming transactions (with simple fields such as sender, recipient, amount, nonce, etc.).
   - **Transaction Accumulation**:
     The node stores received transactions in a local database.
   - **Rollup Block Generation**:
     After a certain number of transactions (e.g., 10) or a certain time interval (e.g., 30 seconds), the node should process any pending transactions in bulk and generate a "rollup block" (a batch containing these transactions).
   - **Posting Rollup Blocks to Scroll Network**:
     The generated rollup blocks should be posted to a specified Rollup Contract on the Scroll network. This process can be implemented using transaction submission APIs, smart contract calls, or HTTP requests to an appropriate endpoint.
2. **P2P Network Synchronization**
   - The system should assume multiple nodes in a network. Implement a simple P2P- mechanism allowing nodes to synchronize transactions.

- When a node is started, it can be configured with the address(es) and port(s) of other nodes to connect to as peers.
- When a new transaction is received by any node, that node should broadcast the transaction to all its peers so that the transaction can be shared across the network.
- The P2P protocol can be very simple and does not need to adhere to any specific standard.

3. **Local DB Management**
   - Transactions should be recorded in a local persistent storage (DB).
   - After restarting the node, it is desirable (though not strictly required) that the previously stored transactions can be reused from the DB.

**Deliverables**:

- Rust source code
- README (including startup instructions and examples)
- A simple design document (optional)

RUST_LOG=info cargo run

## design

Basically, the simple rust blockchain node includes p2p network, rpc server, blockchain state, transaction pool, database and rollup submitter and so on.

### rpc

It is based on the jsonrpc_http_server, start according to --json-server-url to accept the transaction from the user. The valid transaction will be forwarded via channel to main routine

### p2p network

It is based on litep2p lib, start a listen port and send transaction to each other in p2p network. To simplify, just add the communication path from the second node to first one.

### transaction pool

It stores all transactions from the rpc or p2p network in a hashmap. To avoid the conflict, using the sender and nonce as key.

### database

use the rocksdb as key-value storage. just store all transactions now. key is the hash of transaction.

### state

put the current block number in state, update after submit the transactions to scroll network

### rollup

Not found out the rpc details in scroll website, just simulate the submit process.

## manuall test

1. clone the source code and install rust environment

2. run cargo build --release

3. start the first node

   ./target/release/rust-node start --json-server-url "127.0.0.1:3030" --db-path "first"

   record the output of peer id and listen address

4. start the second node

   ./target/release/rust-node start --json-server-url "127.0.0.1:4040" --peer-id "12D3KooWLhLqy6JGZynFnv3eijLYqYo8gKNgaaXJgJtaqQfL6VJj" --peer-listen-addr "/ip4/172.18.0.1/tcp/41973/p2p/12D3KooWLhLqy6JGZynFnv3eijLYqYo8gKNgaaXJgJtaqQfL6VJj" --db-path "second"

   Note: replace the peer-id and peer-listen-addr from the output of first node

5. check the submit can be done via timer

6. send transaction to rust node via rpc

```bash
curl -X POST -H "Content-Type: application/json" -d '{
"jsonrpc": "2.0",
"method": "transaction",
"params": {
"sender": "sender",
"recipient": "recipient",
"amount": "1",
"nonce": "1",
"memo": "memo"
},
"id": 1
}' 127.0.0.1:4040
```

send more transactions with different sender or nonce.
check submitter can rollup multiple transactions to scroll network in log.
