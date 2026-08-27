//! Custom BTreeMap implementation for SigmaOS
//! Reduces dependency on std::collections::BTreeMap
//! Simple implementation using sorted Vec for now

use super::Vec;
use core::fmt;
use core::cmp::PartialEq;

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
        K: core::borrow::Borrow<Q>,
        Q: PartialEq,
    {
        for (k, v) in self.entries.iter() {
            if k.borrow() == key {
                return Some(v);
            }
        }
        None
    }

    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: core::borrow::Borrow<Q>,
        Q: PartialEq,
    {
        for (k, v) in self.entries.iter_mut() {
            if (k as &K).borrow() == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: core::borrow::Borrow<Q>,
        Q: PartialEq,
    {
        for i in 0..self.entries.len() {
            if self.entries[i].0.borrow() == key {
                return Some(self.entries.remove(i).1);
            }
        }
        None
    }

    pub fn remove_str(&mut self, key: &str) -> Option<V>
    where
        K: core::convert::AsRef<str>,
    {
        for i in 0..self.entries.len() {
            if self.entries[i].0.as_ref() == key {
                return Some(self.entries.remove(i).1);
            }
        }
        None
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: core::borrow::Borrow<Q>,
        Q: PartialEq,
    {
        self.get(key).is_some()
    }

    pub fn contains_key_str(&self, key: &str) -> bool
    where
        K: core::convert::AsRef<str>,
    {
        for (k, _) in self.entries.iter() {
            if k.as_ref() == key {
                return true;
            }
        }
        false
    }

    pub fn get_str(&self, key: &str) -> Option<&V>
    where
        K: core::convert::AsRef<str>,
    {
        for (k, v) in self.entries.iter() {
            if k.as_ref() == key {
                return Some(v);
            }
        }
        None
    }

    pub fn get_mut_str(&mut self, key: &str) -> Option<&mut V>
    where
        K: core::convert::AsRef<str>,
    {
        for (k, v) in self.entries.iter_mut() {
            if k.as_ref() == key {
                return Some(v);
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> BTreeMapIter<'_, K, V> {
        BTreeMapIter {
            entries: &self.entries,
            idx: 0,
        }
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut {
            entries: &mut self.entries,
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

pub struct ValuesMut<'a, K, V> {
    entries: &'a mut Vec<(K, V)>,
    idx: usize,
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.entries.len() {
            let ptr = self.entries.as_mut_ptr();
            unsafe {
                let item = &mut *ptr.add(self.idx);
                self.idx += 1;
                Some(&mut item.1)
            }
        } else {
            None
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

impl<K, V> PartialEq for BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord,
    V: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        if self.entries.len() != other.entries.len() {
            return false;
        }
        for (k, v) in self.entries.iter() {
            if let Some(ov) = other.get(k) {
                if v != ov {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl<K, V> Eq for BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord + Eq,
    V: PartialEq + Clone + Eq,
{
}

impl<K, V> fmt::Debug for BTreeMap<K, V>
where
    K: fmt::Debug + PartialEq + Clone + Ord,
    V: fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.entries.iter().map(|(k, v)| (k, v))).finish()
    }
}

impl<K, V> BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    pub fn values(&self) -> Values<'_, K, V> {
        Values {
            entries: &self.entries,
            idx: 0,
        }
    }
}

pub struct Values<'a, K, V> {
    entries: &'a Vec<(K, V)>,
    idx: usize,
}

impl<'a, K, V> Iterator for Values<'a, K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.entries.len() {
            let item = &self.entries[self.idx].1;
            self.idx += 1;
            Some(item)
        } else {
            None
        }
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
    use crate::klib::Vec;

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

        let mut items = Vec::new();
        for (k, v) in map.iter() {
            items.push((*k, *v));
        }
        assert_eq!(items.len(), 3);
    }
}

impl<K: Clone + core::cmp::Ord, V: Clone> core::ops::Index<K> for BTreeMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &Self::Output {
        self.get(&key).expect("key not found in BTreeMap")
    }
}

impl<K: Clone + core::cmp::Ord, V: Clone> BTreeMap<K, V> {
    pub fn range<R>(&self, range: R) -> Range<'_, K, V>
    where
        R: core::ops::RangeBounds<K>,
    {
        let start = match range.start_bound() {
            core::ops::Bound::Included(x) => x.clone(),
            core::ops::Bound::Excluded(x) => {
                // For simple types, we'd need successor logic
                // For now, just start from the bound
                x.clone()
            }
            core::ops::Bound::Unbounded => {
                if let Some(first) = self.entries.first() {
                    first.0.clone()
                } else {
                    return Range { map: self, index: 0, end: 0 };
                }
            }
        };

        let end = match range.end_bound() {
            core::ops::Bound::Included(x) => {
                // Find index after x
                self.entries.iter().position(|(k, _)| k > x).unwrap_or(self.entries.len())
            }
            core::ops::Bound::Excluded(x) => {
                self.entries.iter().position(|(k, _)| k >= x).unwrap_or(self.entries.len())
            }
            core::ops::Bound::Unbounded => self.entries.len(),
        };

        let start_index = self.entries.iter().position(|(k, _)| k >= &start).unwrap_or(0);

        Range {
            map: self,
            index: start_index,
            end,
        }
    }
}

pub struct Range<'a, K: Clone + Ord, V: Clone> {
    map: &'a BTreeMap<K, V>,
    index: usize,
    end: usize,
}

impl<'a, K: Clone + Ord, V: Clone> Iterator for Range<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end && self.index < self.map.entries.len() {
            let entry = &self.map.entries[self.index];
            self.index += 1;
            Some((&entry.0, &entry.1))
        } else {
            None
        }
    }
}
