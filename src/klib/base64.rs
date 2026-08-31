// SigmaOS Self-Hosted Base64 Codec
// Zero-dependency base64 encoding and decoding
// Reduces reliance on external base64 crates

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn char_to_val(c: u8) -> Option<u8> {
    match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

/// Encode raw bytes into a base64 string
pub fn encode(input: &[u8]) -> String {
    let mut result = String::new();
    let mut chunks = input.chunks(3);

    for chunk in chunks {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };

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

/// Decode a base64 string into raw bytes
pub fn decode(input: &str) -> Result<Vec<u8>, &'static str> {
    let bytes = input.bytes().collect::<Vec<u8>>();
    let mut result = Vec::new();

    if bytes.len() % 4 != 0 {
        return Err("Base64 input length must be a multiple of 4");
    }

    for chunk in bytes.chunks(4) {
        if chunk.is_empty() {
            break;
        }

        let a = char_to_val(chunk[0]).ok_or("Invalid base64 character")?;
        let b = if chunk.len() > 1 {
            char_to_val(chunk[1]).ok_or("Invalid base64 character")?
        } else {
            return Err("Truncated base64 input");
        };
        let c = if chunk.len() > 2 && chunk[2] != b'=' {
            char_to_val(chunk[2]).ok_or("Invalid base64 character")?
        } else {
            0
        };
        let d = if chunk.len() > 3 && chunk[3] != b'=' {
            char_to_val(chunk[3]).ok_or("Invalid base64 character")?
        } else {
            0
        };

        result.push((a << 2) | (b >> 4));

        if chunk.len() > 2 && chunk[2] != b'=' {
            result.push(((b & 0x0F) << 4) | (c >> 2));
        }

        if chunk.len() > 3 && chunk[3] != b'=' {
            result.push(((c & 0x03) << 6) | d);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode_hello() {
        assert_eq!(encode(b"Hello"), "SGVsbG8=");
    }

    #[test]
    fn test_base64_encode_empty() {
        assert_eq!(encode(b""), "");
    }

    #[test]
    fn test_base64_encode_short_strings() {
        assert_eq!(encode(b"f"), "Zg==");
        assert_eq!(encode(b"fo"), "Zm8=");
        assert_eq!(encode(b"foo"), "Zm9v");
    }

    #[test]
    fn test_base64_roundtrip() {
        let inputs: &[&[u8]] = &[
            b"",
            b"f",
            b"fo",
            b"foo",
            b"foob",
            b"fooba",
            b"foobar",
            b"Hello, World!",
            b"SigmaOS Sovereign OS",
            &[0x00, 0xFF, 0x80, 0x7F],
        ];
        for &input in inputs {
            let encoded = encode(input);
            let decoded = decode(&encoded).expect("decode failed");
            assert_eq!(decoded.as_slice(), input);
        }
    }

    #[test]
    fn test_base64_decode_invalid() {
        assert!(decode("abc!@#").is_err());
    }

    #[test]
    fn test_base64_decode_truncated() {
        assert!(decode("SGV").is_err());
    }

    #[test]
    fn test_base64_binary_data() {
        let data = [0x00u8, 0xFF, 0x80, 0x7F];
        let encoded = encode(&data);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded.as_slice(), data);
    }
}
