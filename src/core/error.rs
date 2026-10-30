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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_and_traits() {
        let err = CardParseError::InvalidLength;
        assert_eq!(err.to_string(), "invalid card notation length");
        assert_eq!(err, CardParseError::InvalidLength);
        assert_ne!(err, CardParseError::InvalidValue);
        let std_err: &dyn std::error::Error = &err;
        assert!(std_err.source().is_none());
    }
}
