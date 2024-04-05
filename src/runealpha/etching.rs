use super::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Default, Serialize, Debug, PartialEq, Copy, Clone)]
pub struct Etching {
  pub(crate) divisibility: u8,
  pub(crate) limit: Option<u128>,
  pub(crate) rune: Rune,
  pub(crate) symbol: Option<char>,
  pub(crate) term: Option<u64>,
}
