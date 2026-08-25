//! Custom string implementation for SigmaOS
//! This module provides no_std alternatives to std::string and reduces dependency on predefined functions

use core::ops::{Deref, DerefMut};
use core::slice;
use core::str;
use core::fmt;
use super::vec::SigmaVec;

/// Custom string type for SigmaOS with reduced dependency on predefined functions
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SigmaString {
    data: SigmaVec<u8>,
    len: usize,
}

impl fmt::Debug for SigmaString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SigmaString({:?})", self.as_str())
    }
}

impl fmt::Display for SigmaString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl SigmaString {
    /// Create a new empty SigmaString
    pub fn new() -> Self {
        Self {
            data: SigmaVec::new(),
            len: 0,
        }
    }
    
    /// Create a SigmaString from a string slice
    pub fn from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        let mut data = SigmaVec::with_capacity(bytes.len());
        for &byte in bytes {
            data.push(byte);
        }
        
        Self {
            data,
            len: bytes.len(),
        }
    }
    
    /// Create a SigmaString from a byte slice
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, str::Utf8Error> {
        str::from_utf8(bytes)?;
        let mut data = SigmaVec::with_capacity(bytes.len());
        for &byte in bytes {
            data.push(byte);
        }
        
        Ok(Self {
            data,
            len: bytes.len(),
        })
    }
    
    /// Convert to string slice
    pub fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(self.data.as_slice())
        }
    }
    
    /// Convert to byte slice
    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_slice()
    }
    
    /// Get the length of the string
    pub fn len(&self) -> usize {
        self.len
    }
    
    /// Check if the string is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    /// Push a character to the string
    pub fn push(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        let bytes = ch.encode_utf8(&mut buf);
        for byte in bytes.as_bytes() {
            self.data.push(*byte);
        }
        self.len += bytes.len();
    }
    
    /// Push a string slice to the string
    pub fn push_str(&mut self, s: &str) {
        for byte in s.as_bytes() {
            self.data.push(*byte);
        }
        self.len += s.len();
    }
    
    /// Clear the string
    pub fn clear(&mut self) {
        self.data.clear();
        self.len = 0;
    }
    
    /// Remove the last character
    pub fn pop(&mut self) -> Option<char> {
        if self.len == 0 {
            return None;
        }
        
        // Find the last character boundary
        let mut new_len = self.len - 1;
        while new_len > 0 && !self.is_char_boundary(new_len) {
            new_len -= 1;
        }
        
        let char_bytes = &self.data.as_slice()[new_len..self.len];
        let result = str::from_utf8(char_bytes).ok().and_then(|s| s.chars().next());
        
        self.len = new_len;
        self.data.truncate(new_len);
        
        result
    }
    
    /// Remove a character at a specific position
    pub fn remove(&mut self, idx: usize) -> char {
        let ch = self[idx];
        
        // Remove the character bytes
        let char_len = ch.len_utf8();
        let start = idx;
        let end = idx + char_len;
        
        for i in end..self.len {
            self.data.as_mut_slice()[start + (i - end)] = self.data.as_slice()[i];
        }
        
        self.len -= char_len;
        self.data.truncate(self.len);
        
        ch
    }
    
    /// Insert a character at a specific position
    pub fn insert(&mut self, idx: usize, ch: char) {
        assert!(idx <= self.len);
        
        let mut buf = [0u8; 4];
        let bytes = ch.encode_utf8(&mut buf);
        let char_len = bytes.len();
        
        // Make space for the new character
        for i in (idx..self.len).rev() {
            self.data.as_mut_slice()[i + char_len] = self.data.as_slice()[i];
        }
        
        // Insert the character bytes
        for (i, &byte) in bytes.as_bytes().iter().enumerate() {
            self.data.as_mut_slice()[idx + i] = byte;
        }
        
        self.len += char_len;
    }
    
    /// Insert a string slice at a specific position
    pub fn insert_str(&mut self, idx: usize, s: &str) {
        assert!(idx <= self.len);
        
        let s_len = s.len();
        
        // Make space for the new string
        for i in (idx..self.len).rev() {
            self.data.as_mut_slice()[i + s_len] = self.data.as_slice()[i];
        }
        
        // Insert the string bytes
        for (i, &byte) in s.as_bytes().iter().enumerate() {
            self.data.as_mut_slice()[idx + i] = byte;
        }
        
        self.len += s_len;
    }
    
    /// Check if a position is a character boundary
    fn is_char_boundary(&self, idx: usize) -> bool {
        if idx == 0 || idx == self.len {
            return true;
        }
        
        let byte = self.data.as_slice()[idx];
        (byte as i8) >= -0x40
    }
    
    /// Split the string at a position
    pub fn split_at(&self, mid: usize) -> (SigmaString, SigmaString) {
        assert!(mid <= self.len);
        
        let left = SigmaString::from_bytes(&self.data.as_slice()[..mid]).unwrap();
        let right = SigmaString::from_bytes(&self.data.as_slice()[mid..self.len]).unwrap();
        
        (left, right)
    }
    
    /// Truncate the string to a new length
    pub fn truncate(&mut self, new_len: usize) {
        assert!(new_len <= self.len);
        self.len = new_len;
        self.data.truncate(new_len);
    }
    
    /// Reserve capacity for additional bytes
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
    
    /// Get the capacity of the string
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    
    /// Trim leading whitespace
    pub fn trim_start(&self) -> SigmaString {
        let bytes = self.as_bytes();
        let mut start = 0;
        while start < self.len && bytes[start].is_ascii_whitespace() {
            start += 1;
        }
        SigmaString::from_bytes(&bytes[start..]).unwrap()
    }
    
    /// Trim trailing whitespace
    pub fn trim_end(&self) -> SigmaString {
        let bytes = self.as_bytes();
        let mut end = self.len;
        while end > 0 && bytes[end - 1].is_ascii_whitespace() {
            end -= 1;
        }
        SigmaString::from_bytes(&bytes[..end]).unwrap()
    }
    
    /// Trim both leading and trailing whitespace
    pub fn trim(&self) -> SigmaString {
        self.trim_start().trim_end()
    }
    
    /// Split the string by a pattern
    pub fn split<'a, P>(&'a self, pat: P) -> Split<'a, P>
    where
        P: Pattern,
    {
        Split {
            string: self,
            pat,
        }
    }
    
    /// Check if the string contains a pattern
    pub fn contains<'a, P>(&'a self, pat: P) -> bool
    where
        P: Pattern,
    {
        pat.find_in(self).is_some()
    }

    /// Convert to bytes
    pub fn into_bytes(self) -> SigmaVec<u8> {
        self.data
    }
    
    /// Find the first occurrence of a pattern
    pub fn find<'a, P>(&'a self, pat: P) -> Option<usize>
    where
        P: Pattern,
    {
        pat.find_in(self)
    }
    
    /// Replace occurrences of a pattern
    pub fn replace<'a, P>(&'a self, pat: P, replacement: &str) -> SigmaString
    where
        P: Pattern,
    {
        let mut result = SigmaString::new();
        let mut last_end = 0;
        
        while let Some(start) = pat.find_in_from(self, last_end) {
            let end = start + pat.pattern_len();
            
            // Add the part before the match
            result.push_str(&self.as_str()[last_end..start]);
            
            // Add the replacement
            result.push_str(replacement);
            
            last_end = end;
        }
        
        // Add the remaining part
        result.push_str(&self.as_str()[last_end..]);
        
        result
    }
}

impl Default for SigmaString {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for SigmaString {
    type Target = str;
    
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl DerefMut for SigmaString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            str::from_utf8_unchecked_mut(self.data.as_mut_slice())
        }
    }
}

impl From<&str> for SigmaString {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for SigmaString {
    fn from(s: String) -> Self {
        Self::from_str(&s)
    }
}

impl From<SigmaString> for String {
    fn from(s: SigmaString) -> Self {
        s.as_str().to_string()
    }
}

impl core::ops::Index<usize> for SigmaString {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        let bytes = self.as_bytes();
        if index >= bytes.len() {
            panic!("index out of bounds");
        }
        // Return single character at position
        &self.as_str()[index..index + 1]
    }
}

/// Pattern trait for string operations
pub trait Pattern {
    fn find_in(&self, haystack: &SigmaString) -> Option<usize>;
    fn find_in_from(&self, haystack: &SigmaString, start: usize) -> Option<usize>;
    fn pattern_len(&self) -> usize;
}

impl Pattern for char {
    fn find_in(&self, haystack: &SigmaString) -> Option<usize> {
        haystack.as_str().find(*self)
    }
    
    fn find_in_from(&self, haystack: &SigmaString, start: usize) -> Option<usize> {
        haystack.as_str()[start..].find(*self).map(|i| start + i)
    }
    
    fn pattern_len(&self) -> usize {
        self.len_utf8()
    }
}

impl Pattern for &str {
    fn find_in(&self, haystack: &SigmaString) -> Option<usize> {
        haystack.as_str().find(*self)
    }
    
    fn find_in_from(&self, haystack: &SigmaString, start: usize) -> Option<usize> {
        haystack.as_str()[start..].find(*self).map(|i| start + i)
    }
    
    fn pattern_len(&self) -> usize {
        self.len()
    }
}

/// Split iterator for SigmaString
pub struct Split<'a, P> {
    string: &'a SigmaString,
    pat: P,
}

impl<'a, P> Iterator for Split<'a, P>
where
    P: Pattern,
{
    type Item = SigmaString;
    
    fn next(&mut self) -> Option<Self::Item> {
        let haystack = self.string.as_str();
        let start = 0;
        
        if let Some(idx) = self.pat.find_in(self.string) {
            let end = idx + self.pat.pattern_len();
            let result = SigmaString::from_str(&haystack[start..idx]);
            self.string = &SigmaString::from_str(&haystack[end..]);
            Some(result)
        } else {
            let result = SigmaString::from_str(haystack);
            self.string = &SigmaString::new();
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_creation() {
        let s = SigmaString::new();
        assert!(s.is_empty());
        
        let s = SigmaString::from_str("hello");
        assert_eq!(s.as_str(), "hello");
        assert_eq!(s.len(), 5);
    }
    
    #[test]
    fn test_string_push() {
        let mut s = SigmaString::new();
        s.push('h');
        s.push('e');
        s.push('l');
        s.push('l');
        s.push('o');
        assert_eq!(s.as_str(), "hello");
        
        s.push_str(" world");
        assert_eq!(s.as_str(), "hello world");
    }
    
    #[test]
    fn test_string_trim() {
        let s = SigmaString::from_str("  hello  ");
        assert_eq!(s.trim().as_str(), "hello");
        
        let s = SigmaString::from_str("  hello");
        assert_eq!(s.trim().as_str(), "hello");
        
        let s = SigmaString::from_str("hello  ");
        assert_eq!(s.trim().as_str(), "hello");
    }
    
    #[test]
    fn test_string_split() {
        let s = SigmaString::from_str("hello world");
        let parts: Vec<SigmaString> = s.split(' ').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].as_str(), "hello");
        assert_eq!(parts[1].as_str(), "world");
    }
    
    #[test]
    fn test_string_contains() {
        let s = SigmaString::from_str("hello world");
        assert!(s.contains("hello"));
        assert!(s.contains("world"));
        assert!(!s.contains("foo"));
    }
    
    #[test]
    fn test_string_replace() {
        let s = SigmaString::from_str("hello world");
        let replaced = s.replace("world", "sigma");
        assert_eq!(replaced.as_str(), "hello sigma");
    }
}
