//! Custom BTreeMap implementation for SigmaOS
//! Reduces dependency on std::collections::BTreeMap
//! Simple implementation using sorted Vec for now
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


use super::Vec;

pub struct BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    entries: Vec<(K, V)>,
}

impl<K, V> BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    #[allow(clippy::new_without_default)]
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

    pub fn get(&self, key: &K) -> Option<&V> {
        for (k, v) in self.entries.iter() {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        for (k, v) in self.entries.iter_mut() {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        for i in 0..self.entries.len() {
            if self.entries[i].0 == *key {
                return Some(self.entries.remove(i).1);
            }
        }
        None
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
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