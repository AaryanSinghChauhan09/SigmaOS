// SPDX-License-Identifier: MIT
// Minimal Path and PathBuf for no_std klib

use core::ops::Deref;
use std::string::String;

pub type Path = PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathBuf {
    inner: String,
}

impl PathBuf {
    pub fn join(&self, path: &str) -> PathBuf {
        let mut p = self.clone();
        p.push(path);
        p
    }
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

    pub fn to_path_buf(&self) -> PathBuf {
        self.clone()
    }

    pub fn exists(&self) -> bool {
        !self.inner.is_empty()
    }
}

impl Default for PathBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for PathBuf {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::fmt::Display for PathBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<&str> for PathBuf {
    fn from(s: &str) -> Self {
        PathBuf {
            inner: String::from(s),
        }
    }
}

impl From<String> for PathBuf {
    fn from(s: String) -> Self {
        PathBuf { inner: s }
    }
}

impl AsRef<str> for PathBuf {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}
