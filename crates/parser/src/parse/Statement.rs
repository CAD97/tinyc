use super::*;

/// ```text
/// Statement |= If: { "if" cond:(Expression::Parenthesized) then:Statement else:{ "else" then:Statement } } ;
/// ```
pub(super) fn If(p: &mut Parser) {
    let m = p.start();
    p.bump(TokenKind::If);
    Expression::Parenthesized(p);
    Statement(p);
    if p.eat(TokenKind::Else) {
        Statement(p);
    }
    m.complete(p, SyntaxKind::StatementIf);
}

/// ```text
/// Statement |= While: { "while" cond:(Expression::Parenthesized) then:Statement } ;
/// ```
pub(super) fn While(p: &mut Parser) {
    assert_eq!(p.current(), Some(TokenKind::While));
    let m = p.start();
    p.bump(TokenKind::While);
    Expression::Parenthesized(p);
    Statement(p);
    m.complete(p, SyntaxKind::StatementWhile);
}

/// ```text
/// Statement |= Block: { "{" then:Statement* "}" } ;
/// ```
pub(super) fn Block(p: &mut Parser) {
    let m = p.start();
    p.bump(TokenKind::LeftCurlyBracket);
    while !p.at(TokenKind::RightCurlyBracket) {
        Statement(p);
    }
    p.expect(TokenKind::RightCurlyBracket);
    m.complete(p, SyntaxKind::StatementBlock);
}

/// ```text
/// Statement |= Expression: { then:Expression? ";" } ;
/// ```
///
/// This node emits `error` on unexpected leading tokens.
pub(super) fn Expression(p: &mut Parser) {
    let m = p.start();
    if !p.eat(TokenKind::Semicolon) {
        super::Expression(p);
        p.expect(TokenKind::Semicolon);
    }
    m.complete(p, SyntaxKind::StatementExpression);
}
