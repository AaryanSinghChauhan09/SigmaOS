//! Custom HashSet implementation for SigmaOS
//! Reduces dependency on std::collections::HashSet

use super::HashMap;

pub struct HashSet<T>
where
    T: PartialEq + Clone,
{
    map: HashMap<T, ()>,
}

impl<T> HashSet<T>
where
    T: PartialEq + Clone,
{
    pub fn new() -> Self {
        HashSet {
            map: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        HashSet {
            map: HashMap::with_capacity(capacity),
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
    T: PartialEq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct HashSetIter<'a, T> {
    map_iter: HashMapIter<'a, T, ()>,
}

impl<'a, T> Iterator for HashSetIter<'a, T>
where
    T: PartialEq + Clone,
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
