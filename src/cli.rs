use clap::{Parser, Subcommand};


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
    #[arg(short='p', long, required=true)]
    pub rpc_pass: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// blocks params
    #[command(about = "extract data from blocks numbers")]
    Decode {
        /// the block height to extract, single(824958) or range(820000:820010)
        #[arg(short='b', long)]
        height: Option<String>,

        /// the tx id to extract, single(04ij...dhf92) or range(04ij...dhf92,jgi..8gjs)
        #[arg(short='t', long)]
        txid: Option<String>,
        
        /// the protocols to extract
        #[arg(short='p', long)]
        protocol: String,

        /// the path to save jsonl data 
        #[arg(short='o', long)]
        output: String,

    },
    /// transactions params
    #[command(about = "index all the data and save to db")]
    Index {
        /// lists test values
        #[arg(short='s', long)]
        start: u64,

        /// the protocols to extract
        #[arg(short='p', long, )]
        protocol: String,

        /// the uri to connection postgres 
        #[arg(short='c', long)]
        connection: Option<String>,
    }
}
