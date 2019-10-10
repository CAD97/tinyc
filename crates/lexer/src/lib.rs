use std::u32;
// Re-export for ease of use.
pub use tinyc_grammar::SyntaxKind;

mod serde;

/// A single token in the document stream.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Token {
    /// The kind of token.
    pub kind: SyntaxKind,
    /// How many bytes this token is.
    pub len: u32,
}

/// Convenience function for repeatedly applying `lex`.
pub fn tokenize(mut source: &str) -> impl Iterator<Item = Token> + '_ {
    // Our compiler tooling assumes source files < 4GiB in size.
    assert!(source.len() < u32::MAX as usize);
    std::iter::from_fn(move || {
        if source.is_empty() {
            return None;
        }
        let token = lex(source);
        source = &source[token.len as usize..];
        Some(token)
    })
}

/// Lex the first token off of the source string.
pub fn lex(source: &str) -> Token {
    debug_assert!(!source.is_empty());
    debug_assert!(source.len() < u32::MAX as usize);
    // Classify the token.
    if source.starts_with(is_whitespace) {
        // Whitespace token.
        Token {
            kind: SyntaxKind::Whitespace,
            len: source
                .find(is_not_whitespace)
                .unwrap_or_else(|| source.len()) as u32,
        }
    } else if source.starts_with(is_digit) {
        // Integer token.
        Token {
            kind: SyntaxKind::Integer,
            len: source.find(is_not_digit).unwrap_or_else(|| source.len()) as u32,
        }
    } else if source.starts_with(is_ident) {
        // Identifier token.
        let len = source.find(is_not_ident).unwrap_or_else(|| source.len());
        Token {
            // This is a new function on `SyntaxKind` we'll add next.
            kind: SyntaxKind::from_identifier(&source[..len]),
            len: len as u32,
        }
    } else {
        // Punctuation token.
        let ch = source.chars().next().unwrap();
        Token {
            kind: match ch {
                '{' => SyntaxKind::LeftCurlyBracket,
                '}' => SyntaxKind::RightCurlyBracket,
                '(' => SyntaxKind::LeftParenthesis,
                ')' => SyntaxKind::RightParenthesis,
                '+' => SyntaxKind::PlusSign,
                '-' => SyntaxKind::HyphenMinus,
                '<' => SyntaxKind::LessThanSign,
                ';' => SyntaxKind::Semicolon,
                '=' => SyntaxKind::EqualsSign,
                // Unknown tokens are an error.
                _ => SyntaxKind::Error,
            },
            len: ch.len_utf8() as u32,
        }
    }
}

// Helper functions for classifying characters.
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
