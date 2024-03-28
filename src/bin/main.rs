
use inscan::cli;
use inscan::scan;

use {
    std::str::FromStr,
    clap::{Parser, Subcommand},
    bitcoincore_rpc::{Auth, Client},
    bitcoin::{Txid, hashes::Hash, Transaction}
};

fn create_connection(host: &String, user: String, pass: String)-> Result<Client, bitcoincore_rpc::Error>{
    let rpc = match Client::new(host, Auth::UserPass(user,pass)) {
        Ok(rpc) => rpc,
        Err(err) => return Err(err),
    };
    // &rpc.get_blockchain_info().unwrap();
    Ok(rpc)
}

fn main() {
    let cli = cli::Cli::parse();

    // connection to rpc server
    let rpc = create_connection(&cli.rpc_host, cli.rpc_user, cli.rpc_pass).unwrap();

    // matches just as you would the top level cmd
    match &cli.command {
        Some(cli::Commands::Blocks { height, protocol, output }) => {
            println!("Extract {protocol:?} from blocks {height:?} ...");
            scan::run_blocks(&rpc,height, &protocol);
            // 
        }
        Some(cli::Commands::Txs { txid, protocol, output }) => {
            println!("Extract {protocol:?} from txs {txid:?} ...");
            scan::run_txs(&rpc, &txid, &protocol);
        }
        None => {}
    }

    // Continued program logic goes here...
}