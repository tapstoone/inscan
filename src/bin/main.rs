
use inscan::cli;
use inscan::scan;

use {
    std::process::exit,
    clap::{Parser, Subcommand},
    bitcoincore_rpc::{Auth, Client},
    bitcoin::{Txid, hashes::Hash, Transaction}
};

fn create_connection(host: &String, user: String, pass: String)-> Result<Client, bitcoincore_rpc::Error>{
    let rpc = match Client::new(host, Auth::UserPass(user,pass)) {
        Ok(rpc) => rpc,
        Err(err) => return Err(err),
    };
    Ok(rpc)
}

fn main() {
    let cli = cli::Cli::parse();
    
    let protocol = cli.protocol;
    
    if (cli.out_file.is_some() && cli.out_db.is_some()) || (cli.out_file.is_none() && cli.out_db.is_none()){
        eprintln!("ERROR: out_file and out_db can only choose one. can't be both extis or both none!");
        exit(1);
    };

    // connection to rpc server
    let rpc = create_connection(&cli.rpc_host, cli.rpc_user, cli.rpc_pass).unwrap();

    // matches just as you would the top level cmd
    match &cli.command {
        Some(cli::Commands::Decode { block, txid }) => {

            if (block.is_some() && txid.is_some()) || (block.is_none() && txid.is_none()){
                panic!("height and txid can only choose one. can't be both extis or both none");
            }
            
            if block.is_some() && cli.out_file.is_some(){
                println!("Extract {protocol:?} from blocks {block:?} and save to local file ...");
                scan::run_blocks(&rpc, &block.as_ref().unwrap(), &protocol, &cli.out_file.as_ref().unwrap());
            }
            if block.is_some() && cli.out_db.is_some(){
                println!("Extract {protocol:?} from blocks {block:?} and save to database ...");
                scan::run_blocks(&rpc, &block.as_ref().unwrap(), &protocol, &cli.out_db.as_ref().unwrap());
            }

            if txid.is_some() && cli.out_file.is_some(){
                println!("Extract {protocol:?} from txs {txid:?} and save to local file ...");
                scan::run_txs(&rpc, &txid.as_ref().unwrap(), &protocol, &cli.out_file.as_ref().unwrap());
            }
            if txid.is_some() && cli.out_db.is_some(){
                println!("Extract {protocol:?} from txs {txid:?} and save to database ...");
                scan::run_txs(&rpc, &txid.as_ref().unwrap(), &protocol, &cli.out_db.as_ref().unwrap());
                // TODO
            }
        }
        Some(cli::Commands::Index { start }) => {
            if cli.out_file.is_some(){
                println!("Start scaning {protocol:?} from block {start:?} to latest block and save to local file ...");
                scan::index_realtime(&rpc, *start, &protocol, &cli.out_file.as_ref().unwrap());
            }
            if cli.out_db.is_some(){
                println!("Start scaning {protocol:?} from block {start:?} to latest block and save to database ...");
                scan::index_realtime(&rpc, *start, &protocol, &cli.out_db.as_ref().unwrap());
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}