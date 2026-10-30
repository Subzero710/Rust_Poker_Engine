use crate::core::card_bit_set::{SUITS, TOTAL_CARDS, VALUES};
use crate::core::{Card, CardBitSet, Suit, Value};

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
    assert_eq!(set.count(), TOTAL_CARDS);
}

#[test]
fn test_all_cards_representable() {
    let mut set = CardBitSet::new();
    for suit in SUITS {
        for value in VALUES {
            let card = Card::new(value, suit);
            assert!(!set.contains(card));
            set.insert(card);
            assert!(set.contains(card));
        }
    }
    assert_eq!(set.count(), TOTAL_CARDS);
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
    assert_eq!(set.count(), TOTAL_CARDS);

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
    assert_eq!(cards.len(), TOTAL_CARDS);

    let mut set_check = CardBitSet::new();
    for card in cards {
        assert!(!set_check.contains(card));
        set_check.insert(card);
    }
    assert_eq!(set_check.count(), TOTAL_CARDS);
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

    let union_set = set_a | set_b;
    assert_eq!(union_set.count(), 3);
    assert!(union_set.contains(card1));
    assert!(union_set.contains(card2));
    assert!(union_set.contains(card3));

    let mut union_assign = set_a;
    union_assign |= set_b;
    assert_eq!(union_assign, union_set);

    let inter_set = set_a & set_b;
    assert_eq!(inter_set.count(), 1);
    assert!(inter_set.contains(card2));

    let mut inter_assign = set_a;
    inter_assign &= set_b;
    assert_eq!(inter_assign, inter_set);

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
    assert_eq!(comp.count(), TOTAL_CARDS - 1);
    assert!(!comp.contains(card));
}
