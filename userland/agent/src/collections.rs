// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Custom collections implementation to avoid std::collections dependency
// Zero-allocation, performance-optimized data structures

/// Simple fixed-capacity map for key-value storage
/// Replaces BTreeMap with a more efficient, allocation-free alternative
pub struct SigmaMap<K, V, const N: usize> {
    entries: [(Option<K>, Option<V>); N],
    count: usize,
}

impl<K, V, const N: usize> SigmaMap<K, V, N>
where
    K: Copy + Eq,
    V: Copy,
{
    pub const fn new() -> Self {
        Self {
            entries: [(None, None); N],
            count: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.count >= N {
            return None; // Capacity exceeded
        }

        // Check if key already exists
        for i in 0..self.count {
            if let (Some(k), Some(v)) = (self.entries[i].0, self.entries[i].1) {
                if k == key {
                    let old = self.entries[i].1;
                    self.entries[i] = (Some(key), Some(value));
                    return old;
                }
            }
        }

        // Insert new entry
        self.entries[self.count] = (Some(key), Some(value));
        self.count += 1;
        None
    }

    pub fn get(&self, key: K) -> Option<V> {
        for i in 0..self.count {
            if let (Some(k), Some(v)) = (self.entries[i].0, self.entries[i].1) {
                if k == key {
                    return Some(v);
                }
            }
        }
        None
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        for i in 0..self.count {
            if let (Some(k), Some(v)) = (self.entries[i].0, self.entries[i].1) {
                if k == key {
                    let old = self.entries[i].1;
                    // Shift remaining entries
                    for j in i..self.count - 1 {
                        self.entries[j] = self.entries[j + 1];
                    }
                    self.entries[self.count - 1] = (None, None);
                    self.count -= 1;
                    return old;
                }
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn iter(&self) -> SigmaMapIter<K, V, N> {
        SigmaMapIter {
            map: self,
            index: 0,
        }
    }
}

pub struct SigmaMapIter<'a, K, V, const N: usize> {
    map: &'a SigmaMap<K, V, N>,
    index: usize,
}

impl<'a, K, V, const N: usize> Iterator for SigmaMapIter<'a, K, V, N>
where
    K: Copy,
    V: Copy,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.map.count {
            let entry = self.map.entries[self.index];
            self.index += 1;
            if let (Some(k), Some(v)) = entry {
                return Some((k, v));
            }
        }
        None
    }
}

/// Simple fixed-capacity vector for stack-like storage
/// Replaces Vec with allocation-free alternative
pub struct SigmaVec<T, const N: usize> {
    data: [Option<T>; N],
    count: usize,
}

impl<T, const N: usize> SigmaVec<T, N>
where
    T: Copy,
{
    pub const fn new() -> Self {
        Self {
            data: [None; N],
            count: 0,
        }
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.count >= N {
            return false; // Capacity exceeded
        }
        self.data[self.count] = Some(item);
        self.count += 1;
        true
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        let item = self.data[self.count];
        self.data[self.count] = None;
        item
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.count {
            return None;
        }
        self.data[index]
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn iter(&self) -> SigmaVecIter<T, N> {
        SigmaVecIter {
            vec: self,
            index: 0,
        }
    }
}

pub struct SigmaVecIter<'a, T, const N: usize> {
    vec: &'a SigmaVec<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for SigmaVecIter<'a, T, N>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.count {
            return None;
        }
        let item = self.vec.data[self.index];
        self.index += 1;
        item
    }
}

/// Simple string builder to avoid String allocations
pub struct SigmaStringBuilder<const N: usize> {
    buffer: [u8; N],
    length: usize,
}

impl<const N: usize> SigmaStringBuilder<N> {
    pub const fn new() -> Self {
        Self {
            buffer: [0; N],
            length: 0,
        }
    }

    pub fn push_str(&mut self, s: &str) -> bool {
        let bytes = s.as_bytes();
        if self.length + bytes.len() > N {
            return false;
        }
        for (i, &byte) in bytes.iter().enumerate() {
            self.buffer[self.length + i] = byte;
        }
        self.length += bytes.len();
        true
    }

    pub fn push_char(&mut self, c: char) -> bool {
        let mut buf = [0u8; 4];
        let len = c.encode_utf8(&mut buf).len();
        if self.length + len > N {
            return false;
        }
        for (i, &byte) in buf.iter().enumerate() {
            if i < len {
                self.buffer[self.length + i] = byte;
            }
        }
        self.length += len;
        true
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.buffer[..self.length])
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn clear(&mut self) {
        self.length = 0;
    }
}
