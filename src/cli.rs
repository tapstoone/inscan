use clap::{Parser, Subcommand};


/// Extract inscription events from bitcoin.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {

    /// bitcoin rpc server
    #[arg(short='s', long, default_value = "http://localhost:8332")]
    pub rpc_host: String,

    /// bitcoin rpc server user name
    #[arg(short='u', long, required=true)]
    pub rpc_user: String,

    /// bitcoin rpc server user password
    #[arg(short='w', long, required=true)]
    pub rpc_pass: String,

    /// the protocols to extract,
    #[arg(short='p', long, default_value="all")]
    pub protocol: String,

    /// save decoded event to local jsonl file
    #[arg(short='f', long)]
    pub out_file: Option<String>,

    /// save decoded event to postgres database
    #[arg(short='d', long)]
    pub out_db: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// blocks params
    #[command(about = "decode specific block height or transaction id")]
    Decode {
        /// the block height to extract, single(824958) or range(820000:820010)
        #[arg(short='H', long)]
        block: Option<String>,

        /// the tx id to extract, single(04ij...dhf92) or range(04ij...dhf92,jgi..8gjs)
        #[arg(short='T', long)]
        txid: Option<String>,
        
    },
    /// transactions params
    #[command(about = "scan all blocks to latest height in real time")]
    Index {
        /// lists test values
        #[arg(short='S', long)]
        start: String,

    }
}
