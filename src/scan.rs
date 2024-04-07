use {
    crate::ord::{self, Inscription, InscriptionId, ParsedEnvelope},
    crate::runealpha::{self, Runestone as Runealpha},
    anyhow::{Error, Ok, Result},
    base64,
    bitcoin::{
        block,
        blockdata::{opcodes::all::OP_CHECKMULTISIG, script::Instruction},
        Transaction, Txid,
    },
    bitcoincore_rpc::{Client, RpcApi},
    ciborium,
    crypto::{rc4::Rc4, symmetriccipher::SynchronousStreamCipher},
    serde::{Deserialize, Serialize},
    serde_json::{self, Value},
    std::{
        iter::repeat,
        str::{self, FromStr},
        fs::File,
        io::{BufWriter, Write},
        fs::OpenOptions,
        thread,
        time::Duration,
    },
    thiserror,
    sqlx::postgres::PgPoolOptions,
    futures::executor::block_on,
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

fn decode_ord(rawtx:Transaction)->Result<serde_json::Value>{
    // let compact = true;
    let parsed_inscriptions = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
    let result = Box::new(CompactOutput {
        inscriptions: parsed_inscriptions
            .clone()
            .into_iter()
            .map(|inscription| inscription.payload.try_into())
            .collect::<Result<Vec<CompactInscription>>>()
            .unwrap(),
    });
    if !result.inscriptions.is_empty(){
        // println!("\n{:?}: {:?}", txid, result);
        return Ok(serde_json::to_value(result)?);
    }else{
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

fn decode_ord_brc20(inscription: Inscription) ->Result<serde_json::Value> {
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
        // let brc20_event = serde_json::to_string(&value).map_err(|err| Error::from(err))?;
        return Ok(value);
    } else {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

fn decode_ord_brc100(inscription: Inscription) ->Result<serde_json::Value> {
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
        // let brc20_event = serde_json::to_string(&value).map_err(|err| Error::from(err))?;
        return Ok(value);
    } else {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

fn decode_ord_brc420(inscription: Inscription) ->Result<serde_json::Value> {
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
        return Ok(serde_json::json!({"mint":content_body.to_string()}));
    }
    let value = parse_json(content_body).ok_or_else(|| BRC20Error::ContentBodyNotJson)?;
    // deploy new collection
    let protocol = value.get("p").ok_or_else(|| BRC20Error::ContentBodyNotJson)?;
    if protocol == "brc-420" {
        // let brc20_event = serde_json::to_string(&value).map_err(|err| Error::from(err))?;
        return Ok(value);
    } else {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

fn decode_ord_sns(inscription: Inscription) ->Result<serde_json::Value> {
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
        // let brc20_event = serde_json::to_string(&value).map_err(|err| Error::from(err))?;
        return Ok(value);
    } else {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

// https://github.com/BennyTheDev/tap-protocol-specs
fn decode_ord_tap(inscription: Inscription) ->Result<serde_json::Value> {
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
        // let brc20_event = serde_json::to_string(&value).map_err(|err| Error::from(err))?;
        return Ok(value);
    } else {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

//TODO nft transfer is bind with ordinals number, not the nft self?
fn decode_ord_bitmap(inscription: Inscription) ->Result<serde_json::Value> {
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
        return Ok(serde_json::json!({"mint":content_body.to_string()}));
    } else {
        return Err(BRC20Error::ContentTypeNotValid.into());
    }
}

fn decode_atom_arc20(inscription: Inscription)->Result<serde_json::Value>{
    let op = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    // println!("operation: {:?}", op);
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
    let cbor_value: ciborium::Value = ciborium::de::from_reader(&body_string[..])?; //TODO: get the diagnostic notation result
    let json_value = cbor_to_json(cbor_value);
    // let temp = payload_value.as_map().unwrap();
    // println!("\n>>> atomicals inscription decoded: {:?}", temp);
    Ok(json_value)
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
fn decode_atom_relam(inscription: Inscription)->Result<serde_json::Value>{
    let op = inscription.content_type().ok_or_else(|| BRC20Error::ContentTypeNull)?;
    // println!("operation: {:?}", op);
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
        Ok(jsons)
    }
    
    // println!("\n>>> atomicals inscription decoded: {:?}", jsons.to_string());
    // let temp = payload_value.as_map().unwrap();
    // println!("\n>>> atomicals inscription decoded: {:?}", temp);
    
}


fn decode_atom_nft(inscription: Inscription)->Result<serde_json::Value>{
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
        Ok(jsons)
    }
    
}

fn decode_atom_others(inscription: Inscription)->Result<serde_json::Value>{
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
    println!("{:?}", jsons);
    // check if request_realm and request_subrealm in keys
    let request_realm = &jsons["args"]["request_realm"];
    let request_subrealm = &jsons["args"]["request_subrealm"];
    if !request_realm.is_null() || !request_subrealm.is_null() {
        Err(BRC20Error::ContentTypeNotValid.into())
    }
    else{
        Ok(jsons)
    }
}


// 1. find all the multisig ouput in ouputs
// 2. take the first two ouput  
pub fn decode_stamp_src20(rawtx: &Transaction) ->Result<serde_json::Value> {
    let tx_input0 = rawtx.input[0].previous_output.txid.to_string();
    let signing_key = hex::decode(&tx_input0).unwrap();
    // println!("signing_key: {:#?}", key);


    let mut payload = Vec::new();

    let mut pubkeys_idx:u8 = 1;
    for (i, output) in rawtx.output.iter().enumerate(){
        // check if output is multisig
        if pubkeys_idx>2 { continue; }
        let encoded_script = &output.script_pubkey;

        let mut instructions_check  =  encoded_script.instructions().peekable();
        let is_multisig = instructions_check.any(|instruction| {
            matches!(&instruction, std::result::Result::Ok(Instruction::Op(OP_CHECKMULTISIG)))
        });

        if  !is_multisig{continue;}

        let mut instructions  =  encoded_script.instructions().peekable();
        let mut insctrction_idx:u8 = 1;
        while let Some(insctrction) = instructions.next().transpose()?{
            if insctrction_idx > 2 {break;}
            // TODO: check whether opcode is MULTISIG, // Take the first two pubkeys from all present multisig scripts
            // if let Some(pushed_op) = &insctrction.opcode(){
            // }
            if let Some(pushed_bytes) = insctrction.push_bytes() {
                // payload.push(pushed_bytes);
                let value = pushed_bytes.as_bytes().to_vec();
                payload.extend_from_slice(&value[1..value.len()-1]);
                insctrction_idx += 1;
            }
            // println!("{:?}", insctrction);
        }
        pubkeys_idx +=1;
    }
    if payload.is_empty(){
        return Err(BRC20Error::ContentBodyNull.into());
    }

    let mut rc4 = Rc4::new(&signing_key);
    let mut decode_result: Vec<u8> = repeat(0).take(payload.len()).collect();
    rc4.process(&payload, &mut decode_result);

    // strip and decode hex to string
    let len = &decode_result[0..2];
    let protocol = str::from_utf8(&decode_result[2..2+6])?;
    if protocol == "stamp:" {
        let result = str::from_utf8(&decode_result[2+6..])?;
        // return Ok(result.to_string())
        return Ok(serde_json::from_str(result)?)
    }
    else {
        Err(BRC20Error::ContentBodyNull.into())
    }
    // println!("\nlens: {:?}, protocol: {:?}", len, );
    // println!(">>> stamps(src20) inscription decoded: {:#?}", str::from_utf8(&decode_result[2+6..]).unwrap());

    // Ok("aaa".to_string())
}

// fn decode_rune_stone(rawtx: &Transaction)->Result<String>{
//     let aa = Runestone::from_transaction(rawtx);
//     println!("rune stone: {:?}",aa);
//     Ok("t".to_lowercase())
// }

fn decode_rune_alpha(rawtx: &Transaction)->Result<serde_json::Value>{
    // let rune = Runealpha::from_transaction(rawtx).ok_or("name");
    let rune = Runealpha::from_transaction(rawtx).ok_or_else(|| BRC20Error::ContentTypeNull)?;
    // let json_string = serde_json::to_string(&rune).expect("Serialization failed");
    let json_value = serde_json::to_value(&rune)?;
    // println!("rune alpha: {:?}",json_value);
    Ok(json_value)
}


/// extract assets by protocol name from transaction id, should return Option<Vec<Value>>
pub fn decode_tx(rpc: &Client, txid: &Txid, protocol: &str) -> Vec<serde_json::Value>{
    let rawtx = rpc.get_raw_transaction(&txid, None).unwrap();
    let mut events: Vec<serde_json::Value> = Vec::new();
    
    match protocol.to_lowercase().as_str() {
        "ord" => {
            let event = match decode_ord(rawtx) {
                std::result::Result::Ok(event) => {
                    // println!("{:?}: {:?}", txid, event);
                    events.push(serde_json::json!({"protocol":"ord", "payload":event}));
                },
                Err(err) =>{},
            } ;
        }

        "ord-bitmap" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_ord_bitmap(inscription) {
                    std::result::Result::Ok(event) => {
                        events.push(serde_json::json!({"protocol":"ord-bitmap", "payload":event}));
                        // println!("{:?}: {:?}", txid, event);
                    },
                    Err(err) =>{},
                } ;
            }
        }

        "ord-brc20" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_ord_brc20(inscription) {
                    std::result::Result::Ok(event) => {
                        // println!("{:?}: {:?}", txid, event);
                        events.push(serde_json::json!({"protocol":"ord-brc20", "payload":event}));

                        // write_jsonl(event, "temp.jsonl");
                    },
                    Err(err) =>{},
                } ;
            }
        }

        "ord-brc100" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_ord_brc100(inscription) {
                    std::result::Result::Ok(event) => {
                        // println!("{:?}: {:?}", txid, event);
                        events.push(serde_json::json!({"protocol":"ord-brc100", "payload":event}));
                    },
                    Err(err) =>{},
                } ;
            }
        }

        "ord-brc420" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_ord_brc420(inscription) {
                    std::result::Result::Ok(event) => 
                    {
                        events.push(serde_json::json!({"protocol":"ord-brc420", "payload":event}));
                        // println!("{:?}: {:?}", txid, event);
                    },
                    Err(err) =>{},
                } ;
            }
        }

        "ord-sns" =>{
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_ord_sns(inscription) {
                    std::result::Result::Ok(event) => {
                        events.push(serde_json::json!({"protocol":"ord-sns", "payload":event}));
                        // println!("{:?}: {:?}", txid, event)
                    },
                    Err(err) =>{},
                } ;
            }
        }

        "ord-tap" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"ord");
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_ord_tap(inscription) {
                    std::result::Result::Ok(event) => {
                        events.push(serde_json::json!({"protocol":"ord-tap", "payload":event}));
                        // println!("{:?}: {:?}", txid, event)
                    },
                    Err(err) =>{},
                };
            }
        }

        // ===Atomicals===
        "atom-arc20" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"atom");
            // let raw_envelopes = ord::RawEnvelope::from_transaction(&rawtx);
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let brc20_event = match decode_atom_arc20(inscription) {
                    std::result::Result::Ok(event) => {
                        // println!("{:?}: {:?}", txid, event);
                        events.push(serde_json::json!({"protocol":"ord-arc20", "payload":event}));
                    },
                    Err(err) =>{},
                } ;
            }
        }
        "atom-relam" => {
            let envelopes = ord::ParsedEnvelope::from_transaction(&rawtx, b"atom");
            // let raw_envelopes = ord::RawEnvelope::from_transaction(&rawtx);
            for item in envelopes.iter() {
                // let body = item.clone().payload.body.unwrap();
                let inscription = item.payload.clone();
                let event = match decode_atom_relam(inscription) {
                    std::result::Result::Ok(event) => {
                        // println!("{:?}: {:?}", txid, event),
                        events.push(serde_json::json!({"protocol":"atom-relam", "payload":event}));
                    },
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
                let _ = match decode_atom_nft(inscription) {
                    std::result::Result::Ok(event) => {
                        events.push(serde_json::json!({"protocol":"atom-nft", "payload":event}));
                        // println!("{:?}: {:?}", txid, event)
                    },
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
                let _ = match decode_atom_others(inscription) {
                    std::result::Result::Ok(event) => {
                        events.push(serde_json::json!({"protocol":"atom-others", "payload":event}));
                        // println!("{:?}: {:?}", txid, event)
                    },
                    Err(err) =>{},
                } ;
            }
        }

        // ===stamps===
        "stamp-src20" =>{
            match decode_stamp_src20(&rawtx) {
                // std::result::Result::Ok(event)=>println!("{:?}: {:?}", txid, event),
                std::result::Result::Ok(event) => {
                    events.push(serde_json::json!({"protocol":"stamp-src20", "payload":event}));
                }
                Err(err) => {}
            }
        }

        // ===runes===
        "rune-alpha" => {
            match decode_rune_alpha(&rawtx) {
                std::result::Result::Ok(event) => {
                    events.push(serde_json::json!({"protocol":"rune-alpha", "payload":event}));
                },
                Err(err) => {}
            }
        }
        // "rune-stone" =>{
        //     match decode_rune_stone(&rawtx) {
        //         std::result::Result::Ok(event)=>println!("{:?}: {:?}", txid, event),
        //         Err(err) => {}
        //     }
        // }
        // "rune-alpha" => {
        //     match decode_rune_alpha(&rawtx) {
        //         std::result::Result::Ok(event) => {
        //             println!("{:?}: {:?}", txid, event);
        //             let _ = block_on(save_to_pg(&event));
        //         },
        //         Err(err) => {}
        //     }
        // }
        _ => {
            // Default case if none of the above match
            println!("Unknown Protocol Name {:?}", protocol);
        }
    }
    return events
}



fn write_jsonl(value: Value, file_path: &str) -> Result<(), Error> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;
    // let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &value)?;
    writeln!(&mut writer)?;
    writer.flush()?;
    Ok(())
}


fn split_string(input_string: &str, delimiter: &str) -> Vec<String> {
    if input_string.contains(delimiter) {
        input_string.split(delimiter).map(|s| s.to_string()).collect()
    } else {
        vec![input_string.to_string()]
    }
}

pub fn run_txs(rpc: &Client, txids: &String, protocol: &str, output:&String) {
    let txs = split_string(&txids, ",");
    for tx in txs{
        let txid = Txid::from_str(&tx).unwrap();
        let result = decode_tx(rpc, &txid, protocol);
        for evt in result{
            let data = serde_json::json!({
                "txhash": txid,
                "protocol":evt.get("protocol").unwrap(),
                "payload":evt.get("payload").unwrap(),
            });
            let _ = write_jsonl(data, output);
        }
    }
}

pub fn run_blocks(rpc: &Client, block_number: &String, protocol: &str, output:&String) {
    let blocks:Vec<u64> = if block_number.contains(","){
        let blocks_str = split_string(&block_number, ",");
        blocks_str.iter().map(|s| s.parse::<u64>().unwrap()).collect()
    }
    else if block_number.contains(":") {
        let txs = split_string(&block_number, ":");
        let start: u64 = txs[0].parse().unwrap();
        let stop: u64 = txs[1].parse().unwrap();
        (start..=stop).collect()
    } else{
        vec![block_number.parse::<u64>().unwrap()]
    };

    // iterate over the blocks
    for block in blocks {
        let block_hash = rpc.get_block_hash(block).unwrap();
        
        let block_data = rpc.get_block(&block_hash).unwrap();
        let timestamp = block_data.header.time;
        for (idx, tx) in block_data.txdata.iter().enumerate() {
            let txid = tx.txid();
            let result = decode_tx(rpc, &txid, protocol);
            for evt in result{
                let data = serde_json::json!({
                    "height": block,
                    "blocktime": timestamp,
                    "txhash": txid,
                    "txindex": idx,
                    "protocol":evt.get("protocol").unwrap(),
                    "payload":evt.get("payload").unwrap(),
                });
                let _ = write_jsonl(data, output);
            }
        }
    }
}


async fn save_to_pg(json_value: &Value) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:postgres@localhost/postgres")
    .await?;

    sqlx::query("INSERT INTO people (address) VALUES ($1)")
    .bind(json_value)
    .execute(&pool)
    .await?;

    std::result::Result::Ok(())
}

fn get_current_height()->u64 {
    return 545;
}

///scan all blocks and save to postgresql
pub fn scan_all(rpc: &Client, start_height:u64, output:&String){
    let pg_height = get_current_height();
    let mut current_height = if pg_height > start_height { pg_height } else { start_height };
    loop {
        let rpc_height = rpc.get_block_count().unwrap();
        if current_height > rpc_height{
            thread::sleep(Duration::from_secs(1)); // sleep 2sec
            println!("best height {:?}, waiting for {:?}, sleep 1 sec...", rpc_height, rpc_height+1);
        } else{
            println!("processing the height {:?}/{:?}...", current_height, rpc_height);
            // process current_block
            run_blocks(rpc, &current_height.to_string(), "ord-brc20", output);
            current_height += 1;
    
        }

    }
}