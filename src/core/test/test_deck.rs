use crate::core::{Card, CardBitSet, Deck, Suit, Value};

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

#[test]
fn test_new_empty() {
    let deck = Deck::new();
    assert_eq!(deck.len(), 0);
    assert!(deck.is_empty());
}

#[test]
fn test_default_contains_52_cards() {
    let deck = Deck::default();
    assert_eq!(deck.len(), 52);
    assert!(!deck.is_empty());
}

#[test]
fn test_default_contains_every_possible_card() {
    let deck = Deck::default();
    for suit in SUITS {
        for value in VALUES {
            let card = Card::new(value, suit);
            assert!(deck.contains(&card));
        }
    }
}

#[test]
fn test_membership_checks() {
    let mut deck = Deck::new();
    let card = Card::new(Value::Ace, Suit::Spade);
    assert!(!deck.contains(&card));
    deck.insert(card);
    assert!(deck.contains(&card));
}

#[test]
fn test_insert_absent_card() {
    let mut deck = Deck::new();
    let card = Card::new(Value::King, Suit::Heart);
    assert!(deck.insert(card));
    assert!(deck.contains(&card));
    assert_eq!(deck.len(), 1);
}

#[test]
fn test_insert_already_present_card() {
    let mut deck = Deck::new();
    let card = Card::new(Value::King, Suit::Heart);
    assert!(deck.insert(card));
    assert!(!deck.insert(card));
    assert_eq!(deck.len(), 1);
}

#[test]
fn test_remove_present_card() {
    let mut deck = Deck::default();
    let card = Card::new(Value::Ace, Suit::Spade);
    assert!(deck.contains(&card));
    assert!(deck.remove(&card));
    assert!(!deck.contains(&card));
}

#[test]
fn test_remove_absent_card() {
    let mut deck = Deck::new();
    let card = Card::new(Value::Ace, Suit::Spade);
    assert!(!deck.remove(&card));
    assert_eq!(deck.len(), 0);
}

#[test]
fn test_length_changes_after_insertion_and_removal() {
    let mut deck = Deck::new();
    let card1 = Card::new(Value::Ace, Suit::Spade);
    let card2 = Card::new(Value::King, Suit::Heart);

    assert_eq!(deck.len(), 0);

    deck.insert(card1);
    assert_eq!(deck.len(), 1);

    deck.insert(card2);
    assert_eq!(deck.len(), 2);

    deck.insert(card1);
    assert_eq!(deck.len(), 2);

    deck.remove(&card1);
    assert_eq!(deck.len(), 1);

    deck.remove(&card1);
    assert_eq!(deck.len(), 1);

    deck.remove(&card2);
    assert_eq!(deck.len(), 0);
}

#[test]
#[allow(clippy::len_zero)]
fn test_is_empty_consistency() {
    let mut deck = Deck::new();
    assert!(deck.is_empty());
    assert_eq!(deck.is_empty(), deck.len() == 0);

    let card = Card::new(Value::Ten, Suit::Club);
    deck.insert(card);
    assert!(!deck.is_empty());
    assert_eq!(deck.is_empty(), deck.len() == 0);

    deck.remove(&card);
    assert!(deck.is_empty());
    assert_eq!(deck.is_empty(), deck.len() == 0);
}

#[test]
fn test_iter_empty_deck() {
    let deck = Deck::new();
    let cards: Vec<Card> = deck.iter().collect();
    assert!(cards.is_empty());
}

#[test]
fn test_iter_non_empty_deck_and_removed_cards() {
    let mut deck = Deck::default();
    let removed_card = Card::new(Value::Ace, Suit::Spade);
    deck.remove(&removed_card);

    let collected: Vec<Card> = deck.iter().collect();
    assert_eq!(collected.len(), 51);
    assert!(!collected.contains(&removed_card));

    let mut unique_check = Deck::new();
    for card in collected {
        assert!(unique_check.insert(card));
    }
    assert_eq!(unique_check.len(), 51);
}

#[test]
fn test_consuming_iteration() {
    let mut deck = Deck::new();
    let card1 = Card::new(Value::Jack, Suit::Diamond);
    let card2 = Card::new(Value::Queen, Suit::Heart);
    deck.insert(card1);
    deck.insert(card2);

    let mut count = 0;
    let mut found1 = false;
    let mut found2 = false;

    for card in deck {
        count += 1;
        if card == card1 {
            found1 = true;
        }
        if card == card2 {
            found2 = true;
        }
    }

    assert_eq!(count, 2);
    assert!(found1);
    assert!(found2);
}

#[test]
fn test_from_empty_card_bit_set() {
    let empty_bitset = CardBitSet::new();
    let deck: Deck = empty_bitset.into();
    assert!(deck.is_empty());
    assert_eq!(deck.len(), 0);
}

#[test]
fn test_from_non_empty_card_bit_set() {
    let mut bitset = CardBitSet::new();
    let card1 = Card::new(Value::Nine, Suit::Club);
    let card2 = Card::new(Value::Eight, Suit::Spade);
    bitset.insert(card1);
    bitset.insert(card2);

    let deck: Deck = bitset.into();
    assert_eq!(deck.len(), 2);
    assert!(deck.contains(&card1));
    assert!(deck.contains(&card2));
}

#[test]
fn test_deck_equality() {
    let mut deck1 = Deck::new();
    let mut deck2 = Deck::new();

    let card1 = Card::new(Value::Two, Suit::Heart);
    let card2 = Card::new(Value::Seven, Suit::Diamond);

    deck1.insert(card1);
    deck1.insert(card2);

    deck2.insert(card2);
    deck2.insert(card1);

    assert_eq!(deck1, deck2);

    deck2.remove(&card1);
    assert_ne!(deck1, deck2);
}
