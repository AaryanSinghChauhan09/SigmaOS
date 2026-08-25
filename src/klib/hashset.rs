// SPDX-License-Identifier: MIT
//! Custom HashSet implementation for SigmaOS
//! Reduces dependency on std::collections::HashSet

use super::BTreeMap;
use super::btreemap::BTreeMapIter;
use super::hashmap::BTreeMapIter;

pub struct HashSet<T>
where
    T: Eq + core::hash::Hash + Clone + Ord,
{
    map: BTreeMap<T, ()>,
}

impl<T> Clone for HashSet<T>
where
    T: Eq + core::hash::Hash + Clone + Ord,
{
    fn clone(&self) -> Self {
        HashSet {
            map: self.map.clone(),
        }
    }
}

impl<T> core::fmt::Debug for HashSet<T>
where
    T: Eq + core::hash::Hash + Clone + core::fmt::Debug + Ord,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut set = f.debug_set();
        for item in self.iter() {
            set.entry(item);
        }
        set.finish()
    }
}

impl<T> core::iter::FromIterator<T> for HashSet<T>
where
    T: Eq + core::hash::Hash + Clone + Ord,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = HashSet::new();
        for item in iter {
            set.insert(item);
        }
        set
    }
}

impl<T> HashSet<T>
where
    T: Eq + core::hash::Hash + Clone + Ord,
{
    pub fn new() -> Self {
        HashSet {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, item: T) -> bool {
        let was_present = self.map.contains_key(&item);
        self.map.insert(item, ());
        !was_present
    }

    pub fn remove(&mut self, item: &T) -> bool {
        self.map.remove(item).is_some()
    }

    pub fn contains(&self, item: &T) -> bool {
        self.map.contains_key(item)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn iter(&self) -> HashSetIter<'_, T> {
        HashSetIter {
            map_iter: self.map.iter(),
        }
    }
}

impl<T> Default for HashSet<T>
where
    T: Eq + core::hash::Hash + Clone + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct HashSetIter<'a, T> {
    map_iter: BTreeMapIter<'a, T, ()>,
}

impl<'a, T> Iterator for HashSetIter<'a, T>
where
    T: Eq + core::hash::Hash + Clone + Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next().map(|(key, _)| key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashset_basic() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(!set.contains(&3));
    }

    #[test]
    fn test_hashset_remove() {
        let mut set = HashSet::new();
        set.insert(1);
        assert!(set.remove(&1));
        assert!(!set.contains(&1));
    }

    #[test]
    fn test_hashset_iter() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);

        let items: Vec<i32> = set.iter().cloned().collect();
        assert_eq!(items.len(), 2);
    }
}
