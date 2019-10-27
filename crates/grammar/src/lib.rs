use serde::ser::{Serialize, Serializer};

include!(concat!(env!("OUT_DIR"), "/token_kinds.rs"));
include!(concat!(env!("OUT_DIR"), "/syntax_kinds.rs"));

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_variant(
            "Token",
            u16::from(self.kind).into(),
            self.kind.name(),
            &self.len,
        )
    }
}
