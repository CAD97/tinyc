pub(crate) use crate::{event::Event, parser::Parser, token_set::TokenSet};
pub use tinyc_grammar::{SyntaxKind, TokenKind};

mod event;
mod parse;
mod parser;
mod token_set;

// TODO: structured errors
#[derive(serde::Serialize)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseError(pub String);

/// A source of tokens for the parser.
///
/// `TokenKind::Whitespace` should be handled above this layer.
pub trait TokenSource {
    fn current(&self) -> Option<TokenKind> {
        self.la(0)
    }
    fn la(&self, n: usize) -> Option<TokenKind>;
    fn bump(&mut self);
}

pub trait TreeSink {
    fn leaf(&mut self, kind: SyntaxKind);
    fn start(&mut self, kind: SyntaxKind);
    fn finish(&mut self);
    fn error(&mut self, error: ParseError);
}

pub fn parse(tokens: &mut dyn TokenSource, sink: &mut dyn TreeSink) {
    let p = parse_from_tokens(tokens, parse::Program);
    Event::sink(&mut p.finish(), sink);
}

fn parse_from_tokens(tokens: &mut dyn TokenSource, f: impl FnOnce(&mut Parser)) -> Parser {
    let mut p = Parser::new(tokens);
    f(&mut p);
    p
}

#[cfg(test)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct EmptyTokenSource;

#[cfg(test)]
impl TokenSource for EmptyTokenSource {
    fn current(&self) -> Option<TokenKind> {
        None
    }

    fn la(&self, _n: usize) -> Option<TokenKind> {
        None
    }

    fn bump(&mut self) {}
}

#[cfg(test)]
mod tests {
    use {
        crate::{event::Event, parse, parse_from_tokens, TokenSource},
        tinyc_lexer::{tokenize, TokenKind},
    };

    struct VecTokenSource {
        tokens: Vec<TokenKind>,
        here: usize,
    }

    impl TokenSource for VecTokenSource {
        fn la(&self, n: usize) -> Option<TokenKind> {
            self.tokens.get(self.here + n).copied()
        }

        fn bump(&mut self) {
            self.here += 1;
        }
    }

    #[allow(non_snake_case)]
    #[conformance::tests(exact, serde=yaml, file="tests/KartikTalwar.yaml.test")]
    fn parse_Program_events(s: &str) -> Vec<Event> {
        let mut tokens = VecTokenSource {
            tokens: tokenize(s)
                .filter(|token| token.kind != TokenKind::Whitespace)
                .map(|token| token.kind)
                .collect(),
            here: 0,
        };
        parse_from_tokens(&mut tokens, parse::Program).finish()
    }
}
