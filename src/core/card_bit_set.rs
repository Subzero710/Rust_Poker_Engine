use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use super::{Card, Suit, Value};

pub(crate) const SUITS: [Suit; 4] = [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond];

pub(crate) const VALUES: [Value; 13] = [
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
    Value::Ace,
];

pub(crate) const NUM_SUITS: usize = SUITS.len();
pub(crate) const NUM_VALUES: usize = VALUES.len();
pub(crate) const TOTAL_CARDS: usize = NUM_SUITS * NUM_VALUES;
pub(crate) const CARD_MASK: u64 = (1u64 << TOTAL_CARDS) - 1;

fn card_to_bit_index(card: Card) -> u8 {
    let suit_idx = SUITS
        .iter()
        .position(|&s| s == card.suit)
        .expect("invalid suit") as u8;
    let val_idx = VALUES
        .iter()
        .position(|&v| v == card.value)
        .expect("invalid value") as u8;
    suit_idx * (NUM_VALUES as u8) + val_idx
}

fn bit_index_to_card(index: u8) -> Card {
    let suit = SUITS[(index as usize) / NUM_VALUES];
    let value = VALUES[(index as usize) % NUM_VALUES];
    Card::new(value, suit)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardBitSet {
    bits: u64,
}

impl CardBitSet {
    #[must_use]
    pub const fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn insert(&mut self, card: Card) {
        let bit = card_to_bit_index(card);
        self.bits |= 1u64 << bit;
    }

    pub fn remove(&mut self, card: Card) {
        let bit = card_to_bit_index(card);
        self.bits &= !(1u64 << bit);
    }

    #[must_use]
    pub fn contains(&self, card: Card) -> bool {
        let bit = card_to_bit_index(card);
        (self.bits & (1u64 << bit)) != 0
    }

    #[must_use]
    pub fn count(&self) -> usize {
        self.bits.count_ones() as usize
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

impl Default for CardBitSet {
    fn default() -> Self {
        Self { bits: CARD_MASK }
    }
}

#[derive(Debug, Clone)]
pub struct CardBitSetIter {
    bits: u64,
}

impl Iterator for CardBitSetIter {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            None
        } else {
            let bit_idx = self.bits.trailing_zeros() as u8;
            self.bits &= self.bits - 1;
            Some(bit_index_to_card(bit_idx))
        }
    }
}

impl IntoIterator for CardBitSet {
    type Item = Card;
    type IntoIter = CardBitSetIter;

    fn into_iter(self) -> Self::IntoIter {
        CardBitSetIter { bits: self.bits }
    }
}

impl FromIterator<Card> for CardBitSet {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        let mut set = Self::new();
        for card in iter {
            set.insert(card);
        }
        set
    }
}

impl BitOr for CardBitSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl BitOrAssign for CardBitSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl BitAnd for CardBitSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
        }
    }
}

impl BitAndAssign for CardBitSet {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
    }
}

impl BitXor for CardBitSet {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits,
        }
    }
}

impl BitXorAssign for CardBitSet {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits;
    }
}

impl Not for CardBitSet {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            bits: (!self.bits) & CARD_MASK,
        }
    }
}
