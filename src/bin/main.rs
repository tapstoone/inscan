
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

    // println!("{:?}", cli.rpc_host);

    // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {name}");
    // }

    // if let Some(config_path) = cli.config.as_deref() {
    //     println!("Value for config: {}", config_path.display());
    // }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    // connection to rpc server
    let rpc = create_connection(&cli.rpc_host, cli.rpc_user, cli.rpc_pass).unwrap();
    // println!("{:?}", rpc);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(cli::Commands::Blocks { heights, protocols, output }) => {
            println!("Extract {protocols:?} from blocks {heights:?} ...");

            scan::run_blocks(&rpc,heights.clone(), &protocols);
            // 
        }
        Some(cli::Commands::Txs { txids, protocols, output }) => {
            println!("Extract {protocols:?} from txs {txids:?} ...");
            let tx = Txid::from_str(txids).unwrap();
            scan::decode_tx(&rpc, &tx, &protocols);
            // let rawtx = rpc.get_raw_transaction(&id, None).unwrap();
            // scan::run_txs(&rpc, &txids, &protocols);
        }
        None => {}
    }

    // Continued program logic goes here...
}