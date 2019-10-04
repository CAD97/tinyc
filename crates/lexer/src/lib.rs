// For lexer implementation ideals, see
// - <https://github.com/rust-lang/rust/pull/59706>
// - <https://github.com/rust-lang/wg-grammar/issues/3#issuecomment-528256994>
//
// For the "canonical" Tiny-C lexer, see <https://gist.github.com/KartikTalwar/3095780>

pub use tinyc_grammar::SyntaxKind;

use std::fmt;

mod serde;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.len)
    }
}

pub fn tokenize(mut source: &str) -> impl Iterator<Item = Token> + '_ {
    std::iter::from_fn(move || {
        if source.is_empty() {
            return None;
        }
        let token = lex(source);
        source = &source[token.len as usize..];
        Some(token)
    })
}

pub fn lex(source: &str) -> Token {
    debug_assert!(!source.is_empty());
    if source.starts_with(is_whitespace) {
        Token {
            kind: SyntaxKind::Whitespace,
            len: source.find(is_not_whitespace).unwrap_or(source.len()) as u32,
        }
    } else if source.starts_with(is_digit) {
        Token {
            kind: SyntaxKind::Integer,
            len: source.find(is_not_digit).unwrap_or(source.len()) as u32,
        }
    } else if source.starts_with(is_ident) {
        let len = source.find(is_not_ident).unwrap_or(source.len());
        Token {
            kind: SyntaxKind::from_identifier(&source[..len]),
            len: len as u32,
        }
    } else {
        Token {
            kind: match source.chars().next() {
                Some('{') => SyntaxKind::LeftCurlyBracket,
                Some('}') => SyntaxKind::RightCurlyBracket,
                Some('(') => SyntaxKind::LeftParenthesis,
                Some(')') => SyntaxKind::RightParenthesis,
                Some('+') => SyntaxKind::PlusSign,
                Some('-') => SyntaxKind::HyphenMinus,
                Some('<') => SyntaxKind::LessThanSign,
                Some(';') => SyntaxKind::Semicolon,
                Some('=') => SyntaxKind::EqualsSign,
                _ => panic!(
                    "lex error: {}",
                    source
                        .chars()
                        .next()
                        .map(|c| c.escape_unicode().to_string())
                        .unwrap_or("unexpected eof".to_string())
                ),
            },
            len: 1,
        }
    }
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

fn is_not_whitespace(c: char) -> bool {
    !is_whitespace(c)
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_not_digit(c: char) -> bool {
    !is_digit(c)
}

fn is_ident(c: char) -> bool {
    'a' <= c && c <= 'z'
}

fn is_not_ident(c: char) -> bool {
    !is_ident(c)
}
