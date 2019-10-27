{%- set empty = [] -%}
{%- set all_kinds = empty
    | concat(with=keywords)
    | concat(with=literals)
    | concat(with=punctuation | map(attribute="name"))
    | concat(with=tokens)
-%}

#[repr(u16)]
#[allow(missing_docs)]
#[derive(serde::Serialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TokenKind {
    {%- for kind in all_kinds %}
    // List each kind in camel_case
    {{ kind | camel_case }},
    {%- endfor %}

    ERROR,
}

impl TokenKind {
    /// The token kind for a keyword.
    pub fn from_keyword(ident: &str) -> Option<TokenKind> {
        match ident {
            {%- for keyword in keywords %}
            "{{ keyword }}" => Some(TokenKind::{{ keyword | camel_case }}),
            {%- endfor %}
            _ => None,
        }
    }

    /// The token kind for an identifer.
    ///
    /// Note that this doesn't do any validation of the identifier,
    /// it just uses whatever you give it.
    pub fn from_identifier(ident: &str) -> TokenKind {
        TokenKind::from_keyword(ident).unwrap_or(TokenKind::Identifier)
    }
}

impl TokenKind {
    /// The name of this token kind.
    pub fn name(self) -> &'static str {
        match self {
            {%- for kind in all_kinds %}
            TokenKind::{{ kind | camel_case }} => "{{ kind | camel_case }}",
            {%- endfor %}
            TokenKind::ERROR => "ERROR",
        }
    }
}

impl From<TokenKind> for u16 {
    fn from(kind: TokenKind) -> u16 {
        kind as u16
    }
}

impl From<u16> for TokenKind {
    fn from(raw: u16) -> TokenKind {
        match raw {
            {%- for kind in all_kinds %}
            {{ loop.index }} => TokenKind::{{ kind | camel_case }},
            {%- endfor %}
            _ => TokenKind::ERROR,
        }
    }
}
