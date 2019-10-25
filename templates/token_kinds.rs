#[repr(u16)]
#[allow(missing_docs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TokenKind {
    {%- for kind in terminals %}
    {{ kind | camel_case }},
    {%- endfor %}

    ERROR,
}

#[allow(missing_docs)]
impl TokenKind {
    pub fn is_keyword(self) -> bool {
        match self {
            {% for keyword in keywords -%}
            | TokenKind::{{ keyword | camel_case }}
            {% endfor -%} => true,
            _ => false,
        }
    }

    pub fn is_literal(self) -> bool {
        match self {
            {% for literal in literals -%}
            | TokenKind::{{ literal | camel_case }}
            {% endfor -%} => true,
            _ => false,
        }
    }

    pub fn is_punctuation(self) -> bool {
        match self {
            {% for punct in punctuation -%}
            | TokenKind::{{ punct.name | camel_case }}
            {% endfor -%} => true,
            _ => false,
        }
    }

    pub fn from_keyword(ident: &str) -> Option<TokenKind> {
        match ident {
            {%- for keyword in keywords %}
            "{{ keyword }}" => Some(TokenKind::{{ keyword | camel_case }}),
            {%- endfor %}
            _ => None,
        }
    }

    pub fn from_identifier(ident: &str) -> TokenKind {
        TokenKind::from_keyword(ident).unwrap_or(TokenKind::Identifier)
    }
}

#[allow(missing_docs)]
impl TokenKind {
    pub const NAMES: &'static [&'static str] = &[
        {%- for kind in terminals %}
        "{{ kind | camel_case }}",
        {%- endfor %}
        "ERROR",
    ];

    pub const ALL: &'static [TokenKind] = &[
        {%- for kind in terminals %}
        TokenKind::{{ kind | camel_case }},
        {%- endfor %}
        TokenKind::ERROR,
    ];

    pub const fn name(self) -> &'static str {
        TokenKind::NAMES[self as u16 as usize]
    }
}

#[allow(missing_docs)]
impl TokenKind {
    {%- for kind in terminals %}
    pub fn is_{{ kind | snake_case }}(self) -> bool {
        match self {
            TokenKind::{{ kind | camel_case }} => true,
            _ => false,
        }
    }
    {%- endfor %}
    #[allow(non_snake_case)]
    pub fn is_ERROR(self) -> bool {
        match self {
            TokenKind::ERROR => true,
            _ => false,
        }
    }
}

impl From<TokenKind> for u16 {
    fn from(kind: TokenKind) -> u16 {
        kind as u16
    }
}

impl std::convert::TryFrom<u16> for TokenKind {
    type Error = std::num::TryFromIntError;
    fn try_from(raw: u16) -> Result<TokenKind, Self::Error> {
        TokenKind::ALL.get(raw as usize).copied().ok_or_else(|| u8::try_from(-1).unwrap_err())
    }
}
