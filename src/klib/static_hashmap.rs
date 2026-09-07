use crate::klib::hash::SimpleHasher;
use core::hash::{Hash, Hasher};
use core::mem::MaybeUninit;

/// A lightweight, allocation-free, fixed-size association grid map.
/// Replaces std::collections::BTreeMap inside the kernel.
pub struct StaticHashMap<K, V, const N: usize> {
    entries: [MaybeUninit<Option<(K, V)>>; N],
    len: usize,
}

impl<K, V, const N: usize> StaticHashMap<K, V, N> {
    /// Creates a new, empty StaticHashMap.
    pub fn new() -> Self {
        let mut entries: [MaybeUninit<Option<(K, V)>>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for entry in &mut entries {
            unsafe {
                core::ptr::write(entry.as_mut_ptr(), None);
            }
        }
        StaticHashMap { entries, len: 0 }
    }

    /// Computes the index for a key using direct index mapping and a hashing multiplier.
    fn hash_index<Q: ?Sized + Hash>(&self, key: &Q) -> usize {
        let mut hasher = SimpleHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        // Multiplicative hashing multiplier (fractional part of Golden Ratio)
        let multiplier: u64 = 0x9e3779b97f4a7c15;
        (hash.wrapping_mul(multiplier) as usize) % N
    }

    /// Helper to get a reference to the entry at index `idx`.
    unsafe fn get_entry(&self, idx: usize) -> &Option<(K, V)> {
        &*self.entries[idx].as_ptr()
    }

    /// Helper to get a mutable reference to the entry at index `idx`.
    unsafe fn get_entry_mut(&mut self, idx: usize) -> &mut Option<(K, V)> {
        &mut *self.entries[idx].as_mut_ptr()
    }
}

impl<K, V, const N: usize> StaticHashMap<K, V, N>
where
    K: Eq + Hash,
{
    /// Inserts a key-value pair into the map.
    /// If the map did not have this key present, None is returned.
    /// If the map did have this key present, the value is updated, and the old value is returned.
    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, &'static str> {
        let start_idx = self.hash_index(&key);
        let mut idx = start_idx;

        for _ in 0..N {
            let entry_ptr = self.entries[idx].as_mut_ptr();
            let is_match = unsafe {
                if let Some(ref mut pair) = *entry_ptr {
                    pair.0 == key
                } else {
                    false
                }
            };

            if is_match {
                let pair = unsafe { (*entry_ptr).as_mut().unwrap() };
                let old_val = core::mem::replace(&mut pair.1, value);
                return Ok(Some(old_val));
            }

            let is_none = unsafe { (*entry_ptr).is_none() };
            if is_none {
                if self.len >= N {
                    return Err("StaticHashMap capacity exceeded");
                }
                unsafe {
                    *entry_ptr = Some((key, value));
                }
                self.len += 1;
                return Ok(None);
            }

            idx = (idx + 1) % N;
        }

        Err("StaticHashMap capacity exceeded")
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> Option<&V>
    where
        K: core::borrow::Borrow<Q>,
    {
        let start_idx = self.hash_index(key);
        let mut idx = start_idx;

        loop {
            let entry = unsafe { self.get_entry(idx) };
            match entry {
                Some(ref pair) => {
                    if pair.0.borrow() == key {
                        return Some(&pair.1);
                    }
                }
                None => return None,
            }

            idx = (idx + 1) % N;
            if idx == start_idx {
                return None;
            }
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut<Q: ?Sized + Hash + Eq>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: core::borrow::Borrow<Q>,
    {
        let start_idx = self.hash_index(key);
        let mut idx = start_idx;

        for _ in 0..N {
            let entry_ptr = self.entries[idx].as_mut_ptr();
            let is_none = unsafe { (*entry_ptr).is_none() };
            if is_none {
                return None;
            }

            let is_match = unsafe {
                if let Some(ref pair) = *entry_ptr {
                    pair.0.borrow() == key
                } else {
                    false
                }
            };

            if is_match {
                let pair = unsafe { (*entry_ptr).as_mut().unwrap() };
                return Some(&mut pair.1);
            }

            idx = (idx + 1) % N;
        }
        None
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    pub fn remove<Q: ?Sized + Hash + Eq>(&mut self, key: &Q) -> Option<V>
    where
        K: core::borrow::Borrow<Q>,
    {
        let start_idx = self.hash_index(key);
        let mut idx = start_idx;

        loop {
            let entry = unsafe { self.get_entry(idx) };
            match entry {
                Some(ref pair) => {
                    if pair.0.borrow() == key {
                        let removed = unsafe { self.get_entry_mut(idx) }.take().unwrap();
                        self.len -= 1;

                        // We must rehash any subsequent keys to prevent breaking the probe chain.
                        self.rehash_after(idx);

                        return Some(removed.1);
                    }
                }
                None => return None,
            }

            idx = (idx + 1) % N;
            if idx == start_idx {
                return None;
            }
        }
    }

    /// Rehashes entries after a removed element to ensure collision chain continuity.
    fn rehash_after(&mut self, start_empty_idx: usize) {
        let mut idx = (start_empty_idx + 1) % N;
        while let Some((k, v)) = unsafe { self.get_entry_mut(idx) }.take() {
            self.len -= 1;
            let _ = self.insert(k, v);
            idx = (idx + 1) % N;
        }
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Clears the map, removing all key-value pairs.
    pub fn clear(&mut self) {
        for idx in 0..N {
            let entry = unsafe { self.get_entry_mut(idx) };
            if entry.is_some() {
                *entry = None;
            }
        }
        self.len = 0;
    }
}

impl<K, V, const N: usize> Default for StaticHashMap<K, V, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, const N: usize> Drop for StaticHashMap<K, V, N> {
    fn drop(&mut self) {
        for idx in 0..N {
            unsafe {
                core::ptr::drop_in_place(self.entries[idx].as_mut_ptr());
            }
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_static_hashmap() {
        let mut map: StaticHashMap<&str, i32, 8> = StaticHashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());

        assert_eq!(map.insert("one", 1), Ok(None));
        assert_eq!(map.insert("two", 2), Ok(None));
        assert_eq!(map.len(), 2);

        assert_eq!(map.get(&"one"), Some(&1));
        assert_eq!(map.get(&"two"), Some(&2));
        assert_eq!(map.get(&"three"), None);

        assert_eq!(map.insert("one", 11), Ok(Some(1)));
        assert_eq!(map.get(&"one"), Some(&11));

        assert_eq!(map.remove(&"one"), Some(11));
        assert_eq!(map.get(&"one"), None);
        assert_eq!(map.len(), 1);

        map.clear();
        assert_eq!(map.len(), 0);
    }
}
