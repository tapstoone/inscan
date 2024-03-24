
use ciborium;

fn main(){

    let body: Vec<u8> = vec![185, 0, 1, 100, 97, 114, 103, 115, 185, 0, 3, 107, 109, 105, 110, 116, 95, 116, 105, 99, 107, 101, 114, 100, 100, 111, 103, 101, 101, 110, 111, 110, 99, 101, 104, 55, 49, 56, 51, 53, 55, 55, 55, 100, 116, 105, 109, 101, 26, 101, 221, 101, 177];
    let payload_value: ciborium::Value = ciborium::from_reader(&body[..]).unwrap(); //TODO: get the diagnostic notation result
    println!("\n>>> atomicals inscription decoded:  {:?}", payload_value);
}