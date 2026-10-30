use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use super::{Card, Suit, Value};

const MASK_52: u64 = (1u64 << 52) - 1;

const SUITS: [Suit; 4] = [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond];

const VALUES: [Value; 13] = [
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

fn card_to_bit_index(card: Card) -> u8 {
    let suit_idx = SUITS
        .iter()
        .position(|&s| s == card.suit)
        .expect("invalid suit") as u8;
    let val_idx = VALUES
        .iter()
        .position(|&v| v == card.value)
        .expect("invalid value") as u8;
    suit_idx * 13 + val_idx
}

fn bit_index_to_card(index: u8) -> Card {
    let suit = SUITS[(index / 13) as usize];
    let value = VALUES[(index % 13) as usize];
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
        Self { bits: MASK_52 }
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
            bits: (!self.bits) & MASK_52,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_set() {
        let set = CardBitSet::new();
        assert!(set.is_empty());
        assert_eq!(set.count(), 0);
    }

    #[test]
    fn test_default_full_set() {
        let set = CardBitSet::default();
        assert!(!set.is_empty());
        assert_eq!(set.count(), 52);
    }

    #[test]
    fn test_all_52_cards_representable() {
        let mut set = CardBitSet::new();
        for suit in SUITS {
            for value in VALUES {
                let card = Card::new(value, suit);
                assert!(!set.contains(card));
                set.insert(card);
                assert!(set.contains(card));
            }
        }
        assert_eq!(set.count(), 52);
    }

    #[test]
    fn test_insertion_and_duplicates() {
        let mut set = CardBitSet::new();
        let card = Card::new(Value::Ace, Suit::Spade);

        set.insert(card);
        assert_eq!(set.count(), 1);
        assert!(set.contains(card));

        set.insert(card);
        assert_eq!(set.count(), 1);
    }

    #[test]
    fn test_removal_and_absent_card() {
        let mut set = CardBitSet::new();
        let card1 = Card::new(Value::Ace, Suit::Spade);
        let card2 = Card::new(Value::King, Suit::Heart);

        set.insert(card1);
        set.remove(card2);
        assert_eq!(set.count(), 1);
        assert!(set.contains(card1));

        set.remove(card1);
        assert_eq!(set.count(), 0);
        assert!(!set.contains(card1));
    }

    #[test]
    fn test_clear() {
        let mut set = CardBitSet::default();
        assert_eq!(set.count(), 52);

        set.clear();
        assert!(set.is_empty());
        assert_eq!(set.count(), 0);
    }

    #[test]
    fn test_iter_empty() {
        let set = CardBitSet::new();
        let mut iter = set.into_iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_single_and_multi_card() {
        let card1 = Card::new(Value::Two, Suit::Club);
        let card2 = Card::new(Value::Ace, Suit::Spade);

        let mut set = CardBitSet::new();
        set.insert(card1);
        set.insert(card2);

        let collected: Vec<Card> = set.into_iter().collect();
        assert_eq!(collected.len(), 2);
        assert!(collected.contains(&card1));
        assert!(collected.contains(&card2));
    }

    #[test]
    fn test_iter_unique_cards() {
        let full_set = CardBitSet::default();
        let cards: Vec<Card> = full_set.into_iter().collect();
        assert_eq!(cards.len(), 52);

        let mut set_check = CardBitSet::new();
        for card in cards {
            assert!(!set_check.contains(card));
            set_check.insert(card);
        }
        assert_eq!(set_check.count(), 52);
    }

    #[test]
    fn test_from_iterator() {
        let cards = vec![
            Card::new(Value::Ace, Suit::Spade),
            Card::new(Value::Ace, Suit::Spade),
            Card::new(Value::King, Suit::Heart),
        ];

        let set: CardBitSet = cards.into_iter().collect();
        assert_eq!(set.count(), 2);
        assert!(set.contains(Card::new(Value::Ace, Suit::Spade)));
        assert!(set.contains(Card::new(Value::King, Suit::Heart)));
    }

    #[test]
    fn test_set_operations() {
        let card1 = Card::new(Value::Ace, Suit::Spade);
        let card2 = Card::new(Value::King, Suit::Heart);
        let card3 = Card::new(Value::Queen, Suit::Diamond);

        let mut set_a = CardBitSet::new();
        set_a.insert(card1);
        set_a.insert(card2);

        let mut set_b = CardBitSet::new();
        set_b.insert(card2);
        set_b.insert(card3);

        // Union
        let union_set = set_a | set_b;
        assert_eq!(union_set.count(), 3);
        assert!(union_set.contains(card1));
        assert!(union_set.contains(card2));
        assert!(union_set.contains(card3));

        let mut union_assign = set_a;
        union_assign |= set_b;
        assert_eq!(union_assign, union_set);

        // Intersection
        let inter_set = set_a & set_b;
        assert_eq!(inter_set.count(), 1);
        assert!(inter_set.contains(card2));

        let mut inter_assign = set_a;
        inter_assign &= set_b;
        assert_eq!(inter_assign, inter_set);

        // Symmetric Difference
        let xor_set = set_a ^ set_b;
        assert_eq!(xor_set.count(), 2);
        assert!(xor_set.contains(card1));
        assert!(xor_set.contains(card3));
        assert!(!xor_set.contains(card2));

        let mut xor_assign = set_a;
        xor_assign ^= set_b;
        assert_eq!(xor_assign, xor_set);
    }

    #[test]
    fn test_complement() {
        let empty = CardBitSet::new();
        let full = CardBitSet::default();

        assert_eq!(!empty, full);
        assert_eq!(!full, empty);

        let mut partial = CardBitSet::new();
        let card = Card::new(Value::Ace, Suit::Spade);
        partial.insert(card);

        let comp = !partial;
        assert_eq!(comp.count(), 51);
        assert!(!comp.contains(card));
        assert_eq!(comp.bits & !MASK_52, 0);
    }
}
