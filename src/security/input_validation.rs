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
pub const MAX_IPV4_LEN: usize = 15; // "255.255.255.255"
pub const MAX_IPV6_LEN: usize = 39; // full IPv6 text
pub const MAX_URL_LEN: usize = 2048; // Standard HTTP/HTTPS maximum URL length

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
/// - No ASCII control characters (`b < 32` or `b == 127`) to prevent log/terminal injection
/// - No `..` path-traversal component
pub fn validate_path(path: &[u8]) -> Result<(), ValidationError> {
    let len = path.len();
    if len == 0 {
        return Err(ValidationError::EmptyInput);
    }
    if len > MAX_PATH_LEN {
        return Err(ValidationError::TooLong);
    }

    // Reject URL-encoded and double-URL-encoded path traversal sequences (%2e, %2f, %5c, %25...)
    // to prevent parser differential / decoding bypass attacks (CWE-22).
    if path.windows(5).any(|w| {
        // Double URL-encoded separators or dots (%252e, %252E, %252f, %252F, %255c, %255C)
        w[0] == b'%'
            && w[1] == b'2'
            && w[2] == b'5'
            && ((w[3] == b'2' && (w[4] == b'e' || w[4] == b'E' || w[4] == b'f' || w[4] == b'F'))
                || (w[3] == b'5' && (w[4] == b'c' || w[4] == b'C')))
    }) || path.windows(4).any(|w| {
        // Mixed literal dot and URL-encoded dot (.%2e, .%2E, %2e., %2E.)
        (w[0] == b'.' && w[1] == b'%' && w[2] == b'2' && (w[3] == b'e' || w[3] == b'E'))
            || (w[0] == b'%' && w[1] == b'2' && (w[2] == b'e' || w[2] == b'E') && w[3] == b'.')
    }) || path.windows(6).any(|w| {
        // Double URL-encoded dot pair (%2e%2e, %2E%2E, %2e%2E, %2E%2e)
        w[0] == b'%'
            && w[1] == b'2'
            && (w[2] == b'e' || w[2] == b'E')
            && w[3] == b'%'
            && w[4] == b'2'
            && (w[5] == b'e' || w[5] == b'E')
    }) || path.windows(3).any(|w| {
        // Single URL-encoded separators (%2f, %2F, %5c, %5C)
        w[0] == b'%'
            && ((w[1] == b'2' && (w[2] == b'f' || w[2] == b'F'))
                || (w[1] == b'5' && (w[2] == b'c' || w[2] == b'C')))
    }) {
        return Err(ValidationError::PathTraversal);
    }

    // Single-pass byte slice scan combining NUL-byte injection checks,
    // ASCII control character rejection, and path-traversal detection.
    // Inspects path segments bounded by directory separators (`/`, `\`, `:`).
    // Rejects any segment consisting solely of dot (`.`) characters with len >= 2
    // (e.g. `..`, `...`, `....`) to prevent multi-dot path traversal bypasses.
    let mut seg_len = 0usize;
    let mut seg_only_dots = true;

    for &b in path {
        if b == 0 {
            return Err(ValidationError::NullByte);
        }
        if b < 32 || b == 127 {
            return Err(ValidationError::InvalidChars);
        }

        if b == b'/' || b == b'\\' || b == b':' {
            if seg_only_dots && seg_len >= 2 {
                return Err(ValidationError::PathTraversal);
            }
            seg_len = 0;
            seg_only_dots = true;
        } else {
            seg_len += 1;
            if b != b'.' {
                seg_only_dots = false;
            }
        }
    }

    if seg_only_dots && seg_len >= 2 {
        return Err(ValidationError::PathTraversal);
    }

    Ok(())
}

/// Check whether a filename stem (before the first dot) matches an MS-DOS reserved device name
/// (`CON`, `PRN`, `AUX`, `NUL`, `COM1`..`COM9`, `LPT1`..`LPT9`, `CLOCK$`).
/// Rejects reserved device names case-insensitively to prevent file system hijacking,
/// denial-of-service, and arbitrary device I/O vulnerabilities (CWE-667 / CWE-706).
fn is_reserved_dos_device(stem: &[u8]) -> bool {
    if stem.is_empty() {
        return false;
    }
    if stem.len() == 3 {
        if stem.eq_ignore_ascii_case(b"CON")
            || stem.eq_ignore_ascii_case(b"PRN")
            || stem.eq_ignore_ascii_case(b"AUX")
            || stem.eq_ignore_ascii_case(b"NUL")
        {
            return true;
        }
    } else if stem.len() == 4 {
        let prefix = &stem[..3];
        let digit = stem[3];
        if (prefix.eq_ignore_ascii_case(b"COM") || prefix.eq_ignore_ascii_case(b"LPT"))
            && (b'1'..=b'9').contains(&digit)
        {
            return true;
        }
    } else if stem.len() == 6 && stem.eq_ignore_ascii_case(b"CLOCK$") {
        return true;
    }
    false
}

/// Validate a filename (single component — no directory separators).
///
/// Rules:
/// - Non-empty, ≤ `MAX_FILENAME_LEN` bytes
/// - Not `.` or `..`
/// - No MS-DOS reserved device names (`CON`, `PRN`, `AUX`, `NUL`, `COM1`..`9`, `LPT1`..`9`, `CLOCK$`)
/// - No directory separators (`/` or `\`)
/// - No embedded NUL bytes (returns `NullByte`)
/// - No ASCII control characters (`b < 32` or `b == 127`) to prevent log injection (CWE-117),
///   terminal escape sequence hijacking (CWE-150), and shell script line-splitting.
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
    let stem = name.split(|&b| b == b'.').next().unwrap_or(name);
    if is_reserved_dos_device(stem) {
        return Err(ValidationError::InvalidChars);
    }
    for &b in name {
        if b == 0 {
            return Err(ValidationError::NullByte);
        }
        if b < 32 || b == 127 || b == b'/' || b == b'\\' {
            return Err(ValidationError::InvalidChars);
        }
    }
    Ok(())
}

// ── URL Validation ─────────────────────────────────────────────────────────

/// Validate a URL byte slice per RFC 3986 / RFC 3987.
///
/// Rules:
/// - Non-empty, ≤ `MAX_URL_LEN` (2048 bytes)
/// - Scheme must be allowlisted (`http://` or `https://`)
/// - Rejects NUL bytes (`NullByte`) and percent-encoded NUL bytes (`%00`) to prevent C-ABI string truncation (CWE-626)
/// - Rejects ASCII control characters / spaces (`b <= 32` or `b == 127`), and percent-encoded CRLF / control sequences
///   (`%0d`, `%0a`, `%1b`, `%7f`) to prevent HTTP request smuggling, CRLF injection, and header injection (CWE-113 / CWE-150 / CWE-117)
/// - Rejects userinfo credentials (`user:pass@` or `%40`) in authority to prevent SSRF and credential disclosure (CWE-918)
pub fn validate_url(url: &[u8]) -> Result<(), ValidationError> {
    if url.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if url.len() > MAX_URL_LEN {
        return Err(ValidationError::TooLong);
    }

    // Reject percent-encoded (%XY) and double-percent-encoded (%25XY) NUL bytes and
    // ASCII control / CRLF characters (0x00..0x1F, 0x7F) to prevent HTTP header injection,
    // request smuggling, terminal hijacking, and C-ABI string truncation (CWE-113 / CWE-150 / CWE-626 / CWE-918).
    let is_ctrl_hex = |h1: u8, h2: u8| -> bool {
        ((h1 == b'0' || h1 == b'1') && h2.is_ascii_hexdigit())
            || (h1 == b'7' && (h2 == b'f' || h2 == b'F'))
    };

    let has_single_encoded_ctrl = url
        .windows(3)
        .any(|w| w[0] == b'%' && is_ctrl_hex(w[1], w[2]));
    let has_double_encoded_ctrl = url
        .windows(5)
        .any(|w| w[0] == b'%' && w[1] == b'2' && w[2] == b'5' && is_ctrl_hex(w[3], w[4]));

    if has_single_encoded_ctrl || has_double_encoded_ctrl {
        let is_nul_single = url
            .windows(3)
            .any(|w| w[0] == b'%' && w[1] == b'0' && w[2] == b'0');
        let is_nul_double = url
            .windows(5)
            .any(|w| w[0] == b'%' && w[1] == b'2' && w[2] == b'5' && w[3] == b'0' && w[4] == b'0');

        if is_nul_single || is_nul_double {
            return Err(ValidationError::NullByte);
        }
        return Err(ValidationError::InvalidChars);
    }

    // Check scheme case-insensitively per RFC 3986: http:// or https://
    let host_start = if url.len() >= 7 && url[..7].eq_ignore_ascii_case(b"http://") {
        7
    } else if url.len() >= 8 && url[..8].eq_ignore_ascii_case(b"https://") {
        8
    } else {
        return Err(ValidationError::InvalidChars);
    };

    // Find end of host/authority part (before '/', '?', '#', or '\')
    let rest = &url[host_start..];
    let host_len = rest
        .iter()
        .position(|&b| b == b'/' || b == b'?' || b == b'#' || b == b'\\')
        .unwrap_or(rest.len());
    let authority = &rest[..host_len];

    if authority.is_empty() {
        return Err(ValidationError::InvalidChars);
    }

    // Reject userinfo credentials and percent-encoded / double percent-encoded boundary delimiters
    // (%2f, %2F, %23, %3f, %3F, %5c, %5C, %40, %2540, %2523, %253f, %252f, %255c) in authority
    // to prevent authority boundary ambiguity, SSRF, host header injection, and open redirect bypasses
    // (CWE-918 / CWE-292 / CWE-838 / CWE-601).
    if authority.contains(&b'@')
        || authority.windows(3).any(|w| {
            w[0] == b'%'
                && ((w[1] == b'2' && (w[2] == b'f' || w[2] == b'F' || w[2] == b'3'))
                    || (w[1] == b'3' && (w[2] == b'f' || w[2] == b'F'))
                    || (w[1] == b'4' && w[2] == b'0')
                    || (w[1] == b'5' && (w[2] == b'c' || w[2] == b'C')))
        })
        || authority.windows(5).any(|w| {
            w[0] == b'%'
                && w[1] == b'2'
                && w[2] == b'5'
                && ((w[3] == b'2' && (w[4] == b'f' || w[4] == b'F' || w[4] == b'3'))
                    || (w[3] == b'3' && (w[4] == b'f' || w[4] == b'F'))
                    || (w[3] == b'4' && w[4] == b'0')
                    || (w[3] == b'5' && (w[4] == b'c' || w[4] == b'C')))
        })
    {
        return Err(ValidationError::InvalidChars);
    }

    // Parse and validate IPv6 literal bracket syntax or host:port structure within authority segment per RFC 3986
    if authority.starts_with(b"[") {
        let close_pos = authority.iter().position(|&b| b == b']');
        let close = match close_pos {
            Some(idx) if idx > 1 => idx,
            _ => return Err(ValidationError::InvalidChars),
        };
        let ipv6_bytes = &authority[1..close];
        validate_ipv6(ipv6_bytes)?;

        let rest_auth = &authority[close + 1..];
        if !rest_auth.is_empty() {
            if !rest_auth.starts_with(b":") || rest_auth.len() == 1 {
                return Err(ValidationError::InvalidChars);
            }
            let port_bytes = &rest_auth[1..];
            let port_num = parse_port_bytes(port_bytes).ok_or(ValidationError::InvalidChars)?;
            validate_port(port_num)?;
        }
    } else {
        if authority.contains(&b'[') || authority.contains(&b']') {
            return Err(ValidationError::InvalidChars);
        }
        if let Some(colon_pos) = authority.iter().position(|&b| b == b':') {
            let host_part = &authority[..colon_pos];
            let port_bytes = &authority[colon_pos + 1..];
            if host_part.is_empty() || port_bytes.is_empty() {
                return Err(ValidationError::InvalidChars);
            }
            let port_num = parse_port_bytes(port_bytes).ok_or(ValidationError::InvalidChars)?;
            validate_port(port_num)?;
        }
    }

    // Iterate through all bytes: check for NUL, control chars/spaces, and backslashes (`\`)
    // Rejects backslashes to prevent WHATWG vs RFC 3986 URL parser differential attacks (CWE-601 / CWE-918)
    for &b in url {
        if b == 0 {
            return Err(ValidationError::NullByte);
        }
        if b <= 32 || b == 127 || b == b'\\' {
            return Err(ValidationError::InvalidChars);
        }
    }

    Ok(())
}

// ── MAC Address ─────────────────────────────────────────────────────────────

/// Validate an IEEE 802 48-bit MAC address or EUI-64 64-bit MAC address
/// (e.g., `00:1A:2B:3C:4D:5E`, `00-1A-2B-3C-4D-5E`, or 23-byte `00:1A:2B:3C:4D:5E:6F:70`).
/// Enforces strict ASCII format, NUL-byte rejection, control character rejection, and uniform delimiter checking
/// (all colons or all hyphens) to prevent C-string truncation (CWE-626), terminal sequence injection (CWE-150),
/// and parser differential attacks across network drivers and security components.
pub fn validate_mac_address(mac: &[u8]) -> Result<(), ValidationError> {
    if mac.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if mac.len() > 23 {
        return Err(ValidationError::TooLong);
    }
    if mac.len() != 17 && mac.len() != 23 {
        return Err(ValidationError::OutOfRange);
    }

    for &b in mac {
        if b == 0 {
            return Err(ValidationError::NullByte);
        }
        if b < 32 || b == 127 {
            return Err(ValidationError::InvalidChars);
        }
    }

    let delimiter = mac[2];
    if delimiter != b':' && delimiter != b'-' {
        return Err(ValidationError::InvalidChars);
    }

    for (i, &b) in mac.iter().enumerate() {
        if (i % 3) == 2 {
            if b != delimiter {
                return Err(ValidationError::InvalidChars);
            }
        } else if !b.is_ascii_hexdigit() {
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
///
/// Rules:
/// - Non-empty, total length ≤ `MAX_HOSTNAME_LEN` (253 bytes)
/// - Composed of dot-separated labels
/// - Each label must be 1..=63 bytes long
/// - Label characters must be ASCII alphanumeric or hyphen (`-`)
/// - Labels MUST NOT start or end with a hyphen (`-`). This prevents command-line option
///   injection (e.g. `-oProxyCommand=...` hostnames passed to network utilities).
/// - No empty labels (disallows leading dot, trailing dot, and consecutive dots `..`).
pub fn validate_hostname(name: &[u8]) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if name.len() > MAX_HOSTNAME_LEN {
        return Err(ValidationError::TooLong);
    }

    for label in name.split(|&b| b == b'.') {
        if label.is_empty() {
            return Err(ValidationError::InvalidChars);
        }
        if label.len() > 63 {
            return Err(ValidationError::TooLong);
        }
        if label[0] == b'-' || label[label.len() - 1] == b'-' {
            return Err(ValidationError::InvalidChars);
        }
        for &b in label {
            if !b.is_ascii_alphanumeric() && b != b'-' {
                return Err(ValidationError::InvalidChars);
            }
        }
    }

    Ok(())
}

// ── Environment variables ──────────────────────────────────────────────────

/// Validate an environment variable key per POSIX / IEEE Std 1003.1 (`[a-zA-Z_][a-zA-Z0-9_]*`).
///
/// Rules:
/// - Non-empty, ≤ `MAX_ENV_KEY_LEN` (256 bytes)
/// - First byte MUST be ASCII alphabetic or underscore (`[a-zA-Z_]`).
/// - Subsequent bytes MUST be ASCII alphanumeric or underscore (`[a-zA-Z0-9_]`).
/// Disallows leading digits, hyphens, dots, equals signs (`=`), NUL bytes, or arbitrary special
/// characters that cause parser differential or environment injection in subprocesses / shells.
pub fn validate_env_key(key: &[u8]) -> Result<(), ValidationError> {
    if key.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if key.len() > MAX_ENV_KEY_LEN {
        return Err(ValidationError::TooLong);
    }
    let first = key[0];
    if !first.is_ascii_alphabetic() && first != b'_' {
        return Err(ValidationError::InvalidChars);
    }
    for &b in &key[1..] {
        if !b.is_ascii_alphanumeric() && b != b'_' {
            return Err(ValidationError::InvalidChars);
        }
    }
    Ok(())
}

/// Validate an environment variable value (no NUL; length-bounded; no unsafe control characters).
/// Rejects non-whitespace ASCII control characters (`b < 32` except `\t`, `\n`, `\r`) and DEL (`127`)
/// to prevent terminal sequence hijacking (CWE-150) and environment injection in subprocesses.
pub fn validate_env_value(val: &[u8]) -> Result<(), ValidationError> {
    if val.len() > MAX_ENV_VAR_LEN {
        return Err(ValidationError::TooLong);
    }
    for &b in val {
        if b == 0 {
            return Err(ValidationError::NullByte);
        }
        if (b < 32 && b != b'\t' && b != b'\n' && b != b'\r') || b == 127 {
            return Err(ValidationError::InvalidChars);
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
    if v > u16::MAX as usize {
        None
    } else {
        Some(v as u16)
    }
}

// ── Log sanitisation ───────────────────────────────────────────────────────

/// Copy `input` into `out`, replacing every non-printable / non-ASCII byte
/// with `?`.  Returns the number of bytes written (≤ both slice lengths).
/// Prevents terminal escape injection (CWE-150) and control character log tampering.
pub fn sanitize_for_log(input: &[u8], out: &mut [u8]) -> usize {
    let max = out.len().min(input.len());
    for i in 0..max {
        let b = input[i];
        out[i] = if b.is_ascii_graphic() || b == b' ' {
            b
        } else {
            b'?'
        };
    }
    max
}

// ── Command / shell ────────────────────────────────────────────────────────

/// Validate a shell command string: length-bounded, no NUL bytes or harmful control characters.
/// Rejects non-whitespace ASCII control characters (`b < 32` except `\t`, `\n`, `\r`) and DEL (`127`)
/// to prevent ANSI escape sequence hijacking (CWE-150) and terminal injection attacks.
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
        if (b < 32 && b != b'\t' && b != b'\n' && b != b'\r') || b == 127 {
            return Err(ValidationError::InvalidChars);
        }
    }
    Ok(())
}

// ── Network address ────────────────────────────────────────────────────────

/// Validate a textual IPv4 address (digits and dots, ≤ 15 bytes).
/// Rejects empty octets (e.g., `.1.2.3`, `1..2.3`, `1.2.3.`), missing or trailing dots,
/// and leading zeros in multi-digit octets (e.g., `010.0.0.1`) to prevent
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
            octet_val = octet_val
                .saturating_mul(10)
                .saturating_add((b - b'0') as u32);
            octet_len += 1;
            if octet_len > 3 {
                return Err(ValidationError::OutOfRange);
            }
        } else {
            return Err(ValidationError::InvalidChars);
        }
    }
    // Validate last octet and total count.
    if octet_val > 255
        || octet_len == 0
        || octet_count != 3
        || (octet_len > 1 && octet_has_leading_zero)
    {
        return Err(ValidationError::OutOfRange);
    }
    Ok(())
}

/// Validate a textual IPv6 address (hexadecimal blocks separated by colons, ≤ 39 bytes).
/// Rejects over-length compressed IPv6 addresses (e.g. `1:2:3:4:5:6:7::8`) with 8 or
/// more explicit blocks to prevent parser differential security vulnerabilities.
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
    let mut blocks = 0;
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
            if block_len == 0 {
                blocks += 1;
            }
            block_len += 1;
            if block_len > 4 {
                return Err(ValidationError::OutOfRange);
            }
        } else {
            return Err(ValidationError::InvalidChars);
        }
        i += 1;
    }

    if colons > 7 || blocks > 8 {
        return Err(ValidationError::OutOfRange);
    }
    if double_colon && blocks >= 8 {
        return Err(ValidationError::OutOfRange);
    }
    if !double_colon && (colons != 7 || blocks != 8) {
        return Err(ValidationError::OutOfRange);
    }

    Ok(())
}

// ── Port number ────────────────────────────────────────────────────────────

/// Parse ASCII byte slice into a port number (`u32`), returning `None` if invalid or overflowed.
fn parse_port_bytes(bytes: &[u8]) -> Option<u32> {
    if bytes.is_empty() {
        return None;
    }
    let mut val: u32 = 0;
    for &b in bytes {
        if !b.is_ascii_digit() {
            return None;
        }
        val = val.checked_mul(10)?.checked_add((b - b'0') as u32)?;
    }
    Some(val)
}

/// Validate a TCP/UDP port number (1..=65535).
pub fn validate_port(port: u32) -> Result<(), ValidationError> {
    if port == 0 || port > 65535 {
        return Err(ValidationError::OutOfRange);
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
#[cfg(test)]
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

        // Multi-dot segment path traversal bypass prevention (`...`, `....`, `.....`)
        assert_eq!(
            validate_path(b"/.../etc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"/..../etc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"C:....\\secret.txt"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(validate_path(b"..."), Err(ValidationError::PathTraversal));
        assert_eq!(
            validate_path(b"foo/.../bar"),
            Err(ValidationError::PathTraversal)
        );

        // URL-encoded path traversal sequence rejection (%2e%2e, %2E%2E, %2f, %2F, %5c, %5C)
        assert_eq!(
            validate_path(b"/var/www/%2e%2e/etc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"/var/www/%2E%2E/etc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"/var/www/%2fetc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"/var/www/%2Fetc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"/var/www/%5cetc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"/var/www/%5Cetc/passwd"),
            Err(ValidationError::PathTraversal)
        );

        // Mixed URL-encoded dot and double-URL-encoded traversal sequence rejection
        assert_eq!(
            validate_path(b"/var/www/.%2e/etc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"/var/www/%2e./etc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"/var/www/.%2E/etc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"/var/www/%2E./etc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"/var/www/%252e%252e/etc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"/var/www/%252fetc/passwd"),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_path(b"/var/www/%255cetc/passwd"),
            Err(ValidationError::PathTraversal)
        );
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
        assert_eq!(
            validate_filename(b".."),
            Err(ValidationError::PathTraversal)
        );
        assert_eq!(
            validate_filename(b"dir/file"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_filename(b"dir\\file"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_filename(&[b'a', 0, b'b']),
            Err(ValidationError::NullByte)
        );

        // ASCII control character injection prevention (prevents log injection and ANSI escape sequence hijacking)
        assert_eq!(
            validate_filename(b"file\nname.txt"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_filename(b"file\rname.txt"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_filename(b"file\tname.txt"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_filename(b"file\x1b[31m.txt"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_filename(b"file\x7f.txt"),
            Err(ValidationError::InvalidChars)
        );

        // MS-DOS reserved device name rejection (CWE-667 / CWE-706)
        assert_eq!(
            validate_filename(b"CON"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_filename(b"con"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_filename(b"aux.json"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_filename(b"COM1.txt"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_filename(b"lpt9.log"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_filename(b"CLOCK$"),
            Err(ValidationError::InvalidChars)
        );

        // Legitimate filenames containing reserved words as substrings or prefixes
        assert_eq!(validate_filename(b"concept.txt"), Ok(()));
        assert_eq!(validate_filename(b"console.log"), Ok(()));
        assert_eq!(validate_filename(b"auxiliary.c"), Ok(()));
        assert_eq!(validate_filename(b"computer.png"), Ok(()));

        let long_name = [b'a'; MAX_FILENAME_LEN + 1];
        assert_eq!(validate_filename(&long_name), Err(ValidationError::TooLong));
    }

    #[test]
    fn test_path_control_char_rejected() {
        assert_eq!(
            validate_path(b"/usr/bin/foo\nbar"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_path(b"/var/log/app\r.log"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_path(b"/etc/config\x1b[31m"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_path(b"/tmp/file\x7f"),
            Err(ValidationError::InvalidChars)
        );
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
    fn test_hostname_validation() {
        assert_eq!(validate_hostname(b"example.com"), Ok(()));
        assert_eq!(validate_hostname(b"sub.domain-name.co.uk"), Ok(()));
        assert_eq!(validate_hostname(b"localhost"), Ok(()));
        assert_eq!(validate_hostname(b"node-123"), Ok(()));

        // Command-line option injection prevention (leading hyphen in label)
        assert_eq!(
            validate_hostname(b"-oProxyCommand"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_hostname(b"sub.-domain.com"),
            Err(ValidationError::InvalidChars)
        );

        // Trailing hyphen in label
        assert_eq!(
            validate_hostname(b"domain-.com"),
            Err(ValidationError::InvalidChars)
        );

        // Empty labels / consecutive dots / leading or trailing dot
        assert_eq!(
            validate_hostname(b"example..com"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_hostname(b".example.com"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_hostname(b"example.com."),
            Err(ValidationError::InvalidChars)
        );

        // Empty input
        assert_eq!(validate_hostname(b""), Err(ValidationError::EmptyInput));

        // Label length > 63
        let long_label = [b'a'; 64];
        assert_eq!(
            validate_hostname(&long_label),
            Err(ValidationError::TooLong)
        );

        // Total hostname length > 253
        let long_hostname = [b'a'; MAX_HOSTNAME_LEN + 1];
        assert_eq!(
            validate_hostname(&long_hostname),
            Err(ValidationError::TooLong)
        );
    }

    #[test]
    fn test_env_value_validation() {
        assert_eq!(validate_env_value(b"plain text value"), Ok(()));
        assert_eq!(validate_env_value(b"/usr/local/bin:/usr/bin"), Ok(()));
        assert_eq!(validate_env_value(b"line1\nline2\r\nline3\t"), Ok(()));
        assert_eq!(validate_env_value(b""), Ok(()));

        // NUL byte rejection
        assert_eq!(
            validate_env_value(&[b'a', 0, b'b']),
            Err(ValidationError::NullByte)
        );

        // Control character rejection (CWE-150 / ANSI terminal sequence hijacking)
        assert_eq!(
            validate_env_value(b"val\x1b[31m"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_env_value(b"val\x07bell"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_env_value(b"val\x7fdel"),
            Err(ValidationError::InvalidChars)
        );

        // Exceed max length
        let long_val = [b'v'; MAX_ENV_VAR_LEN + 1];
        assert_eq!(validate_env_value(&long_val), Err(ValidationError::TooLong));
    }

    #[test]
    fn test_env_key_validation() {
        assert_eq!(validate_env_key(b"PATH"), Ok(()));
        assert_eq!(validate_env_key(b"_FOO123"), Ok(()));
        assert_eq!(validate_env_key(b"FOO_BAR"), Ok(()));
        assert_eq!(validate_env_key(b"A"), Ok(()));

        // Disallow leading digit or hyphen or dot
        assert_eq!(
            validate_env_key(b"123KEY"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_env_key(b"-KEY"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_env_key(b".KEY"),
            Err(ValidationError::InvalidChars)
        );

        // Disallow special characters inside key
        assert_eq!(
            validate_env_key(b"KEY=VAL"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_env_key(b"KEY-NAME"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_env_key(b"KEY.NAME"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_env_key(b"KEY@NAME"),
            Err(ValidationError::InvalidChars)
        );

        // Empty and too long
        assert_eq!(validate_env_key(b""), Err(ValidationError::EmptyInput));
        let long_key = [b'A'; MAX_ENV_KEY_LEN + 1];
        assert_eq!(validate_env_key(&long_key), Err(ValidationError::TooLong));
    }

    #[test]
    fn test_username_validation() {
        assert_eq!(validate_username(b"alice"), Ok(()));
        assert_eq!(validate_username(b"_alice"), Ok(()));
        assert_eq!(validate_username(b"alice-admin"), Ok(()));
        assert_eq!(validate_username(b"user123"), Ok(()));

        // Command-line option injection prevention (leading dash/hyphen)
        assert_eq!(
            validate_username(b"-option"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_username(b"--help"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_username(b"-rf"),
            Err(ValidationError::InvalidChars)
        );

        // POSIX compliance (leading digit disallowed)
        assert_eq!(
            validate_username(b"123user"),
            Err(ValidationError::InvalidChars)
        );

        // Disallowed special characters
        assert_eq!(
            validate_username(b"alice@domain"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_username(b"alice;id"),
            Err(ValidationError::InvalidChars)
        );

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
        assert_eq!(
            validate_ipv4(b"010.0.0.1"),
            Err(ValidationError::OutOfRange)
        );
        assert_eq!(
            validate_ipv4(b"192.168.01.1"),
            Err(ValidationError::OutOfRange)
        );
        assert_eq!(
            validate_ipv4(b"001.1.1.1"),
            Err(ValidationError::OutOfRange)
        );

        // Reject trailing dots, leading dots, and empty/consecutive dot octets
        assert_eq!(
            validate_ipv4(b"127.0.0.1."),
            Err(ValidationError::OutOfRange)
        );
        assert_eq!(validate_ipv4(b".1.2.3.4"), Err(ValidationError::OutOfRange));
        assert_eq!(
            validate_ipv4(b"192.168..1"),
            Err(ValidationError::OutOfRange)
        );
        assert_eq!(validate_ipv4(b"..."), Err(ValidationError::OutOfRange));
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
        assert_eq!(
            validate_ipv6(b"2001:0db8:85a3:0000:0000:8a2e:0370:7334:9999"),
            Err(ValidationError::TooLong)
        );
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

        // Over-length compressed IPv6 addresses (8 or more explicit blocks with double colon)
        assert_eq!(
            validate_ipv6(b"1:2:3:4:5:6:7::8"),
            Err(ValidationError::OutOfRange)
        );
        assert_eq!(
            validate_ipv6(b"1::2:3:4:5:6:7:8"),
            Err(ValidationError::OutOfRange)
        );
    }

    #[test]
    fn test_port_validation() {
        assert!(validate_port(80).is_ok());
        assert!(validate_port(65535).is_ok());
        assert!(validate_port(0).is_err());
        assert!(validate_port(65536).is_err());
    }

    #[test]
    fn test_validate_command() {
        assert_eq!(validate_command(b"ls -la /tmp"), Ok(()));
        assert_eq!(validate_command(b"echo \"hello\nworld\""), Ok(()));
        assert_eq!(validate_command(b"cat file.txt\r\n"), Ok(()));
        assert_eq!(validate_command(b"printf 'a\tb'"), Ok(()));

        assert_eq!(validate_command(b""), Err(ValidationError::EmptyInput));
        assert_eq!(
            validate_command(&[b'a', 0, b'b']),
            Err(ValidationError::NullByte)
        );

        // ASCII control character injection prevention (prevents ANSI escape hijacking and terminal injection)
        assert_eq!(
            validate_command(b"echo \x1b[31mred\x1b[0m"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_command(b"ls \x07"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_command(b"rm \x7f"),
            Err(ValidationError::InvalidChars)
        );

        let long_cmd = vec![b'a'; MAX_COMMAND_LEN + 1];
        assert_eq!(validate_command(&long_cmd), Err(ValidationError::TooLong));
    }

    #[test]
    fn test_sanitize_for_log() {
        let input = b"hello\x00world\x01";
        let mut out = [0u8; 20];
        let n = sanitize_for_log(input, &mut out);
        assert_eq!(&out[..n], b"hello?world?");
    }

    #[test]
    fn test_validate_url() {
        assert_eq!(validate_url(b"http://example.com"), Ok(()));
        assert_eq!(
            validate_url(b"https://example.com/path?query=1#frag"),
            Ok(())
        );
        assert_eq!(validate_url(b"http://127.0.0.1:8080/api/v1"), Ok(()));

        // ASCII case-insensitive scheme matching (RFC 3986)
        assert_eq!(validate_url(b"HTTP://example.com"), Ok(()));
        assert_eq!(validate_url(b"Https://example.com/path"), Ok(()));

        // Backslash rejection (WHATWG vs RFC 3986 parser differential / SSRF prevention CWE-601 / CWE-918)
        assert_eq!(
            validate_url(b"http://example.com\\path"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"https://example.com\\@evil.com"),
            Err(ValidationError::InvalidChars)
        );

        // Empty and length checks
        assert_eq!(validate_url(b""), Err(ValidationError::EmptyInput));
        let long_url = [b'a'; MAX_URL_LEN + 1];
        assert_eq!(validate_url(&long_url), Err(ValidationError::TooLong));

        // Missing or invalid scheme
        assert_eq!(
            validate_url(b"ftp://example.com"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"example.com"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http:/example.com"),
            Err(ValidationError::InvalidChars)
        );

        // Empty authority
        assert_eq!(validate_url(b"http://"), Err(ValidationError::InvalidChars));
        assert_eq!(
            validate_url(b"https://"),
            Err(ValidationError::InvalidChars)
        );

        // Userinfo credentials rejection (mitigating SSRF / credential disclosure CWE-918)
        assert_eq!(
            validate_url(b"http://admin:secret@example.com/path"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"https://user@example.com"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://user%40example.com"),
            Err(ValidationError::InvalidChars)
        );

        // Double percent-encoded and single percent-encoded authority boundary injection rejection
        // (mitigating authority ambiguity, SSRF, host header injection CWE-918 / CWE-292 / CWE-838)
        assert_eq!(
            validate_url(b"http://attacker.com%2523.internal.com/"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://admin%2540evil.com/"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://host%2fpath.evil.com/"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://host%252fpath.evil.com/"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://host%253fquery.evil.com/"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://host%255cpath.evil.com/"),
            Err(ValidationError::InvalidChars)
        );

        // Percent-encoded NUL bytes rejection (CWE-626 / C-ABI string truncation)
        assert_eq!(
            validate_url(b"http://example.com/page?id=1%00.jpg"),
            Err(ValidationError::NullByte)
        );
        assert_eq!(
            validate_url(b"http://example.com/page?id=1%2500.jpg"),
            Err(ValidationError::NullByte)
        );

        // Percent-encoded CRLF and control sequence rejection (CWE-113 / HTTP Request Smuggling & Header Injection)
        assert_eq!(
            validate_url(b"http://example.com/api?p=1%0d%0aSet-Cookie:session=evil"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://example.com/api?p=1%0D%0AHost:evil.com"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://example.com/api?p=1%250d%250aSet-Cookie:session=evil"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://example.com/path%09tab"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://example.com/path%251f"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://example.com/path%1b[31m"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://example.com/path%7f"),
            Err(ValidationError::InvalidChars)
        );

        // Control characters, spaces, and NUL bytes
        assert_eq!(
            validate_url(b"http://example.com/path with space"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://example.com/path\n"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://example.com/\x00path"),
            Err(ValidationError::NullByte)
        );
        assert_eq!(
            validate_url(b"http://example.com/\x7fpath"),
            Err(ValidationError::InvalidChars)
        );

        // IPv6 literal URL authority validation
        assert_eq!(validate_url(b"http://[::1]"), Ok(()));
        assert_eq!(validate_url(b"http://[::1]:8080/api"), Ok(()));
        assert_eq!(validate_url(b"https://[2001:db8::1]:443/test"), Ok(()));

        // Malformed IPv6 literal bracket URLs
        assert_eq!(
            validate_url(b"http://[::1"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://::1]"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://[invalid_ipv6]"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://[::1]:"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://[::1]:65536"),
            Err(ValidationError::OutOfRange)
        );
        assert_eq!(
            validate_url(b"http://[::1]:abc"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://example.com:65536"),
            Err(ValidationError::OutOfRange)
        );
        assert_eq!(
            validate_url(b"http://example.com:abc"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_url(b"http://example.com:"),
            Err(ValidationError::InvalidChars)
        );
    }

    #[test]
    fn test_mac_address_validation() {
        // Valid colon-separated MAC (48-bit MAC & 64-bit EUI-64)
        assert_eq!(validate_mac_address(b"00:1A:2B:3C:4D:5E"), Ok(()));
        assert_eq!(validate_mac_address(b"ff:ff:ff:ff:ff:ff"), Ok(()));
        assert_eq!(validate_mac_address(b"01:23:45:67:89:ab"), Ok(()));
        assert_eq!(validate_mac_address(b"00:1A:2B:3C:4D:5E:6F:70"), Ok(()));

        // Valid hyphen-separated MAC (48-bit MAC & 64-bit EUI-64)
        assert_eq!(validate_mac_address(b"00-1A-2B-3C-4D-5E"), Ok(()));
        assert_eq!(validate_mac_address(b"FF-FF-FF-FF-FF-FF"), Ok(()));
        assert_eq!(validate_mac_address(b"00-1A-2B-3C-4D-5E-6F-70"), Ok(()));

        // NUL byte injection check
        assert_eq!(
            validate_mac_address(b"00:1A:2B\x003C:4D:5E"),
            Err(ValidationError::NullByte)
        );

        // ASCII control character injection check
        assert_eq!(
            validate_mac_address(b"00:1A:2B\n3C:4D:5E"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_mac_address(b"00:1A:2B\x1b3C:4D:5E"),
            Err(ValidationError::InvalidChars)
        );

        // Empty and length checks
        assert_eq!(validate_mac_address(b""), Err(ValidationError::EmptyInput));
        assert_eq!(
            validate_mac_address(b"00:1A:2B:3C:4D"),
            Err(ValidationError::OutOfRange)
        );
        assert_eq!(
            validate_mac_address(b"00:1A:2B:3C:4D:5E:6F"),
            Err(ValidationError::OutOfRange)
        );
        assert_eq!(
            validate_mac_address(b"00:1A:2B:3C:4D:5E:6F:70:81"),
            Err(ValidationError::TooLong)
        );

        // Mixed delimiters (rejected to prevent parser differential attacks)
        assert_eq!(
            validate_mac_address(b"00:1A-2B:3C-4D:5E"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_mac_address(b"00-1A:2B-3C:4D-5E"),
            Err(ValidationError::InvalidChars)
        );

        // Invalid delimiter character
        assert_eq!(
            validate_mac_address(b"00.1A.2B.3C.4D.5E"),
            Err(ValidationError::InvalidChars)
        );

        // Non-hex digits
        assert_eq!(
            validate_mac_address(b"00:1G:2B:3C:4D:5E"),
            Err(ValidationError::InvalidChars)
        );
        assert_eq!(
            validate_mac_address(b"00:1A:2B:3C:4D:5Z"),
            Err(ValidationError::InvalidChars)
        );
    }
}
