// SigmaOS Self-Hosted UTF-8 Validation & Whitespace Utilities
// Zero-dependency string inspection and tokenization
// Replaces reliance on std string methods in kernel contexts

use std::string::String;
use std::vec::Vec;

/// Validate that a byte slice is well-formed UTF-8
pub fn is_valid_utf8(data: &[u8]) -> bool {
    let mut i = 0;
    while i < data.len() {
        let byte = data[i];
        if byte < 0x80 {
            i += 1;
        } else if byte < 0xE0 {
            // 2-byte sequence. 0xC0/0xC1 are invalid (overlong: would encode a
            // code point < 0x80) and must be rejected.
            if byte < 0xC2 {
                return false;
            }
            if i + 1 >= data.len() {
                return false;
            }
            if data[i + 1] & 0xC0 != 0x80 {
                return false;
            }
            i += 2;
        } else if byte < 0xF0 {
            if i + 2 >= data.len() {
                return false;
            }
            if data[i + 1] & 0xC0 != 0x80 {
                return false;
            }
            if data[i + 2] & 0xC0 != 0x80 {
                return false;
            }
            i += 3;
        } else if byte < 0xF5 {
            if i + 3 >= data.len() {
                return false;
            }
            if data[i + 1] & 0xC0 != 0x80 {
                return false;
            }
            if data[i + 2] & 0xC0 != 0x80 {
                return false;
            }
            if data[i + 3] & 0xC0 != 0x80 {
                return false;
            }
            i += 4;
        } else {
            return false;
        }
    }
    true
}

/// Trim ASCII whitespace (space, tab, newline, carriage return) from both ends
pub fn trim_ascii_whitespace(data: &[u8]) -> &[u8] {
    let start = data.iter().take_while(|b| is_ascii_whitespace(**b)).count();
    let end = data
        .iter()
        .rev()
        .take_while(|b| is_ascii_whitespace(**b))
        .count();
    if start + end >= data.len() {
        &[]
    } else {
        &data[start..data.len() - end]
    }
}

/// Tokenize a byte slice on ASCII whitespace, returning owned String tokens
pub fn tokenize_whitespace(data: &[u8]) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = Vec::new();

    for &byte in data {
        if is_ascii_whitespace(byte) {
            if !current.is_empty() {
                tokens.push(String::from_utf8_lossy(&current).into_owned());
                current.clear();
            }
        } else {
            current.push(byte);
        }
    }

    if !current.is_empty() {
        tokens.push(String::from_utf8_lossy(&current).into_owned());
    }

    tokens
}

/// Tokenize returning byte-slice references without allocation
pub fn tokenize_whitespace_ref(data: &[u8]) -> Vec<&[u8]> {
    let mut tokens = Vec::new();
    let mut start = 0;
    let mut i = 0;

    while i < data.len() {
        if is_ascii_whitespace(data[i]) {
            if i > start {
                tokens.push(&data[start..i]);
            }
            start = i + 1;
        }
        i += 1;
    }

    if start < data.len() {
        tokens.push(&data[start..]);
    }

    tokens
}

/// Count ASCII whitespace characters at the start of a slice
pub fn count_leading_whitespace(data: &[u8]) -> usize {
    data.iter().take_while(|b| is_ascii_whitespace(**b)).count()
}

/// Count ASCII whitespace characters at the end of a slice
pub fn count_trailing_whitespace(data: &[u8]) -> usize {
    data.iter()
        .rev()
        .take_while(|b| is_ascii_whitespace(**b))
        .count()
}

const fn is_ascii_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b'\x0B' | b'\x0C')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_utf8_ascii() {
        assert!(is_valid_utf8(b"Hello"));
        assert!(is_valid_utf8(b""));
        assert!(is_valid_utf8(b"123456"));
    }

    #[test]
    fn test_is_valid_utf8_multibyte() {
        assert!(is_valid_utf8("こんにちは".as_bytes()));
        assert!(is_valid_utf8("🎉".as_bytes()));
        assert!(is_valid_utf8("café".as_bytes()));
    }

    #[test]
    fn test_is_valid_utf8_invalid() {
        assert!(!is_valid_utf8(&[0xFF]));
        assert!(!is_valid_utf8(&[0xC0, 0x80])); // overlong
        assert!(!is_valid_utf8(&[0xC2])); // truncated
        assert!(!is_valid_utf8(&[0xE0])); // truncated 3-byte
        assert!(!is_valid_utf8(&[0xF0, 0x90, 0x80])); // truncated 4-byte
    }

    #[test]
    fn test_trim_ascii_whitespace() {
        assert_eq!(trim_ascii_whitespace(b"  hello  "), b"hello");
        assert_eq!(trim_ascii_whitespace(b"hello"), b"hello");
        assert_eq!(trim_ascii_whitespace(b"  "), b"");
        assert_eq!(trim_ascii_whitespace(b"\t\nhello\r\n"), b"hello");
    }

    #[test]
    fn test_tokenize_whitespace() {
        let tokens = tokenize_whitespace(b"hello world foo");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], "hello");
        assert_eq!(tokens[1], "world");
        assert_eq!(tokens[2], "foo");
    }

    #[test]
    fn test_tokenize_whitespace_ref() {
        let tokens = tokenize_whitespace_ref(b"hello world foo");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], b"hello");
        assert_eq!(tokens[1], b"world");
        assert_eq!(tokens[2], b"foo");
    }

    #[test]
    fn test_tokenize_whitespace_empty() {
        let tokens = tokenize_whitespace(b"   ");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_count_leading_trailing_whitespace() {
        assert_eq!(count_leading_whitespace(b"  hello"), 2);
        assert_eq!(count_trailing_whitespace(b"hello  "), 2);
        assert_eq!(count_leading_whitespace(b"hello"), 0);
        assert_eq!(count_trailing_whitespace(b"hello"), 0);
    }
}
