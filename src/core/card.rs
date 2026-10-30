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
