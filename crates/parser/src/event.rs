use {
    crate::{ParseError, SyntaxKind, TreeSink},
    std::{mem, num::NonZeroU32},
};

/// A parser produces a flat list of `Event`s.
/// These are converted into the tree-structure and
/// passed into a `TreeSink` via `Event::sink`.
///
/// # Example
///
/// Consider the expression `a<b`.
/// The events for it might look something like this:
///
/// ```yaml
/// - Start { kind: TermIdentifier, forward_parent: Some(3) }   # TermIdentifier {
/// - Leaf { kind: Identifier }                                 #   a
/// - Finish                                                    # }
/// - Start { kind: ExpressionCompare, forward_parent: None }   # ExpressionCompare {
/// - Leaf { kind: LessThanSign }                               #   <
/// - Start { kind: TermIdentifier, forward_parent: None }      #   TermIdentifier {
/// - Leaf { kind: Identifier }                                 #     b
/// - Finish                                                    #   }
/// - Finish                                                    # }
/// ```
///
/// to generate a tree like this:
///
/// ```yaml
/// - ExpressionCompare
///   - TermIdentifier
///     - Identifier: "a"
///   - LessThanSign: "<"
///   - TermIdentifier
///     - Identifier: "b"
/// ```
#[derive(Debug, Clone)]
#[derive(serde::Serialize)]
pub(crate) enum Event {
    /// The start of a node.
    /// It should be completed via a `Finish` event.
    ///
    /// All tokens between a `Start` and a `Finish`
    /// become the children of the respective node.
    ///
    /// For left-recursive syntactic constructs,
    /// the parser produces a child node before the parent;
    /// `forward_parent` offsets to the following parent.
    Start {
        kind: SyntaxKind,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        forward_parent: Option<NonZeroU32>,
    },
    /// Complete the previous `Start` event.
    Finish,
    /// A single terminal leaf element.
    Leaf { kind: SyntaxKind },
    /// An error at this position.
    Error { message: ParseError },
    /// An abandoned or already processed event that should be ignored.
    Abandoned,
}

impl Event {
    pub(crate) fn sink(events: &mut [Self], sink: &mut dyn TreeSink) {
        let mut forward_parents = Vec::new();
        for i in 0..events.len() {
            match mem::replace(&mut events[i], Event::Abandoned) {
                Event::Abandoned => (),
                Event::Start {
                    kind,
                    forward_parent,
                } => {
                    forward_parents.push(kind);
                    let mut idx = i;
                    let mut fp = forward_parent;
                    while let Some(fwd) = fp {
                        idx += fwd.get() as usize;
                        fp = match mem::replace(&mut events[idx], Event::Abandoned) {
                            Event::Start { forward_parent, .. } => forward_parent,
                            Event::Abandoned => None,
                            _ => unreachable!(),
                        };
                    }
                    for kind in forward_parents.drain(..).rev() {
                        sink.start(kind);
                    }
                }
                Event::Finish => sink.finish(),
                Event::Leaf { kind } => sink.leaf(kind),
                Event::Error { message } => sink.error(message),
            }
        }
    }
}
