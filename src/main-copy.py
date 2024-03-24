use std::hash::Hash;
use std::io::Error;
use std::str;
use std::str::FromStr;

use bitcoin::address::Payload;
use bitcoin::hex::DisplayHex;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoin::{self, psbt::raw, BlockHash, Txid};
use bitcoin::blockdata::{
    opcodes,
    script::{
        self,
        Instruction::{self, Op, PushBytes}
    }
};

use ciborium::de::from_reader;
use ciborium::value::Value;

fn create_connection(host: &String, user: String, pass: String)-> Result<Client, Error>{
    let rpc = Client::new(host, Auth::UserPass(user,pass)).unwrap();
    Ok(rpc)
}

fn decode_tx_atomicals(rpc: &Client, txid: &String){
    // let mut result = Vec::new();


    // let hashv = sha256d::Hash::from_slice(txid.as_bytes()).unwrap();
    // let hashv = "c631181e8f7740064ec5e832d773086369d30f5297713a0b098d6d95ffe0c78b".parse().unwrap();
    let aa = Txid::from_str(txid).unwrap();
    println!("txid: {:?}",aa.to_string());
    // let bitcoin_txid = bitcoin::Txid
    // let rawtx = rpc.get_raw_transaction(&aa, None).unwrap();
    // let rawtx = rpc.get_raw_transaction_hex(&aa, None).unwrap();
    let rawtx = rpc.get_raw_transaction(&aa, None).unwrap();
    let tapscript = rawtx.input[0].witness.tapscript().unwrap();
    println!("\ntapscript: {:?}", tapscript);
    let instructions = tapscript.instructions().peekable();

    let mut flag_op0 = false;
    let mut flag_opif = false;
    let mut flag_opendif = false;

    let mut inscription = Vec::new();

    for item in instructions{
        // println!("\ntapscript item: {:?}", item);
        let opitem: Instruction<'_> = item.unwrap();

        if opitem == Op(opcodes::all::OP_ENDIF) {
            flag_opendif = true;
            flag_op0 = false;
            flag_opif = false;
            // println!("OP_ENDIF");
        }

        if flag_op0 && flag_opif {
            // println!("ordinals start...");
            // get the protocol name
            let aa = opitem.push_bytes().unwrap().as_bytes().to_vec();
            inscription.push(aa);
        }

        // if opitem == Op(opcodes::OP_0) {
        if opitem == PushBytes((&[]).into()) { // opcodes::OP_0 is not work
            // println!("OP_FALSE");
            flag_op0 = true;
        } 
        else if  opitem == Op(opcodes::all::OP_IF) {
            flag_opif = true;
            // println!("OP_IF");
        } 

        // else if opitem {
        //     // let aa = opitem.push_bytes().unwrap().as_bytes().to_vec();
        //     // let decoded = str::from_utf8(&aa).unwrap();
        //     // println!("{:?}", decoded);
        // }
        // println!("{:?}", item.unwrap())
    }
    println!("\nordinals inscription bytes: {:?}", inscription);

    
    let payload = &inscription[2]; //payload use CBOR encoding
    let payload_value: Value = from_reader(&payload[..]).unwrap(); //TODO: get the diagnostic notation result
    println!("\nordinals inscription decoded: {:?}, {:?}, {:#?}", 
        str::from_utf8(&inscription[0]).unwrap(),
        str::from_utf8(&inscription[1]).unwrap(),
        payload_value
    );
    // Ok(instructions)
}

fn decode_tx_stamps(rpc: &Client, txid: &String){
    let aa = Txid::from_str(txid).unwrap();
    println!("txid: {:?}",aa.to_string());
    // let bitcoin_txid = bitcoin::Txid
    // let rawtx = rpc.get_raw_transaction(&aa, None).unwrap();
    // let rawtx = rpc.get_raw_transaction_hex(&aa, None).unwrap();
    let rawtx = rpc.get_raw_transaction(&aa, None).unwrap();
    let signing_key = rawtx.input[0].previous_output.txid.to_string();
    println!("{:#?}", signing_key);

    let mut result_encode = Vec::new();

    // encoded_script_1
    let encoded_script_1 = &rawtx.output[1].script_pubkey;
    let mut pushbytes_idx = 1;
    for item in encoded_script_1.instructions().peekable(){
        if pushbytes_idx > 2{
            break;
        }
        let opitem: Instruction<'_> = item.unwrap();
        // println!("{:?}", opitem);
        // let aa = opitem.push_bytes().unwrap();
        if let Some(pushed_bytes) = opitem.push_bytes() {
            let value = pushed_bytes.as_bytes().to_vec();
            println!("{:?}", &value[1..value.len()-1]);
            result_encode.extend_from_slice(&value[1..value.len()-1]);
            pushbytes_idx += 1;
        } else {
            // 处理 None 的情况，如果需要的话
        }
        
    }

    let encoded_script_2 = &rawtx.output[2].script_pubkey;
    let mut pushbytes_idx = 1;
    for item in encoded_script_2.instructions().peekable(){
        if pushbytes_idx > 2{
            break;
        }
        let opitem: Instruction<'_> = item.unwrap();
        // println!("{:?}", opitem);
        // let aa = opitem.push_bytes().unwrap();
        if let Some(pushed_bytes) = opitem.push_bytes() {
            let value = pushed_bytes.as_bytes().to_vec();
            println!("{:?}", &value[1..value.len()-1]);
            result_encode.extend_from_slice(&value[1..value.len()-1]);
            pushbytes_idx += 1;
        } else {
            // 处理 None 的情况，如果需要的话
        }
        
    }


    println!("{:?}", result_encode);
    println!("{:?}", result_encode.as_hex());
}

fn decode_tx_runes(rpc: &Client, txid: &String){
    
}

struct Ordinals{
    protocol: String,
    content_type: String,
    payload: String
}

fn decode_tx_ordinals(rpc: &Client, txid: &String){
    // let mut result = Vec::new();


    // let hashv = sha256d::Hash::from_slice(txid.as_bytes()).unwrap();
    // let hashv = "c631181e8f7740064ec5e832d773086369d30f5297713a0b098d6d95ffe0c78b".parse().unwrap();
    let aa = Txid::from_str(txid).unwrap();
    println!("{:?}",aa.to_string());
    // let bitcoin_txid = bitcoin::Txid
    // let rawtx = rpc.get_raw_transaction(&aa, None).unwrap();
    // let rawtx = rpc.get_raw_transaction_hex(&aa, None).unwrap();
    let rawtx = rpc.get_raw_transaction(&aa, None).unwrap();
    let tapscript = rawtx.input[0].witness.tapscript().unwrap();
    println!("\ntapscript: {:?}", tapscript);
    let instructions = tapscript.instructions().peekable();

    let mut flag_op0 = false;
    let mut flag_opif = false;
    let mut flag_opendif = false;

    let mut inscription = Vec::new();

    for item in instructions{
        // println!("\ntapscript item: {:?}", item);
        let opitem: Instruction<'_> = item.unwrap();

        if opitem == Op(opcodes::all::OP_ENDIF) {
            flag_opendif = true;
            flag_op0 = false;
            flag_opif = false;
            // println!("OP_ENDIF");
        }

        if flag_op0 && flag_opif {
            // println!("ordinals start...");
            // get the protocol name
            let aa = opitem.push_bytes().unwrap().as_bytes().to_vec();
            inscription.push(aa);
        }

        // if opitem == Op(opcodes::OP_0) {
        if opitem == PushBytes((&[]).into()) { // opcodes::OP_0 is not work
            // println!("OP_FALSE");
            flag_op0 = true;
        } 
        else if  opitem == Op(opcodes::all::OP_IF) {
            flag_opif = true;
            // println!("OP_IF");
        } 

        // else if opitem {
        //     // let aa = opitem.push_bytes().unwrap().as_bytes().to_vec();
        //     // let decoded = str::from_utf8(&aa).unwrap();
        //     // println!("{:?}", decoded);
        // }
        // println!("{:?}", item.unwrap())
    }
    println!("\nordinals inscription bytes: {:?}", inscription);
    println!("\nordinals inscription decoded: {:?}, {:?}, {:?}", 
        str::from_utf8(&inscription[0]).unwrap(),
        str::from_utf8(&inscription[2]).unwrap(),
        str::from_utf8(&inscription[4]).unwrap(),
    );
    // Ok(instructions)
}

fn decode_block(rpc:&Client, block_number: u64){
    // let hash = BlockHash::from_str(
    //     "0000000000002917ed80650c6174aac8dfc46f5fe36480aaef682ff6cd83c3ca",
    // );
    let block_hash = rpc.get_block_hash(block_number).unwrap();
    println!("the block hash of {} is: {}", block_number, block_hash);

    // get the txs
    let block_data = rpc.get_block(&block_hash).unwrap();
    println!("the block header of {} is: {:?}", block_number, block_data.header);
    // println!("the block txs of {} is: {:?}", block_number, block_data.txdata);
    
    for tx in &block_data.txdata{
        let txid = tx.txid();
        
        // println!("\nthe tx hash is {:?}", txid.to_string());
        if txid.to_string() == "c631181e8f7740064ec5e832d773086369d30f5297713a0b098d6d95ffe0c78b".to_string(){
            println!("\nthe tx hash is {:?}", tx.txid()); //this is a hex format, should remove the 0x from previous
            println!("\nthe tx input is {:?}", tx.input); 
            for intx in &tx.input{
                // let tapscript = intx.witness.tapscript();
                if let Some(tapscript) = intx.witness.tapscript(){
                    println!("\nthe first input of tx is: {:?}", tapscript.instructions());
                }
                // println!("\nthe first input of tx is: {:?}", tapscript);
                // if tapscript
            }
            println!("\nthe tx output is {:?}", tx.output);
            // let raw_tx = rpc.get_raw_transaction_info(&txid, None);
            // let raw_tx = rpc.get_raw_transaction_info(&txid, Some(&block_hash));
            // println!("\nthe tx output is {:?}", raw_tx);
        }
        // break;
    }


    // get the txs directly
    // let tx_id = "ccd0316ff0dec3469312d543866edac38e2accdc1fcbe23702b954dcf72f942e".to_string();
    // let tx = rpc.decode_raw_transaction(tx_id, None);
    // println!("\nthe tx output is {:?}", tx);
}


fn main() {
    let host  = "http://localhost:8332".to_string();
    let user = "devnet".to_string();
    let pass = "devnet".to_string();
    // let rpc = Client::new(&host, Auth::UserPass(user,pass)).unwrap();
    let rpc = create_connection(&host, user, pass).unwrap();

    // let block_number: u64 = 832222;
    // decode_block(&rpc, block_number);

    // ordinals
    // let txid_ordinals = String::from("ae8eb9d1664e25357f362f0f6030b95825cbeaafc8587b78cf3b1db4735b322d");
    // decode_tx_ordinals(&rpc, &txid_ordinals);

    // atomicals
    // let txid_atomicals = String::from("25c1cc22d744c9d4577d84d94018bc6c48490bccda5d279518b684b3257d85dd");
    // decode_tx_atomicals(&rpc, &txid_atomicals);


    // stamps 
    let txid_stamps = String::from("50aeb77245a9483a5b077e4e7506c331dc2f628c22046e7d2b4c6ad6c6236ae1");
    decode_tx_stamps(&rpc, &txid_stamps);

}