// Custom BTreeMap implementation for SigmaOS
// Reduces dependency on std::collections::BTreeMap
// Simple implementation using sorted Vec for now

use super::Vec;
use core::borrow::Borrow;
use core::cmp::PartialEq;

#[derive(Debug, PartialEq, Eq)]
pub struct BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    entries: Vec<(K, V)>,
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
    K: Eq + Clone + Ord,
    V: Eq + Clone,
{
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
    K: Eq + Clone + Ord,
    V: Eq + Clone,
{}

pub enum Entry<'a, K: 'a + PartialEq + Clone + Ord, V: 'a + Clone> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

pub struct OccupiedEntry<'a, K: 'a + PartialEq + Clone + Ord, V: 'a + Clone> {
    map: &'a mut BTreeMap<K, V>,
    index: usize,
}

pub struct VacantEntry<'a, K: 'a + PartialEq + Clone + Ord, V: 'a + Clone> {
    map: &'a mut BTreeMap<K, V>,
    key: K,
}

impl<'a, K: PartialEq + Clone + Ord, V: Clone> Entry<'a, K, V> {
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => &mut entry.map.entries[entry.index].1,
            Entry::Vacant(entry) => {
                let key = entry.key.clone();
                entry.map.insert(key.clone(), default);
                entry.map.get_mut(&key).unwrap()
            }
        }
    }

    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => &mut entry.map.entries[entry.index].1,
            Entry::Vacant(entry) => {
                let key = entry.key.clone();
                let val = default();
                entry.map.insert(key.clone(), val);
                entry.map.get_mut(&key).unwrap()
            }
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

    /// Optimized by Bolt ⚡: replaces O(N) linear iteration with O(log N) binary search (`binary_search_by`),
    /// finding the exact match or sorted insertion index in logarithmic time.
    pub fn insert(&mut self, key: K, value: V) {
        match self.entries.as_slice().binary_search_by(|entry| entry.0.cmp(&key)) {
            Ok(idx) => {
                self.entries[idx] = (key, value);
            }
            Err(idx) => {
                self.entries.insert(idx, (key, value));
            }
        }
    }

    /// Optimized by Bolt ⚡: uses O(log N) binary search to locate occupied entry index or target vacant index.
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        match self.entries.as_slice().binary_search_by(|entry| entry.0.cmp(&key)) {
            Ok(idx) => Entry::Occupied(OccupiedEntry {
                map: self,
                index: idx,
            }),
            Err(_) => Entry::Vacant(VacantEntry { map: self, key }),
        }
    }

    /// Optimized by Bolt ⚡: replaces O(N) linear iteration with O(log N) binary search lookup.
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        if let Ok(idx) = self.entries.as_slice().binary_search_by(|entry| entry.0.borrow().cmp(key)) {
            Some(&self.entries[idx].1)
        } else {
            None
        }
    }

    /// Optimized by Bolt ⚡: replaces O(N) linear iteration with O(log N) binary search lookup.
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        if let Ok(idx) = self.entries.as_slice().binary_search_by(|entry| entry.0.borrow().cmp(key)) {
            Some(&mut self.entries[idx].1)
        } else {
            None
        }
    }

    /// Optimized by Bolt ⚡: replaces O(N) linear iteration with O(log N) binary search lookup.
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        if let Ok(idx) = self.entries.as_slice().binary_search_by(|entry| entry.0.borrow().cmp(key)) {
            Some(self.entries.remove(idx).1)
        } else {
            None
        }
    }

    /// Optimized by Bolt ⚡: replaces O(N) linear iteration with O(log N) binary search lookup.
    pub fn remove_str(&mut self, key: &str) -> Option<V>
    where
        K: core::convert::AsRef<str>,
    {
        if let Ok(idx) = self.entries.as_slice().binary_search_by(|entry| entry.0.as_ref().cmp(key)) {
            Some(self.entries.remove(idx).1)
        } else {
            None
        }
    }

    /// Optimized by Bolt ⚡: replaces O(N) linear iteration with O(log N) binary search lookup.
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.get(key).is_some()
    }

    /// Optimized by Bolt ⚡: replaces O(N) linear iteration with O(log N) binary search lookup.
    pub fn contains_key_str(&self, key: &str) -> bool
    where
        K: core::convert::AsRef<str>,
    {
        self.entries.as_slice().binary_search_by(|entry| entry.0.as_ref().cmp(key)).is_ok()
    }

    /// Optimized by Bolt ⚡: replaces O(N) linear iteration with O(log N) binary search lookup.
    pub fn get_str(&self, key: &str) -> Option<&V>
    where
        K: core::convert::AsRef<str>,
    {
        if let Ok(idx) = self.entries.as_slice().binary_search_by(|entry| entry.0.as_ref().cmp(key)) {
            Some(&self.entries[idx].1)
        } else {
            None
        }
    }

    /// Optimized by Bolt ⚡: replaces O(N) linear iteration with O(log N) binary search lookup.
    pub fn get_mut_str(&mut self, key: &str) -> Option<&mut V>
    where
        K: core::convert::AsRef<str>,
    {
        if let Ok(idx) = self.entries.as_slice().binary_search_by(|entry| entry.0.as_ref().cmp(key)) {
            Some(&mut self.entries[idx].1)
        } else {
            None
        }
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

    #[test]
    fn test_btreemap_basic() {
        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.insert(3, "c");
        map.insert(2, "b");

        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&2), Some(&"b"));
        assert_eq!(map.get(&3), Some(&"c"));
        assert_eq!(map.get(&4), None);
    }

    #[test]
    fn test_btreemap_binary_search_order_and_entry() {
        let mut map = BTreeMap::new();
        map.insert(10, "ten");
        map.insert(5, "five");
        map.insert(15, "fifteen");

        assert_eq!(map.contains_key(&5), true);
        assert_eq!(map.contains_key(&10), true);
        assert_eq!(map.contains_key(&15), true);
        assert_eq!(map.contains_key(&20), false);

        // Test entry API
        map.entry(20).or_insert("twenty");
        assert_eq!(map.get(&20), Some(&"twenty"));

        *map.entry(10).or_insert("x") = "TEN";
        assert_eq!(map.get(&10), Some(&"TEN"));
    }

    #[test]
    fn test_btreemap_remove() {
        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        assert_eq!(map.remove(&1), Some("a"));
        assert_eq!(map.get(&1), None);
        assert_eq!(map.contains_key(&1), false);
        assert_eq!(map.get(&2), Some(&"b"));
    }

    #[test]
    fn test_btreemap_iter() {
        let mut map = BTreeMap::new();
        map.insert(3, "c");
        map.insert(1, "a");
        map.insert(2, "b");

        let items: std::vec::Vec<(i32, &str)> = map.iter().map(|(k, v)| (*k, *v)).collect();
        assert_eq!(items, std::vec![(1, "a"), (2, "b"), (3, "c")]);
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
                    return Range {
                        map: self,
                        index: 0,
                        end: 0,
                    };
                }
            }
        };

        let end = match range.end_bound() {
            core::ops::Bound::Included(x) => {
                // Find index after x
                self.entries
                    .iter()
                    .position(|(k, _)| k > x)
                    .unwrap_or(self.entries.len())
            }
            core::ops::Bound::Excluded(x) => self
                .entries
                .iter()
                .position(|(k, _)| k >= x)
                .unwrap_or(self.entries.len()),
            core::ops::Bound::Unbounded => self.entries.len(),
        };

        let start_index = self
            .entries
            .iter()
            .position(|(k, _)| k >= &start)
            .unwrap_or(0);

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
