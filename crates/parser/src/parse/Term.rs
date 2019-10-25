use super::*;

/// ```text
/// Term |= Identifier: (Token::Identifier) ;
/// ```
pub(super) fn Identifier(p: &mut Parser) {
    let m = p.start();
    p.bump(TokenKind::Identifier);
    m.complete(p, SyntaxKind::TermIdentifier);
}

/// ```text
/// Term |= Integer: (Token::Integer) ;
/// ```
pub(super) fn Integer(p: &mut Parser) {
    let m = p.start();
    p.bump(TokenKind::Integer);
    m.complete(p, SyntaxKind::TermInteger);
}

/// ```text
/// Term |= Expression: (Expression::Parenthesized) ;
/// ```
pub(super) fn Expression(p: &mut Parser) {
    let m = p.start();
    Expression::Parenthesized(p);
    m.complete(p, SyntaxKind::TermExpression);
}
