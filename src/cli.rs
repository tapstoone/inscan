use clap::{Parser, Subcommand};
use std::path::PathBuf;


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
    Blocks {
        /// the block numbers to extract, single(824958) or range(820000:820010)
        #[arg(short='n', long)]
        height: String,

        /// the protocols to extract
        #[arg(short='p', long)]
        protocol: String,

        /// the path to save extract inscriptions data 
        #[arg(short='o', long)]
        output: Option<PathBuf>,
    },
    /// transactions params
    #[command(about = "extract data from transaction ids")]
    Txs {
        /// lists test values
        #[arg(short, long)]
        txid: String,

        /// the protocols to extract
        #[arg(short='p', long, )]
        protocol: String,

        /// the path to save extract inscriptions data 
        #[arg(short='o', long)]
        output: Option<PathBuf>,
    }
}
