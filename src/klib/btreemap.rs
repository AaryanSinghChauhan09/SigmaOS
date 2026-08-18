//! Custom BTreeMap implementation for SigmaOS
//! Reduces dependency on std::collections::BTreeMap
//! Simple implementation using sorted Vec for now

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
        Q: Ord + Eq,
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
        Q: Ord + Eq,
    {
        for (k, v) in self.entries.iter_mut() {
            if k.borrow() == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: core::borrow::Borrow<Q>,
        Q: Ord + Eq,
    {
        for i in 0..self.entries.len() {
            if self.entries[i].0.borrow() == key {
                return Some(self.entries.remove(i).1);
            }
        }
        None
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: core::borrow::Borrow<Q>,
        Q: Ord + Eq,
    {
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

    pub fn values(&self) -> BTreeMapValues<'_, K, V> {
        BTreeMapValues {
            entries: &self.entries,
            idx: 0,
        }
    }

    pub fn values_mut(&mut self) -> BTreeMapValuesMut<'_, K, V> {
        BTreeMapValuesMut {
            entries: &mut self.entries,
            idx: 0,
        }
    }

    pub fn keys(&self) -> BTreeMapKeys<'_, K, V> {
        BTreeMapKeys {
            entries: &self.entries,
            idx: 0,
        }
    }

    pub fn entry(&mut self, key: K) -> BTreeMapEntry<'_, K, V> {
        for i in 0..self.entries.len() {
            if self.entries[i].0 == key {
                return BTreeMapEntry::Occupied(BTreeMapOccupied {
                    value: &mut self.entries[i].1,
                });
            }
        }
        BTreeMapEntry::Vacant(BTreeMapVacant {
            map: self,
            key,
        })
    }
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

impl<K, V> core::fmt::Debug for BTreeMap<K, V>
where
    K: PartialEq + Clone + Ord + core::fmt::Debug,
    V: Clone + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
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

pub struct BTreeMapValues<'a, K, V> {
    entries: &'a Vec<(K, V)>,
    idx: usize,
}

pub struct BTreeMapValuesMut<'a, K, V> {
    entries: &'a mut Vec<(K, V)>,
    idx: usize,
}

impl<'a, K, V> Iterator for BTreeMapValuesMut<'a, K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.entries.len() {
            // Unsafe to bypass lifetime check when indexing into a mutable slice/vector in a loop
            let ptr = self.entries.as_mut_ptr();
            unsafe {
                let item = &mut (*ptr.add(self.idx)).1;
                self.idx += 1;
                Some(item)
            }
        } else {
            None
        }
    }
}

impl<'a, K, V> Iterator for BTreeMapValues<'a, K, V>
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

pub struct BTreeMapKeys<'a, K, V> {
    entries: &'a Vec<(K, V)>,
    idx: usize,
}

impl<'a, K, V> Iterator for BTreeMapKeys<'a, K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.entries.len() {
            let item = &self.entries[self.idx].0;
            self.idx += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub enum BTreeMapEntry<'a, K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    Occupied(BTreeMapOccupied<'a, V>),
    Vacant(BTreeMapVacant<'a, K, V>),
}

impl<'a, K, V> BTreeMapEntry<'a, K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            BTreeMapEntry::Occupied(o) => o.value,
            BTreeMapEntry::Vacant(v) => {
                v.map.insert(v.key.clone(), default);
                // Find the just-inserted entry
                let key = v.key;
                let len = v.map.entries.len();
                for i in 0..len {
                    if v.map.entries[i].0 == key {
                        return &mut v.map.entries[i].1;
                    }
                }
                unreachable!()
            }
        }
    }

    pub fn or_insert_with<F: FnOnce() -> V>(self, f: F) -> &'a mut V {
        match self {
            BTreeMapEntry::Occupied(o) => o.value,
            BTreeMapEntry::Vacant(v) => {
                let val = f();
                v.map.insert(v.key.clone(), val);
                let key = v.key;
                let len = v.map.entries.len();
                for i in 0..len {
                    if v.map.entries[i].0 == key {
                        return &mut v.map.entries[i].1;
                    }
                }
                unreachable!()
            }
        }
    }
}

pub struct BTreeMapOccupied<'a, V> {
    pub value: &'a mut V,
}

pub struct BTreeMapVacant<'a, K, V>
where
    K: PartialEq + Clone + Ord,
    V: Clone,
{
    map: &'a mut BTreeMap<K, V>,
    key: K,
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
        assert_eq!(items.as_slice(), &[(1, "a"), (2, "b"), (3, "c")]);
    }
}