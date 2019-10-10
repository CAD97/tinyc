// This is just a workaround: for some reason
// a tera filter expression cannot start with a literal empty array
{%- set empty = [] -%}
// Create a variable for accessing every kind
// by concatenating each individual kind
{%- set all_kinds = empty
    | concat(with=keywords)
    | concat(with=literals)
    | concat(with=punctuation | map(attribute="name"))
    | concat(with=tokens)
-%}

// Rowan internally stores kind as a u16
#[repr(u16)]
// We won't be generating docs
#[allow(missing_docs)]
// Derive all of the standard traits we can
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SyntaxKind {
    {%- for kind in all_kinds %}
    // List each kind in camel_case
    {{ kind | camel_case }},
    {%- endfor %}
}

impl SyntaxKind {
    /// The syntax kind for a keyword.
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        match ident {
            {% for keyword in keywords -%}
            "{{ keyword }}" => Some(SyntaxKind::{{ keyword | camel_case }}),
            {% endfor -%}
            _ => None,
        }
    }

    /// The syntax kind for an identifer.
    ///
    /// Note that this doesn't do any validation of the identifier,
    /// it just uses whatever you give it.
    pub fn from_identifier(ident: &str) -> SyntaxKind {
        SyntaxKind::from_keyword(ident).unwrap_or(SyntaxKind::Identifier)
    }
}

impl SyntaxKind {
    /// The name of this syntax kind.
    pub fn name(self) -> &'static str {
        match self {
            {% for kind in all_kinds -%}
            SyntaxKind::{{ kind | camel_case }} => "{{ kind | camel_case }}",
            {% endfor -%}
            #[allow(unreachable_patterns)]
            _ => "", // For the future
        }
    }
}
