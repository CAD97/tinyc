use {
    crate::TokenKind,
    std::{
        iter::FromIterator,
        ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
    },
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct TokenSet(u128);

impl TokenSet {
    pub(crate) const EMPTY: TokenSet = TokenSet(0);

    pub(crate) fn new(kind: TokenKind) -> TokenSet {
        TokenSet(1u128 << (kind as u16))
    }
}

impl BitAnd for TokenSet {
    type Output = TokenSet;

    fn bitand(self, rhs: Self) -> TokenSet {
        TokenSet(self.0 & rhs.0)
    }
}

impl BitAndAssign for TokenSet {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for TokenSet {
    type Output = TokenSet;

    fn bitor(self, rhs: Self) -> TokenSet {
        TokenSet(self.0 | rhs.0)
    }
}

impl BitOrAssign for TokenSet {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitAnd<TokenKind> for TokenSet {
    type Output = bool;

    fn bitand(self, kind: TokenKind) -> bool {
        self & TokenSet::new(kind) != TokenSet::EMPTY
    }
}

impl BitOr<TokenKind> for TokenSet {
    type Output = TokenSet;

    fn bitor(self, kind: TokenKind) -> TokenSet {
        self | TokenSet::new(kind)
    }
}

impl BitOrAssign<TokenKind> for TokenSet {
    fn bitor_assign(&mut self, kind: TokenKind) {
        *self = *self | kind;
    }
}

impl FromIterator<TokenKind> for TokenSet {
    fn from_iter<I: IntoIterator<Item = TokenKind>>(iter: I) -> Self {
        let mut set = TokenSet::EMPTY;
        for kind in iter.into_iter() {
            set |= kind;
        }
        set
    }
}
