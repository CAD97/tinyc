use {
    serde::{
        de::{Deserialize, Deserializer, EnumAccess, VariantAccess, Visitor},
        ser::{Serialize, Serializer},
    },
    std::fmt,
};

include!(concat!(env!("OUT_DIR"), "/syntax_kinds.rs"));
include!(concat!(env!("OUT_DIR"), "/token_kinds.rs"));

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

impl<'de> Deserialize<'de> for Token {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TokenVisitor;

        impl<'de> Visitor<'de> for TokenVisitor {
            type Value = Token;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "an enum")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (kind, variant) = data.variant::<TokenKind>()?;
                let len = variant.newtype_variant()?;
                Ok(Token { kind, len })
            }
        }

        deserializer.deserialize_enum("Token", TokenKind::NAMES, TokenVisitor)
    }
}
