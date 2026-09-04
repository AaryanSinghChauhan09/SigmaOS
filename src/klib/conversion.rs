#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// SigmaOS Custom Conversion Library
// Reduces dependency on predefined conversion functions

// (no_std only applicable at crate root - removed)

/// Custom hexadecimal string to bytes conversion
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, ()> {
    let mut bytes = Vec::new();
    let chars: Vec<char> = hex.chars().collect();

    if chars.len() % 2 != 0 {
        return Err(());
    }

    for i in (0..chars.len()).step_by(2) {
        let high = char_to_hex(chars[i])?;
        let low = char_to_hex(chars[i + 1])?;
        bytes.push((high << 4) | low);
    }

    Ok(bytes)
}

/// Custom bytes to hexadecimal string conversion
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut result = String::new();
    for &byte in bytes {
        result.push(char::from_digit((byte >> 4) as u32, 16).unwrap_or('0'));
        result.push(char::from_digit((byte & 0x0F) as u32, 16).unwrap_or('0'));
    }
    result
}

/// Custom base64 encoding
pub fn base64_encode(input: &[u8]) -> String {
    const BASE64_CHARS: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let mut chunks = input.chunks(3);

    for chunk in chunks {
        let mut buffer = [0u8; 3];
        buffer[..chunk.len()].copy_from_slice(chunk);

        let b0 = buffer[0];
        let b1 = if chunk.len() > 1 { buffer[1] } else { 0 };
        let b2 = if chunk.len() > 2 { buffer[2] } else { 0 };

        result.push(BASE64_CHARS[(b0 >> 2) as usize] as char);
        result.push(BASE64_CHARS[((b0 & 0x03) << 4 | (b1 >> 4)) as usize] as char);

        if chunk.len() > 1 {
            result.push(BASE64_CHARS[((b1 & 0x0F) << 2 | (b2 >> 6)) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(BASE64_CHARS[(b2 & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

/// Custom character to hex digit
fn char_to_hex(c: char) -> Result<u8, ()> {
    match c {
        '0'..='9' => Ok(c as u8 - b'0'),
        'a'..='f' => Ok(c as u8 - b'a' + 10),
        'A'..='F' => Ok(c as u8 - b'A' + 10),
        _ => Err(()),
    }
}

/// Custom binary string to bytes conversion
pub fn binary_to_bytes(binary: &str) -> Result<Vec<u8>, ()> {
    let mut bytes = Vec::new();
    let chars: Vec<char> = binary.chars().collect();

    if chars.len() % 8 != 0 {
        return Err(());
    }

    for i in (0..chars.len()).step_by(8) {
        let mut byte = 0u8;
        for j in 0..8 {
            if chars[i + j] == '1' {
                byte |= 1 << (7 - j);
            } else if chars[i + j] != '0' {
                return Err(());
            }
        }
        bytes.push(byte);
    }

    Ok(bytes)
}

/// Custom bytes to binary string conversion
pub fn bytes_to_binary(bytes: &[u8]) -> String {
    let mut result = String::new();
    for &byte in bytes {
        for i in (0..8).rev() {
            result.push(if (byte >> i) & 1 == 1 { '1' } else { '0' });
        }
    }
    result
}

/// Custom decimal to any base conversion
pub fn dec_to_base(mut n: u64, base: u8) -> String {
    if base < 2 || base > 36 {
        return String::new();
    }

    if n == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    while n > 0 {
        let digit = (n % base as u64) as u8;
        result.push(if digit < 10 {
            (b'0' + digit) as char
        } else {
            (b'a' + digit - 10) as char
        });
        n /= base as u64;
    }

    result.chars().rev().collect()
}

/// Zero-allocation, stack-based u64 integer string parser
pub fn parse_u64_str(s: &str) -> Result<u64, ()> {
    if s.is_empty() {
        return Err(());
    }
    let mut val: u64 = 0;
    for &b in s.as_bytes() {
        if !b.is_ascii_digit() {
            return Err(());
        }
        let digit = (b - b'0') as u64;
        val = val.checked_mul(10).ok_or(())?.checked_add(digit).ok_or(())?;
    }
    Ok(val)
}

/// Zero-allocation, stack-based u64 to hex string formatter
pub fn u64_to_hex_str_stack(mut n: u64, buf: &mut [u8; 16]) -> usize {
    if n == 0 {
        buf[0] = b'0';
        return 1;
    }
    const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut tmp = [0u8; 16];
    let mut len = 0;
    while n > 0 {
        tmp[len] = HEX_DIGITS[(n & 0xF) as usize];
        n >>= 4;
        len += 1;
    }
    for i in 0..len {
        buf[i] = tmp[len - 1 - i];
    }
    len
}

/// Custom any base to decimal conversion
pub fn base_to_dec(s: &str, base: u8) -> Result<u64, ()> {
    if base < 2 || base > 36 {
        return Err(());
    }

    let mut result = 0u64;
    for c in s.chars() {
        let digit = if c.is_digit(10) {
            c.to_digit(10).ok_or(())? as u64
        } else if c.is_ascii_lowercase() {
            (c as u8 - b'a' + 10) as u64
        } else if c.is_ascii_uppercase() {
            (c as u8 - b'A' + 10) as u64
        } else {
            return Err(());
        };

        if digit >= base as u64 {
            return Err(());
        }

        result = result * base as u64 + digit;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bytes() {
        let hex = "48656c6c6f";
        let bytes = hex_to_bytes(hex).expect("Failed to convert hex to bytes");
        assert_eq!(bytes, b"Hello");
    }

    #[test]
    fn test_bytes_to_hex() {
        let bytes = b"Hello";
        let hex = bytes_to_hex(bytes);
        assert_eq!(hex, "48656c6c6f");
    }

    #[test]
    fn test_base64_encode() {
        let input = b"Hello";
        let encoded = base64_encode(input);
        assert_eq!(encoded, "SGVsbG8=");
    }

    #[test]
    fn test_binary_to_bytes() {
        let binary = "0100100001100101011011000110110001101111";
        let bytes = binary_to_bytes(binary).expect("Failed to convert binary to bytes");
        assert_eq!(bytes, b"Hello");
    }

    #[test]
    fn test_bytes_to_binary() {
        let bytes = b"Hi";
        let binary = bytes_to_binary(bytes);
        assert_eq!(binary.len(), 16);
    }

    #[test]
    fn test_parse_u64_str() {
        assert_eq!(parse_u64_str("123456").unwrap(), 123456);
        assert!(parse_u64_str("").is_err());
        assert!(parse_u64_str("123a").is_err());
    }

    #[test]
    fn test_u64_to_hex_str_stack() {
        let mut buf = [0u8; 16];
        let len = u64_to_hex_str_stack(255, &mut buf);
        assert_eq!(&buf[..len], b"ff");

        let len_zero = u64_to_hex_str_stack(0, &mut buf);
        assert_eq!(&buf[..len_zero], b"0");
    }

    #[test]
    fn test_dec_to_base() {
        assert_eq!(dec_to_base(255, 16), "ff");
        assert_eq!(dec_to_base(255, 2), "11111111");
        assert_eq!(dec_to_base(255, 10), "255");
    }

    #[test]
    fn test_base_to_dec() {
        assert_eq!(
            base_to_dec("ff", 16).expect("Failed to convert base 16 to decimal"),
            255
        );
        assert_eq!(
            base_to_dec("11111111", 2).expect("Failed to convert base 2 to decimal"),
            255
        );
        assert_eq!(
            base_to_dec("255", 10).expect("Failed to convert base 10 to decimal"),
            255
        );
    }
}
