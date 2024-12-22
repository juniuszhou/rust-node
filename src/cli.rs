use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the process
    Start {
        /// The name of the process to start
        #[arg(long)]
        peer_id: Option<String>,
        #[arg(long)]
        peer_listen_addr: Option<String>,
        #[arg(long)]
        json_server_url: String,
        #[arg(long)]
        db_path: String,
    },
}
