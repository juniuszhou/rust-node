use std::str::FromStr;

use clap::Parser;
use litep2p::PeerId;
use multiaddr::Multiaddr;
use node::RustNode;
use parity_scale_codec::{Decode, Encode};

mod blockchain;
mod cli;
mod database;
mod network;
mod node;
mod rollup;
mod rpc;
use blockchain::*;
use cli::*;
use network::*;
use rollup::*;
use rpc::*;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    log::info!("start the rust node");

    let cli = Cli::parse();
    match cli.command {
        Commands::Start {
            peer_id,
            peer_listen_addr,
            json_server_url,
            db_path,
        } => {
            log::info!("start the rust node start");

            // channel between rpc server and main thread
            let (tx, mut rx): (
                Sender<blockchain::Transaction>,
                Receiver<blockchain::Transaction>,
            ) = mpsc::channel(32);

            let rpc_server = JsonRpcServer::new(tx.clone(), &json_server_url);
            rpc_server.start();
            log::info!("rpc server started");

            let (mut litep2p, mut protocol_hander) = make_litep2p();
            let my_peer_id = litep2p.local_peer_id().clone();
            let listen_address = litep2p.listen_addresses().next().unwrap().clone();

            log::info!(
                "litep2p network started, my peer {:?}, my listen address {:?}",
                &my_peer_id,
                &listen_address
            );

            // add known peer into
            if let (Some(peer_id), Some(peer_listen_addr)) = (&peer_id, &peer_listen_addr) {
                let peer = PeerId::from_str(&peer_id).unwrap();
                let listen_addr = Multiaddr::from_str(&peer_listen_addr)?;
                litep2p.add_known_address(peer, std::iter::once(listen_addr));
            }

            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        event = litep2p.next_event() => {
                            log::info!("litep2p received event {:?}", &event);
                        }
                    }
                }
            });

            // init the node
            let mut node = RustNode::init(&db_path);

            // init timer for roll up in interval
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

            loop {
                log::info!("Into main routine loop");

                tokio::select! {
                    _ = interval.tick() => {
                        log::info!("Timer triggerred rollup");
                            let (block_number, transactions) = node.rollup()?;
                            let client = RollupSubmitter::new("");
                            client.submit(block_number, transactions)?;
                    }

                    tx = rx.recv() => {
                        log::info!("Received transaction in main thread from rpc {:?}", tx);

                        if let Some(transaction) = tx {
                            let result = node.add_transaction(&transaction)?;

                            // transaction size triggerred rollup
                            if let Some(data) = result {
                                let client = RollupSubmitter::new("");
                                client.submit(data.0, data.1)?;
                            }

                            let message = transaction.encode();
                            // send transaction to known peer if configured
                            if let Some(peer) = &peer_id {
                                let peer = PeerId::from_str(&peer)?;
                                protocol_hander.cmd_tx.send(CustomProtocolCommand::SendMessage { peer: peer, message }).await?;
                            }
                        }
                    }
                    message = protocol_hander.event_rx.recv() => {
                        log::info!("message from network as {:?}", message);

                        if let Some(CustomProtocolEvent::MessageReceived { peer: _, message }) = message {
                            let transaction = Transaction::decode(&mut message.as_slice())?;
                            log::info!("transaction from network as {:?}", transaction);
                            let result = node.add_transaction(&transaction)?;

                            if let Some(data) = result {
                                let client = RollupSubmitter::new("");
                                client.submit(data.0, data.1)?;
                            }
                        }
                    }
                }
            }
        }
    }
}
