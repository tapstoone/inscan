use {
    // self::error::Error, 
    anyhow::{anyhow, bail, ensure, Context, Error},
    super::*};

pub use runestone::Runestone;

pub(crate) use {edict::Edict, etching::Etching, pile::Pile, rune::Rune, rune_id::RuneId};

pub(crate) const CLAIM_BIT: u128 = 1 << 48;
const MAX_DIVISIBILITY: u8 = 38;
pub(crate) const MAX_LIMIT: u128 = 1 << 64;

mod edict;
mod error;
mod etching;
mod pile;
mod rune;
mod rune_id;
mod runestone;
pub mod varint;

mod chain;
