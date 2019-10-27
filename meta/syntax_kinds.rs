{%- set empty = [] -%}
// Separate variables for terminals and nonterminals
{%- set terminals = empty
    | concat(with=keywords)
    | concat(with=literals)
    | concat(with=punctuation | map(attribute="name"))
    | concat(with=tokens)
-%}
{%- set all_kinds = empty
    | concat(with=terminals)
    | concat(with=nonterminals)
-%}

#[repr(u16)]
#[allow(missing_docs)]
// Add serde to runtime dependencies.
// TokenKind should impl `Serialize` as well.
#[derive(serde::Serialize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SyntaxKind {
    {%- for kind in all_kinds %}
    {{ kind | camel_case }},
    {%- endfor %}
    ERROR,
}

impl SyntaxKind {
    /// The name of this syntax kind.
    pub fn name(self) -> &'static str {
        match self {
            {%- for kind in all_kinds %}
            SyntaxKind::{{ kind | camel_case }} => "{{ kind | camel_case }}",
            {%- endfor %}
            SyntaxKind::ERROR => "ERROR",
        }
    }
}

// We need to convert from TokenKind to SyntaxKind.
// This could be explicitly zero-cost by using a transmute,
// but we stick here to the safe version and let it optimize.
impl From<TokenKind> for SyntaxKind {
    fn from(token: TokenKind) -> SyntaxKind {
        match token {
            {%- for kind in terminals %}
            TokenKind::{{ kind | camel_case }} => SyntaxKind::{{ kind | camel_case }},
            {%- endfor %}
            TokenKind::ERROR => SyntaxKind::ERROR,
        }
    }
}

// For rowan, we also want conversions to/from u16.
// We also want these impls for TokenKind as well.
impl From<SyntaxKind> for u16 {
    fn from(kind: SyntaxKind) -> u16 {
        kind as u16
    }
}

// This could be explicitly zero-cost by using a transmute,
// but we stick here to the safe version and let it optimize.
impl From<u16> for SyntaxKind {
    fn from(raw: u16) -> SyntaxKind {
        match raw {
            {%- for kind in all_kinds %}
            {{ loop.index }} => SyntaxKind::{{ kind | camel_case }},
            {%- endfor %}
            _ => SyntaxKind::ERROR,
        }
    }
}
