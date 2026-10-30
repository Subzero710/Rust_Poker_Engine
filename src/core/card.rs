use std::fmt;

use super::error::CardParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
}

impl TryFrom<char> for Suit {
    type Error = CardParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            's' | 'S' => Ok(Self::Spade),
            'c' | 'C' => Ok(Self::Club),
            'h' | 'H' => Ok(Self::Heart),
            'd' | 'D' => Ok(Self::Diamond),
            _ => Err(CardParseError::InvalidSuit),
        }
    }
}

impl From<Suit> for char {
    fn from(suit: Suit) -> Self {
        match suit {
            Suit::Spade => 's',
            Suit::Club => 'c',
            Suit::Heart => 'h',
            Suit::Diamond => 'd',
        }
    }
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

impl TryFrom<char> for Value {
    type Error = CardParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' | 't' => Ok(Self::Ten),
            'J' | 'j' => Ok(Self::Jack),
            'Q' | 'q' => Ok(Self::Queen),
            'K' | 'k' => Ok(Self::King),
            'A' | 'a' => Ok(Self::Ace),
            _ => Err(CardParseError::InvalidValue),
        }
    }
}

impl From<Value> for char {
    fn from(value: Value) -> Self {
        match value {
            Value::Two => '2',
            Value::Three => '3',
            Value::Four => '4',
            Value::Five => '5',
            Value::Six => '6',
            Value::Seven => '7',
            Value::Eight => '8',
            Value::Nine => '9',
            Value::Ten => 'T',
            Value::Jack => 'J',
            Value::Queen => 'Q',
            Value::King => 'K',
            Value::Ace => 'A',
        }
    }
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

impl TryFrom<&str> for Card {
    type Error = CardParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.chars().count() != 2 {
            return Err(CardParseError::InvalidLength);
        }

        let mut chars = s.chars();
        let v_char = chars.next().unwrap();
        let s_char = chars.next().unwrap();

        let value = Value::try_from(v_char)?;
        let suit = Suit::try_from(s_char)?;

        Ok(Self::new(value, suit))
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v: char = self.value.into();
        let s: char = self.suit.into();
        write!(f, "{}{}", v, s)
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

    #[test]
    fn test_value_char_conversions() {
        let mappings = [
            (Value::Two, '2'),
            (Value::Three, '3'),
            (Value::Four, '4'),
            (Value::Five, '5'),
            (Value::Six, '6'),
            (Value::Seven, '7'),
            (Value::Eight, '8'),
            (Value::Nine, '9'),
            (Value::Ten, 'T'),
            (Value::Jack, 'J'),
            (Value::Queen, 'Q'),
            (Value::King, 'K'),
            (Value::Ace, 'A'),
        ];

        for (val, ch) in mappings {
            assert_eq!(char::from(val), ch);
            assert_eq!(Value::try_from(ch), Ok(val));
        }

        // Case insensitivity
        assert_eq!(Value::try_from('t'), Ok(Value::Ten));
        assert_eq!(Value::try_from('j'), Ok(Value::Jack));
        assert_eq!(Value::try_from('q'), Ok(Value::Queen));
        assert_eq!(Value::try_from('k'), Ok(Value::King));
        assert_eq!(Value::try_from('a'), Ok(Value::Ace));

        // Invalid chars
        assert_eq!(Value::try_from('1'), Err(CardParseError::InvalidValue));
        assert_eq!(Value::try_from('x'), Err(CardParseError::InvalidValue));
        assert_eq!(Value::try_from('Z'), Err(CardParseError::InvalidValue));
    }

    #[test]
    fn test_suit_char_conversions() {
        let mappings = [
            (Suit::Spade, 's'),
            (Suit::Club, 'c'),
            (Suit::Heart, 'h'),
            (Suit::Diamond, 'd'),
        ];

        for (suit, ch) in mappings {
            assert_eq!(char::from(suit), ch);
            assert_eq!(Suit::try_from(ch), Ok(suit));
        }

        // Case insensitivity
        assert_eq!(Suit::try_from('S'), Ok(Suit::Spade));
        assert_eq!(Suit::try_from('C'), Ok(Suit::Club));
        assert_eq!(Suit::try_from('H'), Ok(Suit::Heart));
        assert_eq!(Suit::try_from('D'), Ok(Suit::Diamond));

        // Invalid chars
        assert_eq!(Suit::try_from('x'), Err(CardParseError::InvalidSuit));
        assert_eq!(Suit::try_from('1'), Err(CardParseError::InvalidSuit));
    }

    #[test]
    fn test_card_parsing_length_errors() {
        assert_eq!(Card::try_from(""), Err(CardParseError::InvalidLength));
        assert_eq!(Card::try_from("A"), Err(CardParseError::InvalidLength));
        assert_eq!(Card::try_from("Asx"), Err(CardParseError::InvalidLength));
    }

    #[test]
    fn test_card_parsing_value_and_suit_errors() {
        assert_eq!(Card::try_from("Xs"), Err(CardParseError::InvalidValue));
        assert_eq!(Card::try_from("Ax"), Err(CardParseError::InvalidSuit));
    }

    #[test]
    fn test_card_formatting_and_roundtrip() {
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
        let suits = [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond];

        for val in values {
            for suit in suits {
                let card = Card::new(val, suit);
                let formatted = card.to_string();
                assert_eq!(formatted.chars().count(), 2);
                let parsed = Card::try_from(formatted.as_str()).unwrap();
                assert_eq!(card, parsed);
            }
        }

        // Explicit canonical formatting check
        assert_eq!(Card::new(Value::Ace, Suit::Spade).to_string(), "As");
        assert_eq!(Card::new(Value::King, Suit::Heart).to_string(), "Kh");
        assert_eq!(Card::new(Value::Ten, Suit::Club).to_string(), "Tc");
        assert_eq!(Card::new(Value::Two, Suit::Diamond).to_string(), "2d");

        // Case insensitivity during string parsing
        assert_eq!(Card::try_from("as"), Ok(Card::new(Value::Ace, Suit::Spade)));
        assert_eq!(
            Card::try_from("KH"),
            Ok(Card::new(Value::King, Suit::Heart))
        );
        assert_eq!(Card::try_from("tC"), Ok(Card::new(Value::Ten, Suit::Club)));
    }
}
