use std::collections::HashSet;

use crate::core::{Card, CardBitSet, CardIter, Suit, Value};

fn combinations_count(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result = 1usize;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

fn sample_cards(n: usize) -> Vec<Card> {
    CardBitSet::default().into_iter().take(n).collect()
}

fn bitset_from_cards(cards: &[Card]) -> CardBitSet {
    cards.iter().copied().collect()
}

fn sparse_cards(n: usize) -> Vec<Card> {
    let mut used_values: Vec<Value> = Vec::new();
    let mut used_suits: Vec<Suit> = Vec::new();
    let mut selected = Vec::new();

    for card in CardBitSet::default() {
        if !used_values.contains(&card.value) && !used_suits.contains(&card.suit) {
            used_values.push(card.value);
            used_suits.push(card.suit);
            selected.push(card);
            if selected.len() == n {
                break;
            }
        }
    }

    selected
}

#[test]
fn test_empty_input_with_k_zero() {
    let input = CardBitSet::new();
    let results: Vec<CardBitSet> = CardIter::new(input, 0).collect();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0], CardBitSet::new());
    assert_eq!(results[0].count(), 0);
}

#[test]
fn test_empty_input_with_k_greater_than_zero() {
    let input = CardBitSet::new();

    for k in [1, 2, 5] {
        let results: Vec<CardBitSet> = CardIter::new(input, k).collect();
        assert!(results.is_empty());
    }
}

#[test]
fn test_non_empty_input_with_k_zero() {
    let cards = sample_cards(4);
    let input = bitset_from_cards(&cards);
    let results: Vec<CardBitSet> = CardIter::new(input, 0).collect();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0], CardBitSet::new());
    assert_eq!(results[0].count(), 0);
}

#[test]
fn test_k_equals_one() {
    let cards = sample_cards(4);
    let input = bitset_from_cards(&cards);
    let results: Vec<CardBitSet> = CardIter::new(input, 1).collect();

    assert_eq!(results.len(), cards.len());

    let unique: HashSet<CardBitSet> = results.iter().copied().collect();
    assert_eq!(unique.len(), cards.len());

    for card in cards {
        let mut expected_single = CardBitSet::new();
        expected_single.insert(card);
        assert!(unique.contains(&expected_single));
    }
}

#[test]
fn test_k_equals_n() {
    let cards = sample_cards(4);
    let input = bitset_from_cards(&cards);
    let results: Vec<CardBitSet> = CardIter::new(input, cards.len()).collect();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0], input);
}

#[test]
fn test_k_greater_than_n() {
    let cards = sample_cards(2);
    let input = bitset_from_cards(&cards);

    for k in [3, 4, 10] {
        let results: Vec<CardBitSet> = CardIter::new(input, k).collect();
        assert!(results.is_empty());
    }
}

#[test]
fn test_three_card_input_with_k_two() {
    let cards = sample_cards(3);
    let input = bitset_from_cards(&cards);
    let results: Vec<CardBitSet> = CardIter::new(input, 2).collect();

    assert_eq!(results.len(), 3);

    let yielded_set: HashSet<CardBitSet> = results.into_iter().collect();
    assert_eq!(yielded_set.len(), 3);

    let expected_pairs = [
        [cards[0], cards[1]],
        [cards[0], cards[2]],
        [cards[1], cards[2]],
    ];
    let expected_set: HashSet<CardBitSet> = expected_pairs
        .iter()
        .map(|pair| bitset_from_cards(pair))
        .collect();

    assert_eq!(yielded_set, expected_set);
}

#[test]
fn test_representative_combination_counts_and_invariants() {
    let cases = [
        (0, 0),
        (1, 0),
        (1, 1),
        (2, 1),
        (3, 2),
        (4, 2),
        (5, 0),
        (5, 1),
        (5, 2),
        (5, 3),
        (5, 4),
        (5, 5),
        (6, 3),
        (7, 4),
        (10, 3),
    ];

    for (n, k) in cases {
        let cards = sample_cards(n);
        let input = bitset_from_cards(&cards);
        let expected_len = combinations_count(n, k);

        let results: Vec<CardBitSet> = CardIter::new(input, k).collect();
        assert_eq!(results.len(), expected_len, "failed for C({n}, {k})");

        let mut unique = HashSet::new();
        for combo in results {
            assert_eq!(combo.count(), k);
            for card in combo {
                assert!(input.contains(card));
            }
            assert!(unique.insert(combo));
        }

        assert_eq!(unique.len(), expected_len);
    }
}

#[test]
fn test_sparse_non_adjacent_cards() {
    let cards = sparse_cards(4);
    assert_eq!(cards.len(), 4);

    let input = bitset_from_cards(&cards);
    let k = 2;
    let results: Vec<CardBitSet> = CardIter::new(input, k).collect();

    let expected_len = combinations_count(cards.len(), k);
    assert_eq!(results.len(), expected_len);

    let mut expected_set = HashSet::new();
    for i in 0..cards.len() {
        for j in (i + 1)..cards.len() {
            expected_set.insert(bitset_from_cards(&[cards[i], cards[j]]));
        }
    }

    let yielded_set: HashSet<CardBitSet> = results.into_iter().collect();
    assert_eq!(yielded_set, expected_set);
}

#[test]
fn test_modifying_yielded_set_does_not_affect_others_or_input() {
    let all_cards = sample_cards(5);
    let input_cards = &all_cards[..4];
    let extra_card = all_cards[4];

    let input = bitset_from_cards(input_cards);
    let original_input = input;

    let mut iter = CardIter::new(input, 2);
    let mut first = iter.next().expect("expected first combination");
    let first_snapshot = first;

    first.clear();
    first.insert(extra_card);

    let mut all_valid = HashSet::new();
    all_valid.insert(first_snapshot);

    for combo in iter {
        assert_eq!(combo.count(), 2);
        assert!(!combo.contains(extra_card));
        for card in combo {
            assert!(original_input.contains(card));
        }
        assert!(all_valid.insert(combo));
    }

    assert_eq!(input, original_input);
    assert_eq!(all_valid.len(), combinations_count(input_cards.len(), 2));
}
