#[repr(u16)]
#[allow(missing_docs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SyntaxKind {
    {%- for kind in terminals %}
    {{ kind | camel_case }},
    {%- endfor %}

    {%- for kind in nonterminals %}
    {{ kind | camel_case }},
    {%- endfor %}

    ERROR,
}

#[allow(bad_style, missing_docs, unreachable_pub)]
impl SyntaxKind {
    pub fn is_token(self) -> bool {
        u16::from(self) < u16::from(TokenKind::ERROR)
    }

    pub fn as_token(self) -> Option<TokenKind> {
        use std::convert::TryFrom;
        if self.is_token() {
            Some(TokenKind::try_from(u16::from(self)).unwrap())
        } else {
            None
        }
    }
}

#[allow(missing_docs)]
impl SyntaxKind {
    pub const NAMES: &'static [&'static str] = &[
        {%- for kind in all_kinds %}
        "{{ kind | camel_case }}",
        {%- endfor %}
        "ERROR",
    ];

    pub const ALL: &'static [SyntaxKind] = &[
        {%- for kind in all_kinds %}
        SyntaxKind::{{ kind | camel_case }},
        {%- endfor %}
        SyntaxKind::ERROR,
    ];

    pub const fn name(self) -> &'static str {
        SyntaxKind::NAMES[self as u16 as usize]
    }
}

#[allow(missing_docs)]
impl SyntaxKind {
    {%- for kind in all_kinds %}
    pub fn is_{{ kind | snake_case }}(self) -> bool {
        match self {
            SyntaxKind::{{ kind | camel_case }} => true,
            _ => false,
        }
    }
    {%- endfor %}
    #[allow(non_snake_case)]
    pub fn is_ERROR(self) -> bool {
        match self {
            SyntaxKind::ERROR => true,
            _ => false,
        }
    }
}

impl From<TokenKind> for SyntaxKind {
    fn from(token: TokenKind) -> SyntaxKind {
        use std::convert::TryFrom;
        if token == TokenKind::ERROR {
            SyntaxKind::ERROR
        } else {
            SyntaxKind::try_from(u16::from(token)).unwrap()
        }
    }
}

impl From<SyntaxKind> for u16 {
    fn from(kind: SyntaxKind) -> u16 {
        kind as u16
    }
}

impl std::convert::TryFrom<u16> for SyntaxKind {
    type Error = std::num::TryFromIntError;
    fn try_from(raw: u16) -> Result<SyntaxKind, Self::Error> {
        SyntaxKind::ALL.get(raw as usize).copied().ok_or_else(|| u8::try_from(-1).unwrap_err())
    }
}
