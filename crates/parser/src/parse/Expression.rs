use super::*;

/// ```text
/// Expression |= Parenthesized: { "(" Expression ")" } ;
/// ```
pub(super) fn Parenthesized(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(TokenKind::LeftParenthesis);
    Expression(p);
    p.expect(TokenKind::RightParenthesis);
    m.complete(p, SyntaxKind::ExpressionParenthesized)
}
/// ```text
/// Expression |= Assignment: { id:Identifier "=" val:Expression } ;
/// ```
///
/// This node expects two known tokens.
pub(super) fn Assignment(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(TokenKind::Identifier);
    p.bump(TokenKind::EqualsSign);
    Expression(p);
    m.complete(p, SyntaxKind::ExpressionAssignment)
}

/// ```text
/// Expression |= Term: Term ;
/// ```
pub(super) fn Term(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    super::Term(p);
    m.complete(p, SyntaxKind::ExpressionTerm)
}
