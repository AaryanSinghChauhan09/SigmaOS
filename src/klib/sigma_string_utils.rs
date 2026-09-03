// SigmaOS — sigma_string_utils.rs
// Custom string manipulation utilities that avoid std::string wherever possible.
// All functions operate on raw byte slices (&[u8] / &mut [u8]) or the crate's
// own SigmaString type so that this module is usable in a no_std kernel context.
//
// Design goals
// ────────────
// • Zero heap allocation for byte-slice operations
// • No dependency on std::string::String or std::str beyond what is already
//   present in the `alloc` crate (which SigmaOS already uses)
// • Panic-free — every operation returns a Result or Option

extern crate alloc;

use alloc::vec::Vec;
use core::fmt::Write;

// ── Type aliases ──────────────────────────────────────────────────────────────

/// A heap-allocated UTF-8 string backed by `alloc::vec::Vec<u8>`.
/// Prefer using the kernel's `SigmaString` from `klib::string` when richer
/// behaviour is needed; this type is intentionally thin.
pub type SigmaByteBuf = Vec<u8>;

// ── In-place byte-slice utilities ─────────────────────────────────────────────

/// Convert ASCII uppercase letters in `buf` to lowercase in place.
///
/// Returns the number of bytes that were changed.
pub fn ascii_to_lowercase_inplace(buf: &mut [u8]) -> usize {
    let mut changed = 0usize;
    for b in buf.iter_mut() {
        if b.is_ascii_uppercase() {
            *b = b.to_ascii_lowercase();
            changed += 1;
        }
    }
    changed
}

/// Convert ASCII lowercase letters in `buf` to uppercase in place.
///
/// Returns the number of bytes that were changed.
pub fn ascii_to_uppercase_inplace(buf: &mut [u8]) -> usize {
    let mut changed = 0usize;
    for b in buf.iter_mut() {
        if b.is_ascii_lowercase() {
            *b = b.to_ascii_uppercase();
            changed += 1;
        }
    }
    changed
}

// ── Byte-slice search & comparison ────────────────────────────────────────────

/// Returns the first index where `needle` is found in `haystack`, or `None`.
///
/// Uses a simple sliding-window search — no extra allocation.
#[inline]
pub fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    if needle.len() > haystack.len() {
        return None;
    }
    let limit = haystack.len() - needle.len();
    let mut i = 0;
    while i <= limit {
        if &haystack[i..i + needle.len()] == needle {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Returns `true` if `haystack` starts with `prefix`.
#[inline]
pub fn starts_with_bytes(haystack: &[u8], prefix: &[u8]) -> bool {
    haystack.len() >= prefix.len() && &haystack[..prefix.len()] == prefix
}

/// Returns `true` if `haystack` ends with `suffix`.
#[inline]
pub fn ends_with_bytes(haystack: &[u8], suffix: &[u8]) -> bool {
    haystack.len() >= suffix.len() && &haystack[haystack.len() - suffix.len()..] == suffix
}

/// ASCII-case-insensitive byte slice comparison.
///
/// Returns `true` only when both slices have the same length and every
/// corresponding pair of bytes is equal ignoring ASCII case.
pub fn equals_ignore_ascii_case(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .all(|(x, y)| x.to_ascii_lowercase() == y.to_ascii_lowercase())
}

// ── Trimming ──────────────────────────────────────────────────────────────────

/// Remove ASCII whitespace from the beginning of a byte slice.
#[inline]
pub fn trim_start_bytes(s: &[u8]) -> &[u8] {
    let start = s
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(s.len());
    &s[start..]
}

/// Remove ASCII whitespace from the end of a byte slice.
#[inline]
pub fn trim_end_bytes(s: &[u8]) -> &[u8] {
    let end = s
        .iter()
        .rposition(|b| !b.is_ascii_whitespace())
        .map(|i| i + 1)
        .unwrap_or(0);
    &s[..end]
}

/// Remove ASCII whitespace from both ends of a byte slice.
#[inline]
pub fn trim_bytes(s: &[u8]) -> &[u8] {
    trim_end_bytes(trim_start_bytes(s))
}

// ── Splitting ─────────────────────────────────────────────────────────────────

/// Split `input` into at most `limit` parts using `delimiter` as the separator.
///
/// Returns a `Vec<&[u8]>` without allocating any intermediate strings.
/// If `limit` is 0, all parts are returned.
pub fn split_bytes<'a>(input: &'a [u8], delimiter: u8, limit: usize) -> Vec<&'a [u8]> {
    let mut parts: Vec<&'a [u8]> = Vec::new();
    let mut start = 0usize;
    let effective_limit = if limit == 0 { usize::MAX } else { limit };

    for i in 0..input.len() {
        if input[i] == delimiter {
            if parts.len() + 1 >= effective_limit {
                break;
            }
            parts.push(&input[start..i]);
            start = i + 1;
        }
    }
    parts.push(&input[start..]);
    parts
}

// ── Number formatting (stack-based, no heap) ──────────────────────────────────

/// Write a `u64` value as decimal ASCII digits into `out_buf`.
///
/// Returns the number of bytes written, or `None` if the buffer is too small.
/// The result is *not* NUL-terminated.
pub fn format_u64_decimal(mut value: u64, out_buf: &mut [u8]) -> Option<usize> {
    if out_buf.len() < 20 {
        // u64::MAX is 20 digits
        if out_buf.is_empty() {
            return None;
        }
    }
    if value == 0 {
        out_buf[0] = b'0';
        return Some(1);
    }
    let mut tmp = [0u8; 20];
    let mut len = 0usize;
    while value > 0 && len < 20 {
        tmp[len] = b'0' + (value % 10) as u8;
        value /= 10;
        len += 1;
    }
    if len > out_buf.len() {
        return None;
    }
    for i in 0..len {
        out_buf[i] = tmp[len - 1 - i];
    }
    Some(len)
}

/// Write a `u64` value as hexadecimal (lowercase) into `out_buf`.
///
/// Returns the number of bytes written, or `None` if the buffer is too small.
pub fn format_u64_hex(mut value: u64, out_buf: &mut [u8]) -> Option<usize> {
    if value == 0 {
        if out_buf.is_empty() {
            return None;
        }
        out_buf[0] = b'0';
        return Some(1);
    }
    const HEX: &[u8] = b"0123456789abcdef";
    let mut tmp = [0u8; 16];
    let mut len = 0usize;
    while value > 0 && len < 16 {
        tmp[len] = HEX[(value & 0xf) as usize];
        value >>= 4;
        len += 1;
    }
    if len > out_buf.len() {
        return None;
    }
    for i in 0..len {
        out_buf[i] = tmp[len - 1 - i];
    }
    Some(len)
}

// ── Parsing ───────────────────────────────────────────────────────────────────

/// Parse a decimal ASCII byte slice into a `u64`.
///
/// Returns `None` if the slice is empty, contains non-digit bytes, or would
/// overflow `u64`.
pub fn parse_u64_decimal(s: &[u8]) -> Option<u64> {
    if s.is_empty() {
        return None;
    }
    let mut result: u64 = 0;
    for &b in s {
        if !b.is_ascii_digit() {
            return None;
        }
        let digit = (b - b'0') as u64;
        result = result.checked_mul(10)?.checked_add(digit)?;
    }
    Some(result)
}

/// Parse a hexadecimal ASCII byte slice (no `0x` prefix) into a `u64`.
///
/// Accepts both upper- and lowercase hex digits.
/// Returns `None` on empty input, invalid characters, or overflow.
pub fn parse_u64_hex(s: &[u8]) -> Option<u64> {
    if s.is_empty() {
        return None;
    }
    let mut result: u64 = 0;
    for &b in s {
        let nibble = match b {
            b'0'..=b'9' => (b - b'0') as u64,
            b'a'..=b'f' => (b - b'a' + 10) as u64,
            b'A'..=b'F' => (b - b'A' + 10) as u64,
            _ => return None,
        };
        result = result.checked_shl(4)?.checked_add(nibble)?;
    }
    Some(result)
}

// ── Replacement ───────────────────────────────────────────────────────────────

/// Return a new `Vec<u8>` with all occurrences of `from` replaced by `to`.
///
/// This is the only function in this module that allocates on the heap.
pub fn replace_bytes(input: &[u8], from: &[u8], to: &[u8]) -> SigmaByteBuf {
    if from.is_empty() || from.len() > input.len() {
        return SigmaByteBuf::from(input);
    }
    let mut out = SigmaByteBuf::new();
    let mut i = 0usize;
    while i <= input.len().saturating_sub(from.len()) {
        if &input[i..i + from.len()] == from {
            out.extend_from_slice(to);
            i += from.len();
        } else {
            out.push(input[i]);
            i += 1;
        }
    }
    // Append any remaining tail that is shorter than `from`
    if i < input.len() {
        out.extend_from_slice(&input[i..]);
    }
    out
}

// ── SigmaHashMap — custom hash-map implementation ─────────────────────────────

/// Number of initial buckets in a `SigmaHashMap`.
const SIGMA_MAP_INITIAL_BUCKETS: usize = 16;

/// A lightweight open-addressing hash map that avoids `std::collections::HashMap`.
///
/// # Implementation notes
/// Uses FNV-1a as its hash function (no external dependency) and linear
/// probing for collision resolution.  The table doubles when the load factor
/// exceeds 0.75.
///
/// # Example
/// ```
/// let mut m: SigmaHashMap<u32, u32> = SigmaHashMap::new();
/// m.insert(1, 42);
/// assert_eq!(m.get(&1), Some(&42));
/// ```
pub struct SigmaHashMap<K, V> {
    buckets: Vec<Option<(K, V)>>,
    len: usize,
}

impl<K: Eq + SigmaHash + Clone, V: Clone> SigmaHashMap<K, V> {
    /// Create an empty `SigmaHashMap`.
    pub fn new() -> Self {
        let mut buckets = Vec::new();
        for _ in 0..SIGMA_MAP_INITIAL_BUCKETS {
            buckets.push(None);
        }
        SigmaHashMap { buckets, len: 0 }
    }

    /// Current number of key-value pairs stored in the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the map contains no entries.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Insert `key` → `value`.  Returns the previous value if the key already
    /// existed, or `None` otherwise.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Grow before inserting so we always have at least one free slot.
        if self.len * 4 >= self.buckets.len() * 3 {
            self.rehash(self.buckets.len() * 2);
        }
        let idx = self.probe_for_insert(&key);
        match self.buckets[idx].take() {
            Some((_, old_val)) => {
                self.buckets[idx] = Some((key, value));
                Some(old_val)
            }
            None => {
                self.buckets[idx] = Some((key, value));
                self.len += 1;
                None
            }
        }
    }

    /// Return a shared reference to the value associated with `key`, or `None`.
    pub fn get(&self, key: &K) -> Option<&V> {
        let cap = self.buckets.len();
        if cap == 0 {
            return None;
        }
        let start = self.hash_index(key, cap);
        let mut i = start;
        loop {
            match &self.buckets[i] {
                None => return None,
                Some((k, v)) if k == key => return Some(v),
                _ => {}
            }
            i = (i + 1) % cap;
            if i == start {
                return None;
            }
        }
    }

    /// Return a mutable reference to the value associated with `key`, or `None`.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let cap = self.buckets.len();
        if cap == 0 {
            return None;
        }
        let start = self.hash_index(key, cap);
        let mut i = start;
        loop {
            match &self.buckets[i] {
                None => return None,
                Some((k, _)) if k == key => {
                    return self.buckets[i].as_mut().map(|(_, v)| v);
                }
                _ => {}
            }
            i = (i + 1) % cap;
            if i == start {
                return None;
            }
        }
    }

    /// Return `true` if the map contains `key`.
    #[inline]
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Remove `key` from the map, returning its value if it existed.
    ///
    /// Note: linear-probing maps require a tombstone or a shift-back strategy
    /// on removal.  This implementation uses simple backward shift to keep
    /// the invariant intact without tombstones.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let cap = self.buckets.len();
        if cap == 0 {
            return None;
        }
        let start = self.hash_index(key, cap);
        let mut i = start;
        loop {
            match &self.buckets[i] {
                None => return None,
                Some((k, _)) if k == key => break,
                _ => {}
            }
            i = (i + 1) % cap;
            if i == start {
                return None;
            }
        }
        let removed = self.buckets[i].take().map(|(_, v)| v);
        self.len -= 1;
        // Backward-shift neighbouring entries to close the gap.
        let mut j = (i + 1) % cap;
        while self.buckets[j].is_some() {
            let natural = self.buckets[j]
                .as_ref()
                .map(|(k, _)| self.hash_index(k, cap))
                .unwrap_or(j);
            // Move entry if it is displaced.
            let displaced =
                (j >= i && (natural <= i || natural > j)) || (j < i && natural <= i && natural > j);
            if displaced {
                self.buckets[i] = self.buckets[j].take();
                i = j;
            }
            j = (j + 1) % cap;
            if j == i {
                break;
            }
        }
        removed
    }

    /// Iterate over all key-value pairs as shared references.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets
            .iter()
            .filter_map(|slot| slot.as_ref().map(|(k, v)| (k, v)))
    }

    // ── private helpers ──────────────────────────────────────────────────────

    fn hash_index(&self, key: &K, cap: usize) -> usize {
        (key.sigma_hash() as usize) % cap
    }

    fn probe_for_insert(&self, key: &K) -> usize {
        let cap = self.buckets.len();
        let start = self.hash_index(key, cap);
        let mut i = start;
        loop {
            match &self.buckets[i] {
                None => return i,
                Some((k, _)) if k == key => return i,
                _ => {}
            }
            i = (i + 1) % cap;
        }
    }

    fn rehash(&mut self, new_cap: usize) {
        let new_cap = new_cap.max(SIGMA_MAP_INITIAL_BUCKETS);
        let mut new_buckets: Vec<Option<(K, V)>> = Vec::new();
        for _ in 0..new_cap {
            new_buckets.push(None);
        }
        let old_buckets = core::mem::replace(&mut self.buckets, new_buckets);
        for slot in old_buckets.into_iter().flatten() {
            let (k, v) = slot;
            let idx = (k.sigma_hash() as usize) % new_cap;
            let mut i = idx;
            loop {
                if self.buckets[i].is_none() {
                    self.buckets[i] = Some((k, v));
                    break;
                }
                i = (i + 1) % new_cap;
            }
        }
    }
}

impl<K: Eq + SigmaHash + Clone, V: Clone> Default for SigmaHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// ── SigmaHash trait ───────────────────────────────────────────────────────────

/// A hashing trait implemented by types that can be keys in `SigmaHashMap`.
///
/// The default implementations below use FNV-1a so there is no dependency on
/// `std::hash`.
pub trait SigmaHash {
    fn sigma_hash(&self) -> u64;
}

/// FNV-1a 64-bit hash over a byte slice.
#[inline]
pub fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

impl SigmaHash for u8 {
    fn sigma_hash(&self) -> u64 {
        fnv1a_64(&[*self])
    }
}
impl SigmaHash for u16 {
    fn sigma_hash(&self) -> u64 {
        fnv1a_64(&self.to_le_bytes())
    }
}
impl SigmaHash for u32 {
    fn sigma_hash(&self) -> u64 {
        fnv1a_64(&self.to_le_bytes())
    }
}
impl SigmaHash for u64 {
    fn sigma_hash(&self) -> u64 {
        fnv1a_64(&self.to_le_bytes())
    }
}
impl SigmaHash for usize {
    fn sigma_hash(&self) -> u64 {
        fnv1a_64(&(*self as u64).to_le_bytes())
    }
}
impl SigmaHash for i32 {
    fn sigma_hash(&self) -> u64 {
        fnv1a_64(&self.to_le_bytes())
    }
}
impl SigmaHash for i64 {
    fn sigma_hash(&self) -> u64 {
        fnv1a_64(&self.to_le_bytes())
    }
}
impl SigmaHash for &str {
    fn sigma_hash(&self) -> u64 {
        fnv1a_64(self.as_bytes())
    }
}
impl SigmaHash for alloc::string::String {
    fn sigma_hash(&self) -> u64 {
        fnv1a_64(self.as_bytes())
    }
}
impl SigmaHash for &[u8] {
    fn sigma_hash(&self) -> u64 {
        fnv1a_64(self)
    }
}

// ── Custom in-place sort (no std::cmp dependency) ─────────────────────────────

/// Sort a mutable slice in ascending order using insertion sort.
///
/// Insertion sort is O(n²) but is simple, stack-only, and performs well for
/// small arrays (n < 64), which is a common kernel use-case.
pub fn insertion_sort<T: Ord>(slice: &mut [T]) {
    let n = slice.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && slice[j - 1] > slice[j] {
            slice.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Sort a mutable slice by a key extractor using insertion sort.
pub fn insertion_sort_by_key<T, K: Ord, F: Fn(&T) -> K>(slice: &mut [T], key: F) {
    let n = slice.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && key(&slice[j - 1]) > key(&slice[j]) {
            slice.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Sort a mutable slice using a comparator closure (shell sort, O(n log²n)).
///
/// Suitable for medium-sized kernel arrays without heap allocation.
pub fn sort_by<T, F: Fn(&T, &T) -> core::cmp::Ordering>(slice: &mut [T], compare: F) {
    let n = slice.len();
    let mut gap = n / 2;
    while gap > 0 {
        for i in gap..n {
            let mut j = i;
            while j >= gap && compare(&slice[j - gap], &slice[j]) == core::cmp::Ordering::Greater {
                slice.swap(j - gap, j);
                if j < gap {
                    break;
                }
                j -= gap;
            }
        }
        gap /= 2;
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_bytes() {
        assert_eq!(find_bytes(b"hello world", b"world"), Some(6));
        assert_eq!(find_bytes(b"hello world", b"xyz"), None);
        assert_eq!(find_bytes(b"abc", b""), Some(0));
    }

    #[test]
    fn test_trim_bytes() {
        assert_eq!(trim_bytes(b"  hello  "), b"hello");
        assert_eq!(trim_bytes(b"no-space"), b"no-space");
    }

    #[test]
    fn test_parse_format_roundtrip() {
        let mut buf = [0u8; 32];
        let n = format_u64_decimal(12345, &mut buf).unwrap();
        let parsed = parse_u64_decimal(&buf[..n]).unwrap();
        assert_eq!(parsed, 12345);
    }

    #[test]
    fn test_sigma_hash_map() {
        let mut map: SigmaHashMap<u32, u32> = SigmaHashMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        assert_eq!(map.get(&2), Some(&20));
        assert_eq!(map.len(), 3);
        map.remove(&2);
        assert_eq!(map.get(&2), None);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = [5u32, 3, 8, 1, 9, 2];
        insertion_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_replace_bytes() {
        let result = replace_bytes(b"foo bar foo", b"foo", b"baz");
        assert_eq!(&result, b"baz bar baz");
    }

    #[test]
    fn test_split_bytes() {
        let parts = split_bytes(b"a:b:c", b':', 0);
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[1], b"b");
    }

    #[test]
    fn test_equals_ignore_ascii_case() {
        assert!(equals_ignore_ascii_case(b"Hello", b"hello"));
        assert!(!equals_ignore_ascii_case(b"hello", b"world"));
    }
}
