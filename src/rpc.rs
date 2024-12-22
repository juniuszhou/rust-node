use jsonrpc_http_server::jsonrpc_core::{IoHandler, Params, Value};
use jsonrpc_http_server::ServerBuilder;
use std::collections::HashMap;

use crate::blockchain::Transaction;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

pub struct JsonRpcServer {
    tx_sender: Arc<Mutex<Sender<Transaction>>>,
    url: String,
}

/*
$ curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "say_hello", "id":123 }' 127.0.0.1:3030
*/

impl JsonRpcServer {
    pub fn new(tx_sender: Sender<Transaction>, url: &str) -> Self {
        JsonRpcServer {
            tx_sender: Arc::new(Mutex::new(tx_sender)),
            url: url.to_owned(),
        }
    }

    pub fn start(&self) {
        let mut io = IoHandler::default();

        io.add_method("transaction", {
            let tx_sender = Arc::clone(&self.tx_sender); // Clone the Arc to share ownership
            move |params: Params| {
                let tx_sender = Arc::clone(&tx_sender);
                async move {
                    let value: HashMap<String, String> = params.parse().unwrap();
                    let message = Transaction::from_map(&value).unwrap();
                    tx_sender.lock().await.send(message).await.unwrap();
                    Ok(Value::String("message".to_owned()))
                }
            }
        });

        let server = ServerBuilder::new(io)
            .threads(1)
            // .start_http(&"127.0.0.1:3030".parse().unwrap())
            .start_http(&self.url.parse().unwrap())
            .unwrap();

        tokio::spawn(async move { server.wait() });
    }
}
