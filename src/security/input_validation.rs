#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Input Validation - Sovereign Security Module
// Prevents injection attacks, buffer overflows, path traversal, and integer
// overflows without depending on any predefined library (no std, no libc).
// All validation is performed with raw byte slice operations.

/// Maximum lengths for various input categories (mirrors Linux kernel limits).
pub const MAX_PATH_LEN: usize = 4096;
pub const MAX_FILENAME_LEN: usize = 255;
pub const MAX_USERNAME_LEN: usize = 32;
pub const MAX_HOSTNAME_LEN: usize = 253;
pub const MAX_COMMAND_LEN: usize = 65536;
pub const MAX_ENV_VAR_LEN: usize = 32768;
pub const MAX_ENV_KEY_LEN: usize = 256;
pub const MAX_IPV4_LEN: usize = 15;    // "255.255.255.255"
pub const MAX_IPV6_LEN: usize = 39;    // full IPv6 text

/// Errors produced by input validation routines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    /// Input exceeds the allowed maximum length.
    TooLong,
    /// Input contains characters that are not permitted.
    InvalidChars,
    /// Input contains an embedded NUL byte (C-string injection).
    NullByte,
    /// Input contains a path traversal sequence (`..`).
    PathTraversal,
    /// Input is empty where a non-empty value is required.
    EmptyInput,
    /// Integer arithmetic would overflow/underflow.
    Overflow,
    /// Integer arithmetic would underflow.
    Underflow,
    /// The value is outside the permitted numeric range.
    OutOfRange,
}

// ── Path & Filename ────────────────────────────────────────────────────────

/// Validate a filesystem path expressed as a raw byte slice.
///
/// Rules:
/// - Non-empty
/// - ≤ `MAX_PATH_LEN` bytes
/// - No embedded NUL bytes
/// - No `..` path-traversal component
pub fn validate_path(path: &[u8]) -> Result<(), ValidationError> {
    if path.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if path.len() > MAX_PATH_LEN {
        return Err(ValidationError::TooLong);
    }
    for &b in path {
        if b == 0 {
            return Err(ValidationError::NullByte);
        }
    }
    // Reject any `..` component separated by `/`, `\`, `:` or at the boundaries.
    let mut i = 0usize;
    while i < path.len() {
        if path[i] == b'.' {
            if i + 1 < path.len() && path[i + 1] == b'.' {
                let before_ok = i == 0 || path[i - 1] == b'/' || path[i - 1] == b'\\' || path[i - 1] == b':';
                let after_ok = i + 2 >= path.len() || path[i + 2] == b'/' || path[i + 2] == b'\\' || path[i + 2] == b':';
                if before_ok && after_ok {
                    return Err(ValidationError::PathTraversal);
                }
            }
        }
        i += 1;
    }
    Ok(())
}

/// Validate a filename (single component — no directory separators).
pub fn validate_filename(name: &[u8]) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if name.len() > MAX_FILENAME_LEN {
        return Err(ValidationError::TooLong);
    }
    if name == b"." || name == b".." {
        return Err(ValidationError::PathTraversal);
    }
    for &b in name {
        if b == 0 || b == b'/' || b == b'\\' {
            return Err(ValidationError::InvalidChars);
        }
    }
    Ok(())
}

// ── Username / Hostname ────────────────────────────────────────────────────

/// Validate a Unix username per POSIX / IEEE Std 1003.1.
///
/// Rules:
/// - Non-empty
/// - ≤ 32 bytes (`MAX_USERNAME_LEN`)
/// - First byte MUST be an ASCII letter or underscore (`[a-zA-Z_]`).
///   This prevents command-line option injection (e.g., usernames starting with `-`
///   such as `-rf` or `--help` passed into system utilities).
/// - Subsequent bytes MUST be ASCII alphanumeric, `_`, or `-`.
pub fn validate_username(name: &[u8]) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if name.len() > MAX_USERNAME_LEN {
        return Err(ValidationError::TooLong);
    }
    // Prevent argument injection: initial character cannot be `-` or a digit
    let first = name[0];
    if !first.is_ascii_alphabetic() && first != b'_' {
        return Err(ValidationError::InvalidChars);
    }
    for &b in &name[1..] {
        if !b.is_ascii_alphanumeric() && b != b'_' && b != b'-' {
            return Err(ValidationError::InvalidChars);
        }
    }
    Ok(())
}

/// Validate a hostname per RFC 952/1123.
pub fn validate_hostname(name: &[u8]) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if name.len() > MAX_HOSTNAME_LEN {
        return Err(ValidationError::TooLong);
    }
    for &b in name {
        if !b.is_ascii_alphanumeric() && b != b'-' && b != b'.' {
            return Err(ValidationError::InvalidChars);
        }
    }
    Ok(())
}

// ── Environment variables ──────────────────────────────────────────────────

/// Validate an environment variable key (no `=`, no NUL).
pub fn validate_env_key(key: &[u8]) -> Result<(), ValidationError> {
    if key.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if key.len() > MAX_ENV_KEY_LEN {
        return Err(ValidationError::TooLong);
    }
    for &b in key {
        if b == 0 || b == b'=' {
            return Err(ValidationError::InvalidChars);
        }
    }
    Ok(())
}

/// Validate an environment variable value (no NUL; length-bounded).
pub fn validate_env_value(val: &[u8]) -> Result<(), ValidationError> {
    if val.len() > MAX_ENV_VAR_LEN {
        return Err(ValidationError::TooLong);
    }
    for &b in val {
        if b == 0 {
            return Err(ValidationError::NullByte);
        }
    }
    Ok(())
}

// ── Safe integer arithmetic ────────────────────────────────────────────────

/// Checked addition — returns `None` on overflow.
#[inline(always)]
pub fn safe_add(a: usize, b: usize) -> Option<usize> {
    a.checked_add(b)
}

/// Checked subtraction — returns `None` on underflow.
#[inline(always)]
pub fn safe_sub(a: usize, b: usize) -> Option<usize> {
    a.checked_sub(b)
}

/// Checked multiplication — returns `None` on overflow.
#[inline(always)]
pub fn safe_mul(a: usize, b: usize) -> Option<usize> {
    a.checked_mul(b)
}

/// Saturating cast from `usize` to `u32` without panicking.
#[inline(always)]
pub fn usize_to_u32_saturating(v: usize) -> u32 {
    if v > u32::MAX as usize {
        u32::MAX
    } else {
        v as u32
    }
}

/// Checked cast from `usize` to `u16` — returns `None` if out of range.
#[inline(always)]
pub fn usize_to_u16(v: usize) -> Option<u16> {
    if v > u16::MAX as usize { None } else { Some(v as u16) }
}

// ── Log sanitisation ───────────────────────────────────────────────────────

/// Copy `input` into `out`, replacing every non-printable / non-ASCII byte
/// with `?`.  Returns the number of bytes written (≤ both slice lengths).
pub fn sanitize_for_log(input: &[u8], out: &mut [u8]) -> usize {
    let max = out.len().min(input.len());
    for i in 0..max {
        let b = input[i];
        out[i] = if b.is_ascii_graphic() || b == b' ' { b } else { b'?' };
    }
    max
}

// ── Command / shell ────────────────────────────────────────────────────────

/// Validate a shell command string: length-bounded, no NUL bytes.
/// Does NOT attempt to parse shell syntax — callers should use allowlists.
pub fn validate_command(cmd: &[u8]) -> Result<(), ValidationError> {
    if cmd.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if cmd.len() > MAX_COMMAND_LEN {
        return Err(ValidationError::TooLong);
    }
    for &b in cmd {
        if b == 0 {
            return Err(ValidationError::NullByte);
        }
    }
    Ok(())
}

// ── Network address ────────────────────────────────────────────────────────

/// Validate a textual IPv4 address (digits and dots, ≤ 15 bytes).
/// Rejects leading zeros in multi-digit octets (e.g., `010.0.0.1`) to prevent
/// octal parser differential and SSRF security bypass vulnerabilities.
pub fn validate_ipv4(addr: &[u8]) -> Result<(), ValidationError> {
    if addr.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if addr.len() > MAX_IPV4_LEN {
        return Err(ValidationError::TooLong);
    }
    let mut octet_count = 0u8;
    let mut octet_val: u32 = 0;
    let mut octet_len = 0u8;
    let mut octet_has_leading_zero = false;
    for &b in addr {
        if b == b'.' {
            if octet_val > 255 || octet_len == 0 || (octet_len > 1 && octet_has_leading_zero) {
                return Err(ValidationError::OutOfRange);
            }
            octet_count += 1;
            octet_val = 0;
            octet_len = 0;
            octet_has_leading_zero = false;
        } else if b.is_ascii_digit() {
            if octet_len == 0 && b == b'0' {
                octet_has_leading_zero = true;
            }
            octet_val = octet_val.saturating_mul(10).saturating_add((b - b'0') as u32);
            octet_len += 1;
            if octet_len > 3 {
                return Err(ValidationError::OutOfRange);
            }
        } else {
            return Err(ValidationError::InvalidChars);
        }
    }
    // Validate last octet and total count.
    if octet_val > 255 || octet_len == 0 || octet_count != 3 || (octet_len > 1 && octet_has_leading_zero) {
        return Err(ValidationError::OutOfRange);
    }
    Ok(())
}

/// Validate a textual IPv6 address (hexadecimal blocks separated by colons, ≤ 39 bytes).
pub fn validate_ipv6(addr: &[u8]) -> Result<(), ValidationError> {
    if addr.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if addr.len() > MAX_IPV6_LEN {
        return Err(ValidationError::TooLong);
    }

    let mut colons = 0;
    let mut double_colon = false;
    let mut block_len = 0;
    let mut i = 0;

    while i < addr.len() {
        let b = addr[i];
        if b == b':' {
            if i + 1 < addr.len() && addr[i + 1] == b':' {
                if double_colon {
                    return Err(ValidationError::InvalidChars);
                }
                double_colon = true;
                colons += 1;
                i += 2;
                block_len = 0;
                if i < addr.len() && addr[i] == b':' {
                    return Err(ValidationError::InvalidChars);
                }
                continue;
            }
            if i == 0 || i + 1 == addr.len() {
                return Err(ValidationError::InvalidChars);
            }
            colons += 1;
            block_len = 0;
        } else if b.is_ascii_hexdigit() {
            block_len += 1;
            if block_len > 4 {
                return Err(ValidationError::OutOfRange);
            }
        } else {
            return Err(ValidationError::InvalidChars);
        }
        i += 1;
    }

    if colons > 7 {
        return Err(ValidationError::OutOfRange);
    }
    if !double_colon && colons != 7 {
        return Err(ValidationError::OutOfRange);
    }

    Ok(())
}

// ── Port number ────────────────────────────────────────────────────────────

/// Validate a TCP/UDP port number (1..=65535).
pub fn validate_port(port: u32) -> Result<(), ValidationError> {
    if port == 0 || port > 65535 {
        return Err(ValidationError::OutOfRange);
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_path_traversal_rejected() {
        assert!(validate_path(b"../../etc/passwd").is_err());
        assert!(validate_path(b"/foo/../bar").is_err());
        assert!(validate_path(b"..").is_err());
        assert!(validate_path(b"..\\..\\etc\\passwd").is_err());
        assert!(validate_path(b"C:\\foo\\..\\bar").is_err());
        assert!(validate_path(b"foo\\..").is_err());
        assert!(validate_path(b"C:..\\passwd").is_err());
        assert!(validate_path(b"file:../secret.txt").is_err());
    }

    #[test]
    fn test_valid_path() {
        assert!(validate_path(b"/usr/share/sigma/config").is_ok());
        assert!(validate_path(b"/home/user/.config").is_ok());
        assert!(validate_path(b"/var/log/app..log").is_ok());
        assert!(validate_path(b"foo/..bar").is_ok());
    }

    #[test]
    fn test_validate_filename() {
        assert_eq!(validate_filename(b"document.pdf"), Ok(()));
        assert_eq!(validate_filename(b".env"), Ok(()));
        assert_eq!(validate_filename(b""), Err(ValidationError::EmptyInput));
        assert_eq!(validate_filename(b"."), Err(ValidationError::PathTraversal));
        assert_eq!(validate_filename(b".."), Err(ValidationError::PathTraversal));
        assert_eq!(validate_filename(b"dir/file"), Err(ValidationError::InvalidChars));
        assert_eq!(validate_filename(b"dir\\file"), Err(ValidationError::InvalidChars));
        assert_eq!(validate_filename(&[b'a', 0, b'b']), Err(ValidationError::InvalidChars));
        let long_name = [b'a'; MAX_FILENAME_LEN + 1];
        assert_eq!(validate_filename(&long_name), Err(ValidationError::TooLong));
    }

    #[test]
    fn test_null_byte_rejected() {
        assert!(validate_path(&[b'/', b'f', 0, b'o']).is_err());
    }

    #[test]
    fn test_safe_arithmetic_overflow() {
        assert_eq!(safe_add(usize::MAX, 1), None);
        assert_eq!(safe_mul(usize::MAX, 2), None);
        assert_eq!(safe_sub(0, 1), None);
    }

    #[test]
    fn test_safe_arithmetic_ok() {
        assert_eq!(safe_add(10, 20), Some(30));
        assert_eq!(safe_mul(3, 7), Some(21));
        assert_eq!(safe_sub(10, 3), Some(7));
    }

    #[test]
    fn test_username_validation() {
        assert_eq!(validate_username(b"alice"), Ok(()));
        assert_eq!(validate_username(b"_alice"), Ok(()));
        assert_eq!(validate_username(b"alice-admin"), Ok(()));
        assert_eq!(validate_username(b"user123"), Ok(()));

        // Command-line option injection prevention (leading dash/hyphen)
        assert_eq!(validate_username(b"-option"), Err(ValidationError::InvalidChars));
        assert_eq!(validate_username(b"--help"), Err(ValidationError::InvalidChars));
        assert_eq!(validate_username(b"-rf"), Err(ValidationError::InvalidChars));

        // POSIX compliance (leading digit disallowed)
        assert_eq!(validate_username(b"123user"), Err(ValidationError::InvalidChars));

        // Disallowed special characters
        assert_eq!(validate_username(b"alice@domain"), Err(ValidationError::InvalidChars));
        assert_eq!(validate_username(b"alice;id"), Err(ValidationError::InvalidChars));

        // Empty and length checks
        assert_eq!(validate_username(b""), Err(ValidationError::EmptyInput));
        let long_user = [b'a'; MAX_USERNAME_LEN + 1];
        assert_eq!(validate_username(&long_user), Err(ValidationError::TooLong));
    }

    #[test]
    fn test_ipv4_validation() {
        assert!(validate_ipv4(b"192.168.1.1").is_ok());
        assert!(validate_ipv4(b"0.0.0.0").is_ok());
        assert!(validate_ipv4(b"192.168.0.1").is_ok());
        assert!(validate_ipv4(b"255.255.255.255").is_ok());
        assert!(validate_ipv4(b"256.0.0.1").is_err());
        assert!(validate_ipv4(b"65536.0.0.1").is_err());
        assert!(validate_ipv4(b"99999.0.0.1").is_err());
        assert!(validate_ipv4(b"192.168.1").is_err());

        // Reject multi-digit octets with leading zeros (prevents octal SSRF bypass)
        assert_eq!(validate_ipv4(b"010.0.0.1"), Err(ValidationError::OutOfRange));
        assert_eq!(validate_ipv4(b"192.168.01.1"), Err(ValidationError::OutOfRange));
        assert_eq!(validate_ipv4(b"001.1.1.1"), Err(ValidationError::OutOfRange));
    }

    #[test]
    fn test_ipv6_validation() {
        // Valid full uncontracted address
        assert!(validate_ipv6(b"2001:db8:85a3:0:0:8a2e:370:7334").is_ok());
        // Valid contracted addresses
        assert!(validate_ipv6(b"2001:db8::1").is_ok());
        assert!(validate_ipv6(b"::1").is_ok());
        assert!(validate_ipv6(b"::").is_ok());
        assert!(validate_ipv6(b"2001:db8:85a3::8a2e").is_ok());

        // Empty input
        assert_eq!(validate_ipv6(b""), Err(ValidationError::EmptyInput));
        // Too long
        assert_eq!(validate_ipv6(b"2001:0db8:85a3:0000:0000:8a2e:0370:7334:9999"), Err(ValidationError::TooLong));
        // Block length > 4
        assert!(validate_ipv6(b"20011:db8::1").is_err());
        // Multiple double colons
        assert!(validate_ipv6(b"2001::db8::1").is_err());
        // Invalid characters
        assert!(validate_ipv6(b"2001:db8:85a3:0:0:8a2e:370:733g").is_err());
        // Single colon at start/end
        assert!(validate_ipv6(b":2001::db8").is_err());
        assert!(validate_ipv6(b"2001::db8:").is_err());
        // Consecutive colons
        assert!(validate_ipv6(b"2001:::db8").is_err());
        // Incorrect block counts
        assert!(validate_ipv6(b"2001:db8:85a3").is_err());
        assert!(validate_ipv6(b"2001:db8:85a3:0:0:8a2e:370:7334:1234").is_err());
    }

    #[test]
    fn test_port_validation() {
        assert!(validate_port(80).is_ok());
        assert!(validate_port(65535).is_ok());
        assert!(validate_port(0).is_err());
        assert!(validate_port(65536).is_err());
    }

    #[test]
    fn test_sanitize_for_log() {
        let input = b"hello\x00world\x01";
        let mut out = [0u8; 20];
        let n = sanitize_for_log(input, &mut out);
        assert_eq!(&out[..n], b"hello?world?");
    }
}
