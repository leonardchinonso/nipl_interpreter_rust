/// is_letter_or_underscore returns true if ch is an english alphabet or an underscore
pub fn is_letter_or_underscore(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
}

/// is_digit returns true if ch is a number between 0 and 9 inclusive and false otherwise
pub fn is_digit(ch: char) -> bool {
    return '0' <= ch && ch <= '9';
}
