use super::{Card, CardBitSet, CardBitSetIter};

/// A unique collection of playing cards backed by `CardBitSet`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Deck(CardBitSet);

impl Deck {
    /// Creates a new empty deck.
    #[must_use]
    pub const fn new() -> Self {
        Self(CardBitSet::new())
    }

    /// Reports whether a card is currently present in the deck.
    #[must_use]
    pub fn contains(&self, card: &Card) -> bool {
        self.0.contains(*card)
    }

    /// Adds a card to the deck.
    ///
    /// Returns `true` if the card was not previously present, or `false` if it was already present.
    pub fn insert(&mut self, card: Card) -> bool {
        if self.0.contains(card) {
            false
        } else {
            self.0.insert(card);
            true
        }
    }

    /// Removes a card from the deck.
    ///
    /// Returns `true` if the card was present before removal, or `false` if it was absent.
    pub fn remove(&mut self, card: &Card) -> bool {
        if self.0.contains(*card) {
            self.0.remove(*card);
            true
        } else {
            false
        }
    }

    /// Returns the number of cards currently contained in the deck.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.count()
    }

    /// Reports whether the deck contains no cards.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over the cards currently present in the deck.
    #[must_use]
    pub fn iter(&self) -> CardBitSetIter {
        self.0.into_iter()
    }
}

impl Default for Deck {
    /// Creates a complete standard 52-card deck containing every combination of value and suit once.
    fn default() -> Self {
        Self(CardBitSet::default())
    }
}

impl IntoIterator for Deck {
    type Item = Card;
    type IntoIter = CardBitSetIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<CardBitSet> for Deck {
    fn from(bitset: CardBitSet) -> Self {
        Self(bitset)
    }
}
