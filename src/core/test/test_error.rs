use super::super::CardParseError;

#[test]
fn test_error_display_and_traits() {
    let err = CardParseError::InvalidLength;
    assert_eq!(err.to_string(), "invalid card notation length");
    assert_eq!(err, CardParseError::InvalidLength);
    assert_ne!(err, CardParseError::InvalidValue);
    let std_err: &dyn std::error::Error = &err;
    assert!(std_err.source().is_none());
}
