use {
    super::*,
    ::serde::{
        de::{Deserialize, Deserializer, EnumAccess, Visitor, VariantAccess},
        ser::{Serialize, Serializer},
    },
    std::fmt,
};

impl Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_variant(
            "Token",
            self.kind as u16 as u32,
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
                let (kind, variant) = data.variant::<SyntaxKind>()?;
                let len = variant.newtype_variant()?;
                Ok(Token { kind, len })
            }
        }

        deserializer.deserialize_enum("Token", SyntaxKind::NAMES, TokenVisitor)
    }
}
