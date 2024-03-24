
use std::str;

use bitcoin::blockdata::{
    opcodes,
    script::{
        self,
        Instruction::{self, Op, PushBytes}
    }
};
use bitcoin::Transaction;


pub(crate) const PROTOCOL_ID: [u8; 3] = *b"ord";
pub(crate) const PROTOCOL:&str = "brc-20";

pub struct Brc20{
    block_number: String,
    tx_hash: String,
    p: String, // Protocol: Helps other systems identify and process brc-20 events
    op: Operation, 
}

enum Operation{
    Brc20Deploy(Brc20Deploy),
    Brc20Mint(Brc20Mint),
    Brc20Transfer(Brc20Transfer),
}

struct Brc20Deploy {
    op: String, //Operation: Type of event (Deploy, Mint, Transfer)
    tick: String, //Ticker: 4 letter identifier of the brc-20
    max: String, //Max supply: set max supply of the brc-20
    lim: Option<String>, //Mint limit: If letting users mint to themsleves, limit per ordinal
    dec: Option<String> //Decimals: set decimal precision, default to 18
}

struct Brc20Mint {
    op: String, //Operation: Type of event (Deploy, Mint, Transfer)
    tick: String, //Ticker: 4 letter identifier of the brc-20
    amt: String, //Amount to mint: States the amount of the brc-20 to mint. Has to be less than "lim" above if stated
}

struct Brc20Transfer {
    op: String, //Operation: Type of event (Deploy, Mint, Transfer)
    tick: String, //Ticker: 4 letter identifier of the brc-20
    amt: String, //Amount to transfer: States the amount of the brc-20 to transfer.
}

impl Brc20 {
    pub fn from_transaction(transaction: &Transaction)->Option<Brc20>{
        
        let tapscript = transaction.input[0].witness.tapscript().unwrap();
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
        // println!("\nordinals inscription bytes: {:?}", inscription);
        println!("\nordinals inscription decoded: {:?}, {:?}, {:?}", 
            str::from_utf8(&inscription[0]).unwrap(),
            str::from_utf8(&inscription[2]).unwrap(),
            str::from_utf8(&inscription[4]).unwrap(),
        );
        Some(Brc20 { 
                block_number: "84234".to_string(), 
                tx_hash: "hash".to_string(), 
                p: "brc-20".to_string(), 
                op: Operation::Brc20Mint(Brc20Mint{
                    op: "mint".to_string(),
                    tick: "ORDI".to_string(),
                    amt: "1000".to_string(),
                }),
            })
    }
}

