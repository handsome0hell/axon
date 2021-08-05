// Generated by Molecule 0.7.0

use super::super::common::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct SidechainFeeCellLockArgs(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for SidechainFeeCellLockArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for SidechainFeeCellLockArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for SidechainFeeCellLockArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "chain_id", self.chain_id())?;
        write!(f, ", {}: {}", "surplus", self.surplus())?;
        write!(f, " }}")
    }
}
impl ::core::default::Default for SidechainFeeCellLockArgs {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        SidechainFeeCellLockArgs::new_unchecked(v.into())
    }
}
impl SidechainFeeCellLockArgs {
    pub const FIELD_COUNT: usize = 2;
    pub const FIELD_SIZES: [usize; 2] = [4, 16];
    pub const TOTAL_SIZE: usize = 20;

    pub fn chain_id(&self) -> ChainId {
        ChainId::new_unchecked(self.0.slice(0..4))
    }

    pub fn surplus(&self) -> Uint128 {
        Uint128::new_unchecked(self.0.slice(4..20))
    }

    pub fn as_reader<'r>(&'r self) -> SidechainFeeCellLockArgsReader<'r> {
        SidechainFeeCellLockArgsReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for SidechainFeeCellLockArgs {
    type Builder = SidechainFeeCellLockArgsBuilder;

    const NAME: &'static str = "SidechainFeeCellLockArgs";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        SidechainFeeCellLockArgs(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SidechainFeeCellLockArgsReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SidechainFeeCellLockArgsReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder().chain_id(self.chain_id()).surplus(self.surplus())
    }
}
#[derive(Clone, Copy)]
pub struct SidechainFeeCellLockArgsReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for SidechainFeeCellLockArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for SidechainFeeCellLockArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for SidechainFeeCellLockArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "chain_id", self.chain_id())?;
        write!(f, ", {}: {}", "surplus", self.surplus())?;
        write!(f, " }}")
    }
}
impl<'r> SidechainFeeCellLockArgsReader<'r> {
    pub const FIELD_COUNT: usize = 2;
    pub const FIELD_SIZES: [usize; 2] = [4, 16];
    pub const TOTAL_SIZE: usize = 20;

    pub fn chain_id(&self) -> ChainIdReader<'r> {
        ChainIdReader::new_unchecked(&self.as_slice()[0..4])
    }

    pub fn surplus(&self) -> Uint128Reader<'r> {
        Uint128Reader::new_unchecked(&self.as_slice()[4..20])
    }
}
impl<'r> molecule::prelude::Reader<'r> for SidechainFeeCellLockArgsReader<'r> {
    type Entity = SidechainFeeCellLockArgs;

    const NAME: &'static str = "SidechainFeeCellLockArgsReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        SidechainFeeCellLockArgsReader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len != Self::TOTAL_SIZE {
            return ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct SidechainFeeCellLockArgsBuilder {
    pub(crate) chain_id: ChainId,
    pub(crate) surplus:  Uint128,
}
impl SidechainFeeCellLockArgsBuilder {
    pub const FIELD_COUNT: usize = 2;
    pub const FIELD_SIZES: [usize; 2] = [4, 16];
    pub const TOTAL_SIZE: usize = 20;

    pub fn chain_id(mut self, v: ChainId) -> Self {
        self.chain_id = v;
        self
    }

    pub fn surplus(mut self, v: Uint128) -> Self {
        self.surplus = v;
        self
    }
}
impl molecule::prelude::Builder for SidechainFeeCellLockArgsBuilder {
    type Entity = SidechainFeeCellLockArgs;

    const NAME: &'static str = "SidechainFeeCellLockArgsBuilder";

    fn expected_length(&self) -> usize {
        Self::TOTAL_SIZE
    }

    fn write<W: ::molecule::io::Write>(&self, writer: &mut W) -> ::molecule::io::Result<()> {
        writer.write_all(self.chain_id.as_slice())?;
        writer.write_all(self.surplus.as_slice())?;
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        SidechainFeeCellLockArgs::new_unchecked(inner.into())
    }
}
