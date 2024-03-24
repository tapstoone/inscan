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

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,


    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    #[command(about = "extract data from blocks number")]
    Blocks {
        /// the block numbers to extract, single(824958) or range(820000:820010)
        #[arg(short='n', long)]
        heights: u64,

        /// the protocols to extract
        #[arg(short='p', long)]
        protocols: String,

        /// the path to save extract inscriptions data 
        #[arg(short='o', long)]
        output: Option<PathBuf>,
    },
    /// does testing things
    #[command(about = "extract data from transaction ids")]
    Txs {
        /// lists test values
        #[arg(short, long)]
        txids: String,

        /// the protocols to extract
        #[arg(short='p', long, )]
        protocols: String,

        /// the path to save extract inscriptions data 
        #[arg(short='o', long)]
        output: Option<PathBuf>,
    }
}
