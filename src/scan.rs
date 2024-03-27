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
    base64
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
fn decode_ord_brc20(inscription: Inscription) ->Result<String> {
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

fn decode_ord_brc100(inscription: Inscription) ->Result<String> {
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

fn decode_ord_brc420(inscription: Inscription) ->Result<String> {
    let content_type = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    if content_type != "text/plain"
        && content_type != "text/plain;charset=utf-8"
        && content_type != "text/plain;charset=UTF-8"
        && content_type != "application/json"
        && content_type != "application/json"
        && content_type != "text/html;charset=utf-8"
        && !content_type.starts_with("text/plain;")
    {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }

    let content_body = std::str::from_utf8(inscription.body().ok_or_else(|| BRC20Error::ContentBodyNull)?)?;
    if content_body.starts_with("/content/"){
        return Ok(content_body.to_string());
    }
    let value = parse_json(content_body).ok_or_else(|| BRC20Error::ContentBodyNotJson)?;
    // TODO: check if this is brc20
    // Check if the key exists and if it is equal to "brc-20"
    let protocol = value.get("p").ok_or_else(|| BRC20Error::ContentBodyNotJson)?;
    if protocol == "brc-420" {
        let brc20_event = serde_json::to_string(&value).map_err(|err| Error::from(err))?;
        return Ok(brc20_event);
    } else {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

fn decode_ord_sns(inscription: Inscription) ->Result<String> {
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
fn decode_ord_tap(inscription: Inscription) ->Result<String> {
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
fn decode_ord_bitmap(inscription: Inscription) ->Result<String> {
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

fn decode_atom_arc20(inscription: Inscription)->Result<ciborium::Value>{
    let op = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    println!("operation: {:?}", op);
    if op != "dft" //TODO: add other op
        && op != "ft"
        && op != "dmt"
        && op != "y"
    {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
    let body = inscription.body().ok_or_else(|| BRC20Error::ContentBodyNull)?;
    let body_string = body.to_vec();
    // println!("{:?}", body_string);
    let payload_value: ciborium::Value = ciborium::de::from_reader(&body_string[..])?; //TODO: get the diagnostic notation result
    // let temp = payload_value.as_map().unwrap();
    // println!("\n>>> atomicals inscription decoded: {:?}", temp);
    Ok(payload_value)
    
}

fn cbor_into_string(cbor: ciborium::Value) -> Option<String> {
    match cbor {
        ciborium::Value::Text(string) => Some(string),
        _ => None,
    }
}

fn cbor_to_json(cbor: ciborium::Value) -> serde_json::Value {
    match cbor {
        ciborium::Value::Null => serde_json::Value::Null,
        ciborium::Value::Bool(boolean) => serde_json::Value::Bool(boolean),
        ciborium::Value::Text(string) => serde_json::Value::String(string),
        ciborium::Value::Integer(int) => serde_json::Value::Number({
            let int: i128 = int.into();
            if let std::result::Result::Ok(int) = u64::try_from(int) {
                serde_json::Number::from(int)
            } else if let std::result::Result::Ok(int) = i64::try_from(int) {
                serde_json::Number::from(int)
            } else {
                serde_json::Number::from_f64(int as f64).unwrap()
            }
        }),
        ciborium::Value::Float(float) => serde_json::Value::Number(serde_json::Number::from_f64(float).unwrap()),
        ciborium::Value::Array(vec) => serde_json::Value::Array(vec.into_iter().map(cbor_to_json).collect()),
        ciborium::Value::Map(map) => serde_json::Value::Object(map.into_iter().map(|(k, v)| (cbor_into_string(k).unwrap(), cbor_to_json(v))).collect()),
        ciborium::Value::Bytes(byte) => serde_json::Value::String(base64::encode(byte)),
        ciborium::Value::Tag(_, _) => unimplemented!(),
        _ => unimplemented!(),
    }
}

// TODO:  atom should be decoded in one place, and let the application decide which one to use
fn decode_atom_relam(inscription: Inscription)->Result<String>{
    let op = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    println!("operation: {:?}", op);
    if op != "nft" //TODO: add other op
    {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
    let body = inscription.body().ok_or_else(|| BRC20Error::ContentBodyNull)?;
    let body_string = body.to_vec();
    // println!("{:?}", body_string);
    let payload_value: ciborium::Value = ciborium::de::from_reader(&body_string[..])?; //TODO: get the diagnostic notation result
    // println!("\n>>> atomicals ciborium::Value: {:?}", payload_value);
    let jsons = cbor_to_json(payload_value);
    // check if request_realm and request_subrealm in keys
    let request_realm = &jsons["args"]["request_realm"];
    let request_subrealm = &jsons["args"]["request_subrealm"];
    if request_realm.is_null() && request_subrealm.is_null() {
        Err(BRC20Error::ContentTypeNotValid.into())
    }
    else{
        Ok(jsons.to_string())
    }
    
    // println!("\n>>> atomicals inscription decoded: {:?}", jsons.to_string());
    // let temp = payload_value.as_map().unwrap();
    // println!("\n>>> atomicals inscription decoded: {:?}", temp);
    
}


fn decode_atom_nft(inscription: Inscription)->Result<String>{
    let op = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    if op != "nft" //TODO: add other op
    {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
    let body = inscription.body().ok_or_else(|| BRC20Error::ContentBodyNull)?;
    let body_string = body.to_vec();
    // println!("{:?}", body_string);
    let payload_value: ciborium::Value = ciborium::de::from_reader(&body_string[..])?; //TODO: get the diagnostic notation result
    // println!("\n>>> atomicals ciborium::Value: {:?}", payload_value);
    let jsons = cbor_to_json(payload_value);
    // check if request_realm and request_subrealm in keys
    let request_realm = &jsons["args"]["request_realm"];
    let request_subrealm = &jsons["args"]["request_subrealm"];
    if !request_realm.is_null() || !request_subrealm.is_null() {
        Err(BRC20Error::ContentTypeNotValid.into())
    }
    else{
        Ok(jsons.to_string())
    }
    
}

fn decode_atom_others(inscription: Inscription)->Result<String>{
    let op = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    if op != "mod"
       && op != "evt"
       && op != "dat"
       && op != "sl"
       && op != "x"
    {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
    let body = inscription.body().ok_or_else(|| BRC20Error::ContentBodyNull)?;
    let body_string = body.to_vec();
    // println!("{:?}", body_string);
    let payload_value: ciborium::Value = ciborium::de::from_reader(&body_string[..])?; //TODO: get the diagnostic notation result
    // println!("\n>>> atomicals ciborium::Value: {:?}", payload_value);
    let jsons = cbor_to_json(payload_value);
    // check if request_realm and request_subrealm in keys
    let request_realm = &jsons["args"]["request_realm"];
    let request_subrealm = &jsons["args"]["request_subrealm"];
    if !request_realm.is_null() || !request_subrealm.is_null() {
        Err(BRC20Error::ContentTypeNotValid.into())
    }
    else{
        Ok(jsons.to_string())
    }
}


/// extract assets by protocol name from transaction id
pub fn decode_tx(rpc: &Client, txid: &Txid, protocol: &str) {
    let compact = true;
    let rawtx = rpc.get_raw_transaction(&txid, None).unwrap();
    
    match protocol.to_lowercase().as_str() {
        // ordinals
        "inscription" => {
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
                let event = match decode_ord_bitmap(inscription) {
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
                let event = match decode_ord_brc20(inscription) {
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
                let event = match decode_ord_brc100(inscription) {
                    std::result::Result::Ok(event) => println!("{:?}: {:?}", txid, event),
                    Err(err) =>{},
                } ;
            }
        }

        "brc420" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_ord_brc420(inscription) {
                    std::result::Result::Ok(event) => println!("{:?}: {:?}", txid, event),
                    Err(err) =>{},
                } ;
            }
        }

        "sns" =>{
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_ord_sns(inscription) {
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
                let event = match decode_ord_tap(inscription) {
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
                let brc20_event = match decode_atom_arc20(inscription) {
                    std::result::Result::Ok(event) => println!("{:?}: {:?}", txid, event),
                    Err(err) =>{},
                } ;
            }
        }
        "relam" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"atom");
            // let raw_envelopes = ord::RawEnvelope::from_transaction(&rawtx);
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let brc20_event = match decode_atom_relam(inscription) {
                    std::result::Result::Ok(event) => println!("{:?}: {:?}", txid, event),
                    Err(err) =>{},
                } ;
            }
        }
        "atom-nft" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"atom");
            // let raw_envelopes = ord::RawEnvelope::from_transaction(&rawtx);
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let brc20_event = match decode_atom_nft(inscription) {
                    std::result::Result::Ok(event) => println!("{:?}: {:?}", txid, event),
                    Err(err) =>{},
                } ;
            }
        }
        "atom-others" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"atom");
            // let raw_envelopes = ord::RawEnvelope::from_transaction(&rawtx);
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let brc20_event = match decode_atom_others(inscription) {
                    std::result::Result::Ok(event) => println!("{:?}: {:?}", txid, event),
                    Err(err) =>{},
                } ;
            }
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
