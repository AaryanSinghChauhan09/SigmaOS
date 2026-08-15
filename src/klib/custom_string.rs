#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Custom String Type
// A heap-backed UTF-8 string that does NOT depend on std::string::String or alloc::string::String.
// Uses raw pointer management via the global SIGMA_ALLOCATOR.

#[allow(dead_code)]

use core::alloc::Layout;
use core::fmt;
use core::ops::{Deref, DerefMut, Index};
use core::ptr;
use core::slice;
use core::str;

// ============================================================================
// SigmaString — core custom string type
// ============================================================================

/// A heap-managed UTF-8 string independent of the standard library.
///
/// Internal representation:
///   - `ptr`:  raw pointer to UTF-8 bytes on the heap
///   - `len`:  number of valid bytes (not including null terminator)
///   - `cap`:  allocated capacity in bytes
///
/// Invariants:
///   - `ptr` is always a valid allocation of at least `cap` bytes when `cap > 0`
///   - bytes `[0, len)` are always valid UTF-8
///   - `len <= cap`
pub struct SigmaString {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

// SAFETY: SigmaString owns its heap buffer; no aliasing possible across threads.
unsafe impl Send for SigmaString {}
unsafe impl Sync for SigmaString {}

impl Clone for SigmaString {
    fn clone(&self) -> Self {
        Self::from_str(self.as_str())
    }
}

impl SigmaString {
    // ------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------

    /// Create an empty string with zero capacity.
    pub const fn empty() -> Self {
        SigmaString { ptr: ptr::null_mut(), len: 0, cap: 0 }
    }

    /// Create a new string with the given initial capacity.
    pub fn with_capacity(cap: usize) -> Self {
        if cap == 0 {
            return Self::empty();
        }
        let ptr = unsafe { alloc_bytes(cap) };
        if ptr.is_null() {
            return Self::empty();
        }
        SigmaString { ptr, len: 0, cap }
    }

    /// Create a SigmaString from a `&str` slice.
    pub fn from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        let len = bytes.len();
        if len == 0 {
            return Self::empty();
        }
        let ptr = unsafe { alloc_bytes(len) };
        if ptr.is_null() {
            return Self::empty();
        }
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, len) };
        SigmaString { ptr, len, cap: len }
    }

    /// Create a SigmaString from raw UTF-8 bytes (checked).
    pub fn from_utf8(bytes: &[u8]) -> Result<Self, Utf8Error> {
        core::str::from_utf8(bytes).map_err(|_| Utf8Error)?;
        Ok(Self::from_str(unsafe { core::str::from_utf8_unchecked(bytes) }))
    }

    /// Create a SigmaString from raw UTF-8 bytes without checking.
    ///
    /// # Safety
    /// `bytes` must be valid UTF-8.
    pub unsafe fn from_utf8_unchecked(bytes: &[u8]) -> Self {
        Self::from_str(core::str::from_utf8_unchecked(bytes))
    }

    // ------------------------------------------------------------------
    // Capacity management
    // ------------------------------------------------------------------

    /// Current length in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the string is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Current allocated capacity.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Ensure at least `additional` more bytes can be stored without reallocating.
    pub fn reserve(&mut self, additional: usize) {
        let needed = self.len.saturating_add(additional);
        if needed <= self.cap {
            return;
        }
        // Growth factor: double, at minimum grow to needed.
        let new_cap = (self.cap * 2).max(needed).max(8);
        self.grow(new_cap);
    }

    fn grow(&mut self, new_cap: usize) {
        let new_ptr = unsafe { alloc_bytes(new_cap) };
        if new_ptr.is_null() {
            // OOM — leave the string unchanged
            return;
        }
        if !self.ptr.is_null() && self.len > 0 {
            unsafe { ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len) };
        }
        if !self.ptr.is_null() && self.cap > 0 {
            unsafe { dealloc_bytes(self.ptr, self.cap) };
        }
        self.ptr = new_ptr;
        self.cap = new_cap;
    }

    // ------------------------------------------------------------------
    // Mutation
    // ------------------------------------------------------------------

    /// Append a `&str` to this string.
    pub fn push_str(&mut self, s: &str) {
        let bytes = s.as_bytes();
        if bytes.is_empty() {
            return;
        }
        self.reserve(bytes.len());
        if self.ptr.is_null() {
            return;
        }
        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr.add(self.len), bytes.len());
        }
        self.len += bytes.len();
    }

    /// Append a single `char`.
    pub fn push(&mut self, c: char) {
        let mut buf = [0u8; 4];
        let s = c.encode_utf8(&mut buf);
        self.push_str(s);
    }

    /// Truncate to `new_len` bytes. Panics if `new_len` is not on a char boundary.
    pub fn truncate(&mut self, new_len: usize) {
        if new_len < self.len {
            assert!(self.as_str().is_char_boundary(new_len));
            self.len = new_len;
        }
    }

    /// Clear the string (set length to 0, keep allocation).
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Remove and return the last character.
    pub fn pop(&mut self) -> Option<char> {
        let s = self.as_str();
        let ch = s.chars().next_back()?;
        self.len -= ch.len_utf8();
        Some(ch)
    }

    /// Concatenate two SigmaStrings.
    pub fn concat(mut self, other: &SigmaString) -> SigmaString {
        self.push_str(other.as_str());
        self
    }

    // ------------------------------------------------------------------
    // Querying
    // ------------------------------------------------------------------

    /// View as a `&str`.
    pub fn as_str(&self) -> &str {
        if self.ptr.is_null() || self.len == 0 {
            return "";
        }
        unsafe {
            let slice = slice::from_raw_parts(self.ptr, self.len);
            core::str::from_utf8_unchecked(slice)
        }
    }

    /// View as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        if self.ptr.is_null() || self.len == 0 {
            return &[];
        }
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }

    /// Check if this string contains a substring.
    pub fn contains(&self, needle: &str) -> bool {
        self.as_str().contains(needle)
    }

    /// Check if this string starts with a prefix.
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.as_str().starts_with(prefix)
    }

    /// Check if this string ends with a suffix.
    pub fn ends_with(&self, suffix: &str) -> bool {
        self.as_str().ends_with(suffix)
    }

    /// Find the byte index of the first occurrence of `needle`, or None.
    pub fn find(&self, needle: &str) -> Option<usize> {
        self.as_str().find(needle)
    }

    /// Return a trimmed view of the string.
    pub fn trim(&self) -> &str {
        self.as_str().trim()
    }

    // ------------------------------------------------------------------
    // Conversion / cloning
    // ------------------------------------------------------------------

    /// Clone this string into a new allocation.
    pub fn clone(&self) -> SigmaString {
        SigmaString::from_str(self.as_str())
    }

    /// Convert to a null-terminated C string view (allocated separately).
    pub fn to_cstring(&self) -> CStringView {
        let len = self.len + 1;
        let ptr = unsafe { alloc_bytes(len) };
        if ptr.is_null() {
            return CStringView { ptr: ptr::null(), len: 0 };
        }
        unsafe {
            if self.len > 0 {
                ptr::copy_nonoverlapping(self.ptr, ptr, self.len);
            }
            *ptr.add(self.len) = 0;
        }
        CStringView { ptr, len: self.len }
    }
}

// ------------------------------------------------------------------
// Drop
// ------------------------------------------------------------------

impl Drop for SigmaString {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.cap > 0 {
            unsafe { dealloc_bytes(self.ptr, self.cap) };
            self.ptr = ptr::null_mut();
        }
    }
}

// ------------------------------------------------------------------
// Deref to &str
// ------------------------------------------------------------------

impl Deref for SigmaString {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

// ------------------------------------------------------------------
// fmt::Display / fmt::Debug
// ------------------------------------------------------------------

impl fmt::Display for SigmaString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Debug for SigmaString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SigmaString({:?})", self.as_str())
    }
}

// ------------------------------------------------------------------
// PartialEq, Eq
// ------------------------------------------------------------------

impl PartialEq for SigmaString {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for SigmaString {}

impl core::hash::Hash for SigmaString {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl PartialEq<str> for SigmaString {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for SigmaString {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

// ------------------------------------------------------------------
// From / Into conversions
// ------------------------------------------------------------------

impl From<&str> for SigmaString {
    fn from(s: &str) -> Self {
        SigmaString::from_str(s)
    }
}

// ============================================================================
// SigmaStringBuilder — an incremental string builder
// ============================================================================

pub struct SigmaStringBuilder {
    inner: SigmaString,
}

impl SigmaStringBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SigmaStringBuilder { inner: SigmaString::with_capacity(64) }
    }

    pub fn with_capacity(cap: usize) -> Self {
        SigmaStringBuilder { inner: SigmaString::with_capacity(cap) }
    }

    pub fn append(mut self, s: &str) -> Self {
        self.inner.push_str(s);
        self
    }

    pub fn append_char(mut self, c: char) -> Self {
        self.inner.push(c);
        self
    }

    pub fn build(self) -> SigmaString {
        self.inner
    }
}

// ============================================================================
// Null-terminated C string view
// ============================================================================

pub struct CStringView {
    ptr: *const u8,
    len: usize,
}

impl CStringView {
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for CStringView {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.len > 0 {
            unsafe { dealloc_bytes(self.ptr as *mut u8, self.len + 1) };
        }
    }
}

// ============================================================================
// Error types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Utf8Error;

impl fmt::Display for Utf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid UTF-8 sequence")
    }
}

// ============================================================================
// Internal allocator helpers
// ============================================================================

/// Allocate `size` bytes with 1-byte alignment via the global allocator.
unsafe fn alloc_bytes(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, 1).expect("invalid layout");
    core::alloc::GlobalAlloc::alloc(&crate::klib::custom_allocator::SIGMA_ALLOCATOR, layout)
}

/// Deallocate `size` bytes at `ptr`.
unsafe fn dealloc_bytes(ptr: *mut u8, size: usize) {
    let layout = Layout::from_size_align(size, 1).expect("invalid layout");
    core::alloc::GlobalAlloc::dealloc(&crate::klib::custom_allocator::SIGMA_ALLOCATOR, ptr, layout)
}

// ============================================================================
// Unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let s = SigmaString::empty();
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
        assert_eq!(s.as_str(), "");
    }

    #[test]
    fn test_from_str() {
        let s = SigmaString::from_str("hello");
        assert_eq!(s.len(), 5);
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_push_str() {
        let mut s = SigmaString::from_str("hello");
        s.push_str(", world");
        assert_eq!(s.as_str(), "hello, world");
    }

    #[test]
    fn test_push_char() {
        let mut s = SigmaString::from_str("hi");
        s.push('!');
        assert_eq!(s.as_str(), "hi!");
    }

    #[test]
    fn test_pop() {
        let mut s = SigmaString::from_str("abc");
        assert_eq!(s.pop(), Some('c'));
        assert_eq!(s.as_str(), "ab");
    }

    #[test]
    fn test_clear() {
        let mut s = SigmaString::from_str("test");
        s.clear();
        assert!(s.is_empty());
    }

    #[test]
    fn test_contains_and_find() {
        let s = SigmaString::from_str("SigmaOS rocks");
        assert!(s.contains("OS"));
        assert_eq!(s.find("OS"), Some(5));
        assert!(!s.contains("Linux"));
    }

    #[test]
    fn test_equality() {
        let a = SigmaString::from_str("sigma");
        let b = SigmaString::from_str("sigma");
        assert_eq!(a, b);
        assert_eq!(a, "sigma");
    }

    #[test]
    fn test_builder() {
        let s = SigmaStringBuilder::new()
            .append("Sigma")
            .append("OS")
            .append_char(' ')
            .append("v1.0")
            .build();
        assert_eq!(s.as_str(), "SigmaOS v1.0");
    }

    #[test]
    fn test_from_utf8_invalid() {
        let invalid = &[0xff, 0xfe];
        assert!(SigmaString::from_utf8(invalid).is_err());
    }

    #[test]
    fn test_concat() {
        let a = SigmaString::from_str("hello ");
        let b = SigmaString::from_str("world");
        let c = a.concat(&b);
        assert_eq!(c.as_str(), "hello world");
    }
}
