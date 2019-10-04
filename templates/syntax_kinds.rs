{%- set empty = [] -%}
{%- set all_kinds = empty
    | concat(with=keywords)
    | concat(with=literals)
    | concat(with=punctuation | map(attribute="name"))
    | concat(with=tokens)
-%}

#[repr(u16)]
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(bad_style, missing_docs, unreachable_pub)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SyntaxKind {
    {%- for kind in all_kinds %}
    {{ kind | camel_case }},
    {%- endfor %}

    // technical, temporary SyntaxKinds
    #[doc(hidden)] EOF,
    #[doc(hidden)] TOMBSTONE,
    #[doc(hidden)] LAST,
}

#[allow(bad_style, missing_docs, unreachable_pub)]
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        match self {
            {% for keyword in keywords -%}
            | SyntaxKind::{{ keyword | camel_case }}
            {% endfor -%}
            => true,
            _ => false,
        }
    }

    pub fn is_literal(self) -> bool {
        match self {
            {% for literal in literals -%}
            | SyntaxKind::{{ literal | camel_case }}
            {% endfor -%}
            => true,
            _ => false,
        }
    }

    pub fn is_punctuation(self) -> bool {
        match self {
            {% for punct in punctuation -%}
            | SyntaxKind::{{ punct.name | camel_case }}
            {% endfor -%}
            => true,
            _ => false,
        }
    }

    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        match ident {
            {% for keyword in keywords -%}
            "{{ keyword }}" => Some(SyntaxKind::{{ keyword | camel_case }}),
            {% endfor -%}
            _ => None,
        }
    }

    pub fn from_identifier(ident: &str) -> SyntaxKind {
        SyntaxKind::from_keyword(ident).unwrap_or(SyntaxKind::Identifier)
    }
}

// These deliberately omit `EOF`, `TOMBSTONE`, and `LAST`, which should be transient.
#[allow(bad_style, missing_docs, unreachable_pub)]
impl SyntaxKind {
    pub const NAMES: &'static [&'static str] = &[
        {%- for kind in all_kinds %}
        "{{ kind | camel_case }}",
        {%- endfor %}
    ];

    pub const ALL: &'static [SyntaxKind] = &[
        {%- for kind in all_kinds %}
        SyntaxKind::{{ kind | camel_case }},
        {%- endfor %}
    ];

    pub const fn name(self) -> &'static str {
        SyntaxKind::NAMES[self as u16 as usize]
    }
}

#[allow(bad_style, missing_docs, unreachable_pub)]
impl SyntaxKind {
    {%- for kind in all_kinds %}
    pub fn is_{{ kind | snake_case }}(self) -> bool {
        match self {
            SyntaxKind::{{ kind | camel_case }} => true,
            _ => false,
        }
    }
    {%- endfor %}
}
