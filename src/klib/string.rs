#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]

pub use std::string::String;

use core::fmt;
use core::ops::{Deref, DerefMut};

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
    pub fn new() -> Self {
        Self {
            data: SigmaVec::new(),
            len: 0,
        }
    }

    pub fn from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        let mut data = SigmaVec::with_capacity(bytes.len());
        data.extend_from_slice(bytes);

        Self {
            data,
            len: bytes.len(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, core::str::Utf8Error> {
        core::str::from_utf8(bytes)?;
        let mut data = SigmaVec::with_capacity(bytes.len());
        data.extend_from_slice(bytes);

        Ok(Self {
            data,
            len: bytes.len(),
        })
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.data.as_slice()) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        let bytes = ch.encode_utf8(&mut buf);
        for byte in bytes.as_bytes() {
            self.data.push(*byte);
        }
        self.len += bytes.len();
    }

    pub fn push_str(&mut self, s: &str) {
        self.data.extend_from_slice(s.as_bytes());
        self.len += s.len();
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.len = 0;
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.len == 0 {
            return None;
        }

        let mut new_len = self.len - 1;
        while new_len > 0 && !self.is_char_boundary(new_len) {
            new_len -= 1;
        }

        let char_bytes = &self.data.as_slice()[new_len..self.len];
        let result = core::str::from_utf8(char_bytes)
            .ok()
            .and_then(|s| s.chars().next());

        self.len = new_len;
        self.data.truncate(new_len);

        result
    }

    pub fn remove(&mut self, idx: usize) -> char {
        let slice = self.as_str();
        let mut char_iter = slice.char_indices();
        let (byte_start, ch) = char_iter.nth(idx).expect("index out of bounds");
        let char_len = ch.len_utf8();
        let byte_end = byte_start + char_len;

        let count = self.len - byte_end;
        if count > 0 {
            unsafe {
                let ptr = self.data.as_mut_slice().as_mut_ptr();
                core::ptr::copy(ptr.add(byte_end), ptr.add(byte_start), count);
            }
        }

        self.len -= char_len;
        self.data.truncate(self.len);

        ch
    }

    pub fn insert(&mut self, idx: usize, ch: char) {
        let mut buf = [0u8; 4];
        let bytes = ch.encode_utf8(&mut buf);
        self.insert_bytes_at(idx, bytes.as_bytes());
    }

    pub fn insert_str(&mut self, idx: usize, s: &str) {
        self.insert_bytes_at(idx, s.as_bytes());
    }

    fn insert_bytes_at(&mut self, idx: usize, bytes: &[u8]) {
        let s_len = bytes.len();
        if s_len == 0 {
            return;
        }

        let old_len = self.len;
        let byte_idx = if idx == old_len {
            old_len
        } else {
            let slice = self.as_str();
            let mut char_iter = slice.char_indices();
            char_iter
                .nth(idx)
                .map_or(old_len, |(byte_start, _)| byte_start)
        };

        self.data.reserve(s_len);
        for &b in bytes {
            self.data.push(b);
        }

        let tail_count = old_len - byte_idx;
        unsafe {
            let ptr = self.data.as_mut_slice().as_mut_ptr();
            if tail_count > 0 {
                core::ptr::copy(ptr.add(byte_idx), ptr.add(byte_idx + s_len), tail_count);
            }
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.add(byte_idx), s_len);
        }

        self.len += s_len;
    }

    fn is_char_boundary(&self, idx: usize) -> bool {
        if idx == 0 || idx == self.len {
            return true;
        }

        let byte = self.data.as_slice()[idx];
        (byte as i8) >= -0x40
    }

    pub fn split_at(&self, mid: usize) -> (SigmaString, SigmaString) {
        let byte_mid = if mid == self.len() {
            self.len
        } else {
            let slice = self.as_str();
            let mut char_iter = slice.char_indices();
            char_iter
                .nth(mid)
                .map_or(self.len, |(byte_start, _)| byte_start)
        };

        let left = SigmaString::from_bytes(&self.data.as_slice()[..byte_mid]).unwrap();
        let right = SigmaString::from_bytes(&self.data.as_slice()[byte_mid..self.len]).unwrap();

        (left, right)
    }

    pub fn truncate(&mut self, new_len: usize) {
        assert!(new_len <= self.len);
        self.len = new_len;
        self.data.truncate(new_len);
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn trim_start(&self) -> SigmaString {
        let bytes = self.as_bytes();
        let mut start = 0;
        while start < self.len && bytes[start].is_ascii_whitespace() {
            start += 1;
        }
        SigmaString::from_bytes(&bytes[start..]).unwrap()
    }

    pub fn trim_end(&self) -> SigmaString {
        let bytes = self.as_bytes();
        let mut end = self.len;
        while end > 0 && bytes[end - 1].is_ascii_whitespace() {
            end -= 1;
        }
        SigmaString::from_bytes(&bytes[..end]).unwrap()
    }

    pub fn trim(&self) -> SigmaString {
        let bytes = self.as_bytes();
        let mut start = 0;
        while start < self.len && bytes[start].is_ascii_whitespace() {
            start += 1;
        }
        let mut end = self.len;
        while end > start && bytes[end - 1].is_ascii_whitespace() {
            end -= 1;
        }
        SigmaString::from_bytes(&bytes[start..end]).unwrap()
    }

    pub fn split<'a, P>(&'a self, pat: P) -> Split<'a, P>
    where
        P: Pattern,
    {
        Split {
            haystack: self.as_str(),
            pat,
            finished: false,
        }
    }

    pub fn contains<'a, P>(&'a self, pat: P) -> bool
    where
        P: Pattern,
    {
        pat.find_in(self).is_some()
    }

    pub fn into_bytes(self) -> SigmaVec<u8> {
        self.data
    }

    pub fn find<'a, P>(&'a self, pat: P) -> Option<usize>
    where
        P: Pattern,
    {
        pat.find_in(self)
    }

    pub fn replace<'a, P>(&'a self, pat: P, replacement: &str) -> SigmaString
    where
        P: Pattern,
    {
        let mut result = SigmaString::new();
        let mut last_end = 0;

        while let Some(start) = pat.find_in_from(self, last_end) {
            let end = start + pat.pattern_len();

            result.push_str(&self.as_str()[last_end..start]);
            result.push_str(replacement);

            last_end = end;
        }

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
        unsafe { core::str::from_utf8_unchecked_mut(self.data.as_mut_slice()) }
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
        &self.as_str()[index..index + 1]
    }
}

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

pub struct Split<'a, P> {
    haystack: &'a str,
    pat: P,
    finished: bool,
}

impl<'a, P> Iterator for Split<'a, P>
where
    P: Pattern,
{
    type Item = SigmaString;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let temp_string = SigmaString::from_str(self.haystack);
        if let Some(idx) = self.pat.find_in(&temp_string) {
            let end = idx + self.pat.pattern_len();
            let result = SigmaString::from_str(&self.haystack[..idx]);
            self.haystack = &self.haystack[end..];
            Some(result)
        } else {
            self.finished = true;
            Some(SigmaString::from_str(self.haystack))
        }
    }
}
