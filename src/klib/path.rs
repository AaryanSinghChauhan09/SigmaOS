// SPDX-License-Identifier: MIT
//! Minimal Path and PathBuf for no_std klib

extern crate alloc;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathBuf {
    inner: String,
}

impl PathBuf {
    pub fn new() -> Self {
        PathBuf {
            inner: String::new(),
        }
    }

    pub fn from(s: &str) -> Self {
        PathBuf {
            inner: String::from(s),
        }
    }

    pub fn push(&mut self, path: &str) {
        if !self.inner.is_empty() && !self.inner.ends_with('/') {
            self.inner.push('/');
        }
        self.inner.push_str(path);
    }

    pub fn to_str(&self) -> Option<&str> {
        Some(&self.inner)
    }
}

impl Default for PathBuf {
    fn default() -> Self {
        Self::new()
    }
}
