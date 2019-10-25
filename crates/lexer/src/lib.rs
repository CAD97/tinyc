// For lexer implementation ideals, see
// - <https://github.com/rust-lang/rust/pull/59706>
// - <https://github.com/rust-lang/wg-grammar/issues/3#issuecomment-528256994>
//
// For the "canonical" Tiny-C lexer, see <https://gist.github.com/KartikTalwar/3095780>

pub use tinyc_grammar::{TokenKind, Token};

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
            kind: TokenKind::Whitespace,
            len: source
                .find(is_not_whitespace)
                .unwrap_or_else(|| source.len()) as u32,
        }
    } else if source.starts_with(is_digit) {
        Token {
            kind: TokenKind::Integer,
            len: source.find(is_not_digit).unwrap_or_else(|| source.len()) as u32,
        }
    } else if source.starts_with(is_ident) {
        let len = source.find(is_not_ident).unwrap_or_else(|| source.len());
        Token {
            kind: TokenKind::from_identifier(&source[..len]),
            len: len as u32,
        }
    } else {
        let ch = source.chars().next().unwrap();
        Token {
            kind: match ch {
                '{' => TokenKind::LeftCurlyBracket,
                '}' => TokenKind::RightCurlyBracket,
                '(' => TokenKind::LeftParenthesis,
                ')' => TokenKind::RightParenthesis,
                '+' => TokenKind::PlusSign,
                '-' => TokenKind::HyphenMinus,
                '<' => TokenKind::LessThanSign,
                ';' => TokenKind::Semicolon,
                '=' => TokenKind::EqualsSign,
                _ => TokenKind::ERROR,
            },
            len: ch.len_utf8() as u32,
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
