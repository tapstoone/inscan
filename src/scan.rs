use {
    crate::{
        atomicals,
        ord::{self, brcX::brc20, Inscription, InscriptionId, ParsedEnvelope},
    },
    anyhow::{Error, Ok, Result},
    bitcoin::Txid,
    bitcoincore_rpc::{Client, RpcApi},
    serde::{Deserialize, Serialize},
    serde_json::{self, Value},
    std::{
        str::{self, FromStr},
        string,
    },
    thiserror,
    ciborium,
};

// type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Serialize, Eq, PartialEq, Deserialize, Debug)]
pub struct CompactOutput {
    pub inscriptions: Vec<CompactInscription>,
}

#[derive(Serialize, Eq, PartialEq, Deserialize, Debug)]
pub struct RawOutput {
    pub inscriptions: Vec<ParsedEnvelope>,
}

#[derive(Serialize, Eq, PartialEq, Deserialize, Debug)]
pub struct CompactInscription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub duplicate_field: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub incomplete_field: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metaprotocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<InscriptionId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pointer: Option<u64>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub unrecognized_even_field: bool,
}

impl TryFrom<Inscription> for CompactInscription {
    type Error = Error;

    fn try_from(inscription: Inscription) -> Result<Self> {
        Ok(Self {
            content_encoding: inscription
                .content_encoding()
                .map(|header_value| header_value.to_str().map(str::to_string))
                .transpose()?,
            content_type: inscription.content_type().map(str::to_string),
            metaprotocol: inscription.metaprotocol().map(str::to_string),
            parent: inscription.parent(),
            pointer: inscription.pointer(),
            body: inscription.body.map(hex::encode),
            duplicate_field: inscription.duplicate_field,
            incomplete_field: inscription.incomplete_field,
            metadata: inscription.metadata.map(hex::encode),
            unrecognized_even_field: inscription.unrecognized_even_field,
        })
    }
}

fn parse_json(json_str: &str) -> Option<Value> {
    match serde_json::from_str(json_str) {
        std::result::Result::Ok(value) => Some(value),
        Err(_) => None,
    }
}

#[derive(Debug, thiserror::Error)]
enum BRC20Error{
    #[error("ContentTypeNull")]
    ContentTypeNull,
    #[error("ContentTypeNotValid")]
    ContentTypeNotValid,
    #[error("ContentBodyNull")]
    ContentBodyNull,
    #[error("ContentBodyNotJson")]
    ContentBodyNotJson,
}

// 
fn decode_brc20(inscription: Inscription) ->Result<String> {
    let content_type = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    if content_type != "text/plain"
        && content_type != "text/plain;charset=utf-8"
        && content_type != "text/plain;charset=UTF-8"
        && content_type != "application/json"
        && !content_type.starts_with("text/plain;")
    {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }

    let content_body = std::str::from_utf8(inscription.body().ok_or_else(|| BRC20Error::ContentBodyNull)?)?;
    let value = parse_json(content_body).ok_or_else(|| BRC20Error::ContentBodyNotJson)?;
    // TODO: check if this is brc20
    // Check if the key exists and if it is equal to "brc-20"
    let protocol = value.get("p").ok_or_else(|| BRC20Error::ContentBodyNotJson)?;
    if protocol == "brc-20" {
        let brc20_event = serde_json::to_string(&value).map_err(|err| Error::from(err))?;
        return Ok(brc20_event);
    } else {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

fn decode_brc100(inscription: Inscription) ->Result<String> {
    let content_type = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    if content_type != "text/plain"
        && content_type != "text/plain;charset=utf-8"
        && content_type != "text/plain;charset=UTF-8"
        && content_type != "application/json"
        && !content_type.starts_with("text/plain;")
    {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }

    let content_body = std::str::from_utf8(inscription.body().ok_or_else(|| BRC20Error::ContentBodyNull)?)?;
    let value = parse_json(content_body).ok_or_else(|| BRC20Error::ContentBodyNotJson)?;
    // TODO: check if this is brc20
    // Check if the key exists and if it is equal to "brc-20"
    let protocol = value.get("p").ok_or_else(|| BRC20Error::ContentBodyNotJson)?;
    if protocol == "BRC-100" || protocol == "BRC-101" || protocol == "BRC-102" {
        let brc20_event = serde_json::to_string(&value).map_err(|err| Error::from(err))?;
        return Ok(brc20_event);
    } else {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

fn decode_sns(inscription: Inscription) ->Result<String> {
    let content_type = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    if content_type != "text/plain"
        && content_type != "text/plain;charset=utf-8"
        && content_type != "text/plain;charset=UTF-8"
        && content_type != "application/json"
        && !content_type.starts_with("text/plain;")
    {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }

    let content_body = std::str::from_utf8(inscription.body().ok_or_else(|| BRC20Error::ContentBodyNull)?)?;
    let value = parse_json(content_body).ok_or_else(|| BRC20Error::ContentBodyNotJson)?;
    // TODO: check if this is brc20
    // Check if the key exists and if it is equal to "brc-20"
    let protocol = value.get("p").ok_or_else(|| BRC20Error::ContentBodyNotJson)?;
    if protocol == "sns" {
        let brc20_event = serde_json::to_string(&value).map_err(|err| Error::from(err))?;
        return Ok(brc20_event);
    } else {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

// https://github.com/BennyTheDev/tap-protocol-specs
fn decode_tap(inscription: Inscription) ->Result<String> {
    let content_type = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    if content_type != "text/plain"
        && content_type != "text/plain;charset=utf-8"
        && content_type != "text/plain;charset=UTF-8"
        && content_type != "application/json"
        && !content_type.starts_with("text/plain;")
    {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }

    let content_body = std::str::from_utf8(inscription.body().ok_or_else(|| BRC20Error::ContentBodyNull)?)?;
    let value = parse_json(content_body).ok_or_else(|| BRC20Error::ContentBodyNotJson)?;
    // TODO: check if this is brc20
    // Check if the key exists and if it is equal to "brc-20"
    let protocol = value.get("p").ok_or_else(|| BRC20Error::ContentBodyNotJson)?;
    if protocol == "tap" {
        let brc20_event = serde_json::to_string(&value).map_err(|err| Error::from(err))?;
        return Ok(brc20_event);
    } else {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

//TODO nft transfer is bind with ordinals number, not the nft self?
fn decode_bitmap(inscription: Inscription) ->Result<String> {
    let content_type = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    if content_type != "text/plain"
        && content_type != "text/plain;charset=utf-8"
        && content_type != "text/plain;charset=UTF-8"
        && content_type != "application/json"
        && !content_type.starts_with("text/plain;")
    {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }

    let content_body = std::str::from_utf8(inscription.body().ok_or_else(|| BRC20Error::ContentBodyNull)?)?;
    if content_body.ends_with(".bitmap") {
        return Ok(content_body.to_string());
    } else {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

fn decode_arc20(inscription: Inscription)->Result<ciborium::Value>{
    let op = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    if op != "dmt" //TODO: add other op
        && op != "dft"
        && op != "y"
    {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
    let body = inscription.body().ok_or_else(|| BRC20Error::ContentBodyNull)?;
    let body_string = body.to_vec();
    // println!("{:?}", body_string);
    let payload_value: ciborium::Value = ciborium::de::from_reader(&body_string[..])?; //TODO: get the diagnostic notation result
    // println!("\n>>> atomicals inscription decoded: {:?}", payload_value);
    Ok(payload_value)
    
}



/// should return Option instead Result -> Match or None
fn brc20_check(inscription: Inscription) {
    let content_type = inscription.content_type();
    match content_type {
        Some(content_type) => {
            // println!("content_type: {content_type:?}");
            if content_type != "text/plain"
                && content_type != "text/plain;charset=utf-8"
                && content_type != "text/plain;charset=UTF-8"
                && content_type != "application/json"
                && !content_type.starts_with("text/plain;")
            {
                // return Err("Unsupport content type");
                // return None;
                // println!("content_type: {content_type:?}");
            }
        }
        None => {
            println!("content_type: {content_type:?}");
            // return Some("aaa".to_string());
        }
    }

    let content_body = std::str::from_utf8(inscription.body().unwrap());
    match content_body {
        std::result::Result::Ok(c) => {
            let value = parse_json(c);
            match value {
                Some(v) => {
                    // TODO: check if this is brc20
                    // Check if the key exists and if it is equal to "brc-20"
                    if let Some(value) = v.get("p") {
                        if value == "brc-20" {
                            println!("{:?}", v);
                        } else {
                        }
                    } else {
                    }
                }
                None => {}
            }
        }
        Err(err) => println!("{:?}", err),
    }

    // let json_value: Value = serde_json::from_str(content_body)?;
    // if let Some(protocol) = json_value.get("p"){
    //     if protocol == "brc-20" {
    //         println!("Key 'key2' exists and its value is 'brc-20'");
    //     } else {
    //         println!("Key 'key2' exists but its value is not 'brc-20'");
    //     }
    // } else {
    //     println!("Key 'key2' does not exist");
    // };

    // let json_string = serde_json::to_string(&json_value)?;

    // Ok(json_string)
}



/// extract assets by protocol name from transaction id
pub fn decode_tx(rpc: &Client, txid: &Txid, protocol: &str) {
    let compact = true;
    let rawtx = rpc.get_raw_transaction(&txid, None).unwrap();
    
    match protocol.to_lowercase().as_str() {
        // ordinals
        "ord-nft" => {
            println!("ord-nft...");
            let ordinals = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            if compact {
                let result = Box::new(CompactOutput {
                    inscriptions: ordinals
                        .clone()
                        .into_iter()
                        .map(|inscription| inscription.payload.try_into())
                        .collect::<Result<Vec<CompactInscription>>>()
                        .unwrap(),
                });
                println!("\n{:?}: {:?}", txid, result);
            } else {
                let result = Box::new(RawOutput {
                    inscriptions: ordinals,
                });
            };
        }

        "bitmap" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_bitmap(inscription) {
                    std::result::Result::Ok(event) => println!("{:?}: {:?}", txid, event),
                    Err(err) =>{},
                } ;
            }
        }

        "brc20" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_brc20(inscription) {
                    std::result::Result::Ok(event) => println!("{:?}: {:?}", txid, event),
                    Err(err) =>{},
                } ;
            }
        }

        "brc100" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_brc100(inscription) {
                    std::result::Result::Ok(event) => println!("{:?}: {:?}", txid, event),
                    Err(err) =>{},
                } ;
            }
        }

        "brc420" => {
            println!("brc420...");
        }

        "sns" =>{
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_sns(inscription) {
                    std::result::Result::Ok(event) => println!("{:?}: {:?}", txid, event),
                    Err(err) =>{},
                } ;
            }
        }

        "tap" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_tap(inscription) {
                    std::result::Result::Ok(event) => println!("{:?}: {:?}", txid, event),
                    Err(err) =>{},
                };
            }
        }

        // ===Atomicals===
        "arc20" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"atom");
            // let raw_envelopes = ord::RawEnvelope::from_transaction(&rawtx);
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let brc20_event = match decode_arc20(inscription) {
                    std::result::Result::Ok(event) => println!("{:?}: {:?}", txid, event),
                    Err(err) =>{},
                } ;
            }
        }
        "arc721" => {
            println!("arc20...");
        }

        // ===runes===

        // ===stamps===
        _ => {
            // Default case if none of the above match
            println!("Unknown Protocol Name {:?}", protocol);
        }
    }
}



pub fn run_txs(rpc: &Client, txids: &String, protocols: &str) {
    let txs: Vec<&str> = txids.split(',').collect();
    // let protocols: Vec<&str> = protocols_str.split(',').collect();
    for item in txs {
        let id = Txid::from_str(item).unwrap();
        let rawtx = rpc.get_raw_transaction(&id, None).unwrap();

        let ordinals = ord::ParsedEnvelope::from_transaction(&rawtx, protocols.as_bytes());
        println!("\n{:?}: {:?}", item, ordinals);

        // for protocol in &protos{
        //     match protocol {
        //         &"arc20" => decode_trasaction_brc20(&rawtx),
        //         &"brc20" => println!("Found 'brc20'"),
        //         &"ordinals" => println!("Found 'ordinals'"),
        //         _ => println!("Found something else: {}", protocol),
        //     }
        // }

        // let tapscript = rawtx.input[0].witness.tapscript().unwrap();
        // println!("\ntapscript: {:?}", tapscript);
    }
}

pub fn run_blocks(rpc: &Client, block_number: u64, protocol: &str) {
    let block_hash = rpc.get_block_hash(block_number).unwrap();

    // get the txs
    let block_data = rpc.get_block(&block_hash).unwrap();
    // println!("the block header of {} is: {:?}", block_number, block_data.header);
    // println!("the block txs of {} is: {:?}", block_number, block_data.txdata);

    for tx in &block_data.txdata {
        let txid = tx.txid();
        decode_tx(rpc, &txid, protocol)
    }
}
