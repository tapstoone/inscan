use {super::*, std::num::TryFromIntError};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use anyhow::{self, Error};

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq, Ord, PartialOrd)]
pub(crate) struct RuneId {
  pub(crate) height: u32,
  pub(crate) index: u16,
}

impl TryFrom<u128> for RuneId {
  type Error = TryFromIntError;

  fn try_from(n: u128) -> Result<Self, Self::Error> {
    Ok(Self {
      height: u32::try_from(n >> 16)?,
      index: u16::try_from(n & 0xFFFF).unwrap(),
    })
  }
}

impl From<RuneId> for u128 {
  fn from(id: RuneId) -> Self {
    u128::from(id.height) << 16 | u128::from(id.index)
  }
}

impl Display for RuneId {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}/{}", self.height, self.index,)
  }
}

impl FromStr for RuneId {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (height, index) = s
      .split_once('/')
      .ok_or_else(|| anyhow!("invalid rune ID: {s}"))?;

    Ok(Self {
      height: height.parse()?,
      index: index.parse()?,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rune_id_to_128() {
    assert_eq!(
      0b11_0000_0000_0000_0001u128,
      RuneId {
        height: 3,
        index: 1,
      }
      .into()
    );
  }

  #[test]
  fn display() {
    assert_eq!(
      RuneId {
        height: 1,
        index: 2
      }
      .to_string(),
      "1/2"
    );
  }

  #[test]
  fn from_str() {
    assert!("/".parse::<RuneId>().is_err());
    assert!("1/".parse::<RuneId>().is_err());
    assert!("/2".parse::<RuneId>().is_err());
    assert!("a/2".parse::<RuneId>().is_err());
    assert!("1/a".parse::<RuneId>().is_err());
    assert_eq!(
      "1/2".parse::<RuneId>().unwrap(),
      RuneId {
        height: 1,
        index: 2
      }
    );
  }

  #[test]
  fn try_from() {
    assert_eq!(
      RuneId::try_from(0x060504030201).unwrap(),
      RuneId {
        height: 0x06050403,
        index: 0x0201
      }
    );

    assert!(RuneId::try_from(0x07060504030201).is_err());
  }
}
