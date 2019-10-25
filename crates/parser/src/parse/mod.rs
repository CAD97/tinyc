//! This is the actual parser for TinyC.
//!
//! Each function in this module and its children
//! corresponds to a production of the grammar.
//! By convention, each submodule imports with
//! `use super::*` and exports via `pub(super)`.
//!
//! As a reminder, the grammar (in rough lyg format) is:
//!
//! ```text
//! Program = Statement*;
//! Statement =
//!   | If:{ "if" cond:(Expression::Parenthesized) then:Statement else:{ "else" then:Statement } }
//!   | While:{ "while" cond:(Expression::Parenthesized) then:Statement }
//!   | Block:{ "{" then:Statement* "}" }
//!   | Expression:{ then:Expression? ";" }
//!   ;
//! Expression =
//!   | Parenthesized:{ "(" Expression ")" }
//!   | Assignment:{ id:Identifier "=" val:Expression }
//!   | Comparison: #[associativity(left)] { lhs:Expression "<" rhs:Expression }
//!   | Addition: #[associativity(left)] { lhs:Expression "+" rhs:Expression }
//!   | Subtraction: #[associativity(left)] { lhs:Expression "-" rhs:Expression }
//!   | Term:Term
//!   ;
//! Term =
//!   | Identifier:Identifier // token
//!   | Integer:Integer // token
//!   | Expression:(Expression::Parenthesized)
//!   ;
//! ```
//!
//! See the docs for `Parser` to learn about the API available to parse with,
//! and for `Event` to learn how this actually produces parse trees.
//!
//! Unless other wise noted, nodes assume their first token is present;
//! the caller is responsible for branching on token lookahead.

#![allow(non_snake_case)]

// TODO: Inline tests

use crate::{parser::CompletedMarker, Parser, SyntaxKind, TokenKind};

/// ```text
/// Program = Statement*;
/// ```
///
/// This node consumes the entire input.
pub(super) fn Program(p: &mut Parser) {
    let m = p.start();
    while p.current().is_some() {
        Statement(p);
    }
    m.complete(p, SyntaxKind::Program);
}

/// ```text
/// Statement =
///   | If:{ "if" cond:(Expression::Parenthesized) then:Statement else:{ "else" then:Statement } }
///   | While:{ "while" cond:(Expression::Parenthesized) then:Statement }
///   | Block:{ "{" then:Statement* "}" }
///   | Expression:{ then:Expression? ";" }
///   ;
/// ```
///
/// This node emits `error` on unexpected leading tokens.
fn Statement(p: &mut Parser) {
    match p.current().unwrap() {
        TokenKind::If => Statement::If(p),
        TokenKind::While => Statement::While(p),
        TokenKind::LeftCurlyBracket => Statement::Block(p),
        _ => Statement::Expression(p),
    }
}
mod Statement;

/// ```text
/// Expression =
///   | Parenthesized:{ "(" Expression ")" }
///   | Assignment:{ id:Identifier "=" val:Expression }
///   | Comparison: #[associativity(left)] { lhs:Expression "<" rhs:Expression }
///   | Addition: #[associativity(left)] { lhs:Expression "+" rhs:Expression }
///   | Subtraction: #[associativity(left)] { lhs:Expression "-" rhs:Expression }
///   | Term:Term
///   ;
/// ```
///
/// This node emits `error` on unexpected leading tokens.
fn Expression(p: &mut Parser) {
    fn Expression_(p: &mut Parser) -> CompletedMarker {
        if p.at(TokenKind::LeftParenthesis) {
            Expression::Parenthesized(p)
        } else if p.at(TokenKind::Identifier) && p.la_at(1, TokenKind::EqualsSign) {
            Expression::Assignment(p)
        } else {
            Expression::Term(p)
        }
    }
    let mut expr = Expression_(p);
    loop {
        if p.at(TokenKind::LessThanSign) {
            let m = expr.precede(p);
            p.bump(TokenKind::LessThanSign);
            Expression_(p);
            expr = m.complete(p, SyntaxKind::ExpressionComparison);
        } else if p.at(TokenKind::PlusSign) {
            let m = expr.precede(p);
            p.bump(TokenKind::PlusSign);
            Expression_(p);
            expr = m.complete(p, SyntaxKind::ExpressionAddition);
        } else if p.at(TokenKind::HyphenMinus) {
            let m = expr.precede(p);
            p.bump(TokenKind::HyphenMinus);
            Expression_(p);
            expr = m.complete(p, SyntaxKind::ExpressionSubtraction);
        } else {
            break;
        }
    }
}
mod Expression;

/// ```text
/// Term =
///   | Identifier:(Token::Identifier)
///   | Integer:(Token::Integer)
///   | Expression:(Expression::Parenthesized)
///   ;
/// ```
///
/// This node emits `error` on unexpected leading tokens.
fn Term(p: &mut Parser) {
    if p.at(TokenKind::Identifier) {
        Term::Identifier(p)
    } else if p.at(TokenKind::Integer) {
        Term::Integer(p)
    } else if p.at(TokenKind::LeftParenthesis) {
        Term::Expression(p)
    } else {
        p.err_bump("expected `Integer`, `Identifier`, or `LeftParenthesis`")
    }
}
mod Term;
