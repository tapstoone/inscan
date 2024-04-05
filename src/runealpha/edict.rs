use super::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};


#[derive(Default, Serialize, Debug, PartialEq, Copy, Clone)]
pub struct Edict {
  pub id: u128,
  pub amount: u128,
  pub output: u128,
}
