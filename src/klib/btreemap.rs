// SPDX-License-Identifier: MIT
//! Custom BTreeMap implementation for SigmaOS
//! Reduces dependency on std::collections::BTreeMap
//! Simple implementation using sorted Vec for now

use super::Vec;
use core::borrow::Borrow;

pub struct BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    entries: Vec<(K, V)>,
}

impl<K, V> Clone for BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    fn clone(&self) -> Self {
        BTreeMap {
            entries: self.entries.clone(),
        }
    }
}

impl<K, V> PartialEq for BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord,
    V: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}

impl<K, V> Eq for BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord + Eq,
    V: PartialEq + Clone + Eq,
{}

impl<K, V> core::fmt::Debug for BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord + core::fmt::Debug,
    V: Clone + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V> BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    pub fn new() -> Self {
        BTreeMap {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        // Find insertion point to maintain sorted order
        let mut insert_idx = self.entries.len();
        for (i, (k, _)) in self.entries.iter().enumerate() {
            if k == &key {
                // Update existing
                self.entries[i] = (key, value);
                return;
            }
            if k > &key {
                insert_idx = i;
                break;
            }
        }
        self.entries.insert(insert_idx, (key, value));
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + PartialEq,
    {
        for (k, v) in self.entries.iter() {
            let k_ref: &Q = k.borrow();
            if k_ref == key {
                return Some(v);
            }
        }
        None
    }

    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord + PartialEq,
    {
        for (k, v) in self.entries.iter_mut() {
            let k_ref: &Q = (k as &K).borrow();
            if k_ref == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + PartialEq,
    {
        for i in 0..self.entries.len() {
            let k_ref: &Q = self.entries[i].0.borrow();
            if k_ref == key {
                return Some(self.entries.remove(i).1);
            }
        }
        None
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + PartialEq,
    {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.entries.iter().map(|(_, v)| v)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.entries.iter_mut().map(|(_, v)| v)
    }

    pub fn iter(&self) -> BTreeMapIter<'_, K, V> {
        BTreeMapIter {
            entries: &self.entries,
            idx: 0,
        }
    }

    pub fn iter_mut(&mut self) -> BTreeMapIterMut<'_, K, V> {
        BTreeMapIterMut {
            entries: &mut self.entries,
            idx: 0,
        }
    }
}

pub struct BTreeMapIterMut<'a, K, V> {
    entries: &'a mut Vec<(K, V)>,
    idx: usize,
}

impl<'a, K, V> Iterator for BTreeMapIterMut<'a, K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.entries.len() {
            let ptr = self.entries.as_mut_ptr();
            unsafe {
                let item = &mut *ptr.add(self.idx);
                self.idx += 1;
                Some((&item.0, &mut item.1))
            }
        } else {
            None
        }
    }
}

impl<'a, K, V> IntoIterator for &'a mut BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = BTreeMapIterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V> Default for BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct BTreeMapIter<'a, K, V> {
    entries: &'a Vec<(K, V)>,
    idx: usize,
}

impl<'a, K, V> Iterator for BTreeMapIter<'a, K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.entries.len() {
            let item = (&self.entries[self.idx].0, &self.entries[self.idx].1);
            self.idx += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a, K, V> IntoIterator for &'a BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    type Item = (&'a K, &'a V);
    type IntoIter = BTreeMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btreemap_basic() {
        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.insert(3, "c");
        map.insert(2, "b");
        
        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&2), Some(&"b"));
        assert_eq!(map.get(&3), Some(&"c"));
    }

    #[test]
    fn test_btreemap_remove() {
        let mut map = BTreeMap::new();
        map.insert(1, "a");
        assert_eq!(map.remove(&1), Some("a"));
        assert_eq!(map.get(&1), None);
    }

    #[test]
    fn test_btreemap_iter() {
        let mut map = BTreeMap::new();
        map.insert(3, "c");
        map.insert(1, "a");
        map.insert(2, "b");
        
        let items: std::vec::Vec<(i32, &str)> = map.iter().map(|(&k, &v)| (k, v)).collect();
        assert_eq!(items, vec![(1, "a"), (2, "b"), (3, "c")]);
    }
}
