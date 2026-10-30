use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardParseError {
    InvalidLength,
    InvalidValue,
    InvalidSuit,
}

impl fmt::Display for CardParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "invalid card notation length"),
            Self::InvalidValue => write!(f, "invalid card value character"),
            Self::InvalidSuit => write!(f, "invalid card suit character"),
        }
    }
}

impl std::error::Error for CardParseError {}
