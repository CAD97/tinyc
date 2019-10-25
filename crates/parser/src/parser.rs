use {
    crate::{Event, ParseError, SyntaxKind, TokenKind, TokenSet, TokenSource},
    drop_bomb::DebugDropBomb,
    std::{cell::Cell, num::NonZeroU32},
};

/// The low-level API for parsing a stream of tokens.
/// The actual parsing is in the `parsing` module.
/// The result of this parser is not a tree, but rather
/// a flat stream of events: see [`Event`] for details.
pub(crate) struct Parser<'tokens> {
    tokens: &'tokens mut dyn TokenSource,
    events: Vec<Event>,
    #[cfg(debug_assertions)]
    steps: Cell<u32>,
}

impl Parser<'_> {
    #[cfg(debug_assertions)]
    fn step(&self) {
        let steps = self.steps.get();
        assert!(steps <= 10_000_000, "the parser seems stuck");
        self.steps.set(steps + 1);
    }

    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn step(&self) {}

    fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }
}

impl<'tokens> Parser<'tokens> {
    pub(crate) fn new(tokens: &'tokens mut dyn TokenSource) -> Self {
        Parser {
            tokens,
            events: vec![],
            steps: Cell::new(0),
        }
    }

    pub(crate) fn finish(self) -> Vec<Event> {
        self.events
    }

    /// The kind of the current token.
    pub(crate) fn current(&self) -> Option<TokenKind> {
        self.la(0)
    }

    /// Lookahead: the kind of the `n`th token.
    pub(crate) fn la(&self, n: usize) -> Option<TokenKind> {
        assert!(n <= 3, "parser should be LL(3)");
        self.step();
        self.tokens.la(n)
    }

    /// Check if the current token is `kind`.
    pub(crate) fn at(&self, kind: TokenKind) -> bool {
        self.la_at(0, kind)
    }

    /// Lookahead: check if the `n`th token is `kind`.
    pub(crate) fn la_at(&self, n: usize, kind: TokenKind) -> bool {
        self.la(n) == Some(kind)
    }

    /// Check if the current token is in `kinds`.
    pub(crate) fn at_any(&self, kinds: TokenSet) -> bool {
        self.current().map_or(false, |kind| kinds & kind)
    }

    /// Consume the next token iff it is `kind`.
    pub(crate) fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.bump_as(kind.into());
            true
        } else {
            false
        }
    }

    /// Start a new node in the syntax tree.
    /// All nodes and tokens consumed between `start`
    /// and the corresponding `Marker::complete`
    /// belong to said new node.
    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        self.push_event(Event::Abandoned);
        Marker::new(pos)
    }

    /// Consume the next token iff `kind` matches.
    pub(crate) fn bump(&mut self, kind: TokenKind) {
        assert!(self.eat(kind));
    }

    /// Advance the parser by one token.
    pub(crate) fn bump_any(&mut self) {
        if let Some(kind) = self.current() {
            self.bump_as(kind.into());
        }
    }

    /// Advance the parser by one token,
    /// treating it as if it had kind `kind`.
    /// This is mostly useful for contextual keywords.
    pub(crate) fn bump_as(&mut self, kind: SyntaxKind) {
        self.tokens.bump();
        self.push_event(Event::Leaf { kind })
    }

    // TODO: structured errors
    /// Emit an error here.
    pub(crate) fn error(&mut self, message: impl Into<String>) {
        let message = ParseError(message.into());
        self.push_event(Event::Error { message })
    }

    /// Consume the next token if it is `kind`,
    /// emitting an error if this is not the case.
    pub(crate) fn expect(&mut self, kind: TokenKind) -> bool {
        if self.eat(kind) {
            true
        } else {
            self.error(format!("expected {:?}", kind));
            false
        }
    }

    /// Create an error node and consume the next token.
    pub(crate) fn err_bump(&mut self, message: impl Into<String>) {
        self.err_recover(message, TokenSet::EMPTY);
    }

    /// Create an error node and consume the next token if it is not in `recovery`.
    pub(crate) fn err_recover(&mut self, message: impl Into<String>, mut recovery: TokenSet) {
        // never eat curly braces during recovery
        recovery |= TokenKind::LeftCurlyBracket;
        recovery |= TokenKind::RightCurlyBracket;

        let m = self.start();
        self.error(message);
        if self.at_any(recovery) {
            m.abandon(self);
        } else {
            self.bump_any();
            m.complete(self, SyntaxKind::ERROR);
        }
    }
}

#[derive(Debug)]
pub(crate) struct Marker {
    pos: u32,
    bomb: DebugDropBomb,
}

impl Marker {
    fn new(pos: u32) -> Marker {
        Marker {
            pos,
            bomb: DebugDropBomb::new("`Marker` must be either `complete`d or `abandon`ed"),
        }
    }

    /// Finish the syntax tre node and assign `kind` to it.
    /// Returns a `CompletedMarker` for possible future operation
    /// like [`CompletedMarker::precede`] for `forward_parent`.
    pub(crate) fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.bomb.defuse();
        let idx = self.pos as usize;
        match &mut p.events[idx] {
            event @ Event::Abandoned => {
                *event = Event::Start {
                    kind,
                    forward_parent: None,
                }
            }
            _ => unreachable!(),
        }
        let finish_pos = p.events.len() as u32;
        p.push_event(Event::Finish);
        CompletedMarker::new(self.pos, finish_pos)
    }

    /// Abandon the syntax tree node.
    /// Its children are attached to its parent instead.
    pub(crate) fn abandon(mut self, p: &mut Parser) {
        self.bomb.defuse();
        // Optimization: Remove `Event::Abandoned` if it's at the end.
        // This makes `p.start().abandon()` not consume extra memory.
        let idx = self.pos as usize;
        if idx == p.events.len() - 1 {
            match p.events.pop() {
                Some(Event::Abandoned) => (),
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct CompletedMarker {
    start_pos: u32,
    finish_pos: u32,
}

impl CompletedMarker {
    fn new(start_pos: u32, finish_pos: u32) -> Self {
        CompletedMarker {
            start_pos,
            finish_pos,
        }
    }

    /// Create a new node that contains this one.
    ///
    /// That is, the parser can parse node `A`, complete it,
    /// then decide that it should be the child of some node `B`.
    /// See also docs about `forward_parent` of [`Event::Start`].
    pub(crate) fn precede(self, p: &mut Parser) -> Marker {
        let parent = p.start();
        let idx = self.start_pos as usize;
        match &mut p.events[idx] {
            Event::Start { forward_parent, .. } => {
                *forward_parent = NonZeroU32::new(parent.pos - self.start_pos);
            }
            _ => unreachable!(),
        }
        parent
    }

    /// Undo the completion and return to a plain `Marker`.
    #[allow(unused)]
    pub(crate) fn undo(self, p: &mut Parser) -> Marker {
        let start_idx = self.start_pos as usize;
        let finish_idx = self.finish_pos as usize;
        match &mut p.events[start_idx] {
            event @ Event::Start {
                forward_parent: None,
                ..
            } => *event = Event::Abandoned,
            _ => unreachable!(),
        }
        // Optimization: Remove `Event::Finish` if it's at the end.
        // This makes `p.start().complete().undo().abandon()` not consume extra memory.
        if finish_idx == p.events.len() - 1 {
            match p.events.pop() {
                Some(Event::Finish) => (),
                _ => unreachable!(),
            }
        } else {
            match &mut p.events[finish_idx] {
                event @ Event::Finish => *event = Event::Abandoned,
                _ => unreachable!(),
            }
        }
        Marker::new(self.start_pos)
    }

//    pub(crate) fn kind(&self, p: &Parser<'_>) -> SyntaxKind {
//        match p.events[self.start_pos as usize] {
//            Event::Start { kind, .. } => kind,
//            _ => unreachable!(),
//        }
//    }
}

#[cfg(test)]
mod tests {
    use crate::{EmptyTokenSource, Parser, SyntaxKind};

    #[test]
    fn pop_optimizations() {
        let mut tokens = EmptyTokenSource;
        let mut p = Parser::new(&mut tokens);
        let p = &mut p;
        assert_eq!(p.events.len(), 0);
        p.start().abandon(p);
        assert_eq!(p.events.len(), 0);
        p.start().complete(p, SyntaxKind::ERROR).undo(p).abandon(p);
        assert_eq!(p.events.len(), 0);
    }
}
