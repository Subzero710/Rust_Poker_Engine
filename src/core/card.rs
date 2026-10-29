#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Self { value, suit }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_suit_variants() {
        let suits = [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond];
        assert_eq!(suits.len(), 4);
        assert_eq!(Suit::Spade, Suit::Spade);
        assert_ne!(Suit::Spade, Suit::Heart);
    }

    #[test]
    fn test_value_variants() {
        let values = [
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
        assert_eq!(values.len(), 13);
        assert_eq!(Value::Ace, Value::Ace);
        assert_ne!(Value::Two, Value::Ace);
    }

    #[test]
    fn test_card_construction_and_equality() {
        let card1 = Card::new(Value::Ace, Suit::Spade);
        let card2 = Card::new(Value::Ace, Suit::Spade);
        let card3 = Card::new(Value::King, Suit::Heart);

        assert_eq!(card1, card2);
        assert_ne!(card1, card3);
        assert_eq!(card1.value, Value::Ace);
        assert_eq!(card1.suit, Suit::Spade);
    }

    #[test]
    fn test_hashing_support() {
        let mut suit_set = HashSet::new();
        suit_set.insert(Suit::Spade);
        assert!(suit_set.contains(&Suit::Spade));

        let mut value_set = HashSet::new();
        value_set.insert(Value::Ace);
        assert!(value_set.contains(&Value::Ace));

        let mut card_set = HashSet::new();
        let card = Card::new(Value::Ace, Suit::Spade);
        card_set.insert(card);
        assert!(card_set.contains(&card));
    }
}
