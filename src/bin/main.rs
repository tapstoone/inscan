
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
        Some(cli::Commands::Decode { height, txid, protocol,output }) => {
            if (height.is_some() && txid.is_some()) || (height.is_none() && txid.is_none()){
                panic!("height and txid can only choose one. can't be both extis or both none");
            }
            if height.is_some() {
                println!("Extract {protocol:?} from blocks {height:?} ...");
                scan::run_blocks(&rpc,&height.as_ref().unwrap(), &protocol, &output);
            }
            if txid.is_some() {
                println!("Extract {protocol:?} from txs {txid:?} ...");
                scan::run_txs(&rpc, &txid.as_ref().unwrap(), &protocol, &output);
            }

            // 
        }
        Some(cli::Commands::Index { start, protocol, connection }) => {

        }
        None => {}
    }

    // Continued program logic goes here...
}