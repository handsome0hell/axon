use core::convert::TryInto;

use crate::{decode_u8, pattern::Pattern, FromRaw};

#[derive(Debug)]
pub struct CheckerQuitSidechainWitness {
    pattern:        Pattern,
    pub chain_id:   u8,
    pub checker_id: u8,
}

impl FromRaw for CheckerQuitSidechainWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerQuitSidechainWitness> {
        if witness_raw_data.len() < 3 {
            return None;
        }

        let pattern = decode_u8(&witness_raw_data[0..1])?.try_into().ok()?;
        let chain_id = decode_u8(&witness_raw_data[1..2])?;
        let checker_id = decode_u8(&witness_raw_data[2..3])?;

        Some(CheckerQuitSidechainWitness {
            pattern,
            chain_id,
            checker_id,
        })
    }
}
