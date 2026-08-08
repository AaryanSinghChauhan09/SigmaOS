// Custom, Zero-Dependency Dynamic Vector Collection for SigmaOS
// Eliminates pre-defined collection libraries by managing raw pointer allocations directly

#![no_std]
#![allow(warnings)]
#![allow(clippy::all)]

use core::alloc::Layout;
use core::ops::{Index, IndexMut};

extern crate alloc;
use alloc::alloc::{alloc, dealloc, realloc};

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    /// Creates a new, empty custom vector
    pub fn new() -> Self {
        Self {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    /// Returns the length of the vector
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the vector is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Pushes an item to the end of the vector, dynamically growing if needed
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            core::ptr::write(self.data.add(self.len), item);
            self.len += 1;
        }
    }

    /// Pops an item from the end of the vector
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(core::ptr::read(self.data.add(self.len))) }
        }
    }

    /// Removes an item at the specified index, shifting trailing elements left
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "Index out of bounds!");

        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }

    /// Checks if the vector contains an item matching a predicate
    pub fn contains<F>(&self, mut f: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        for i in 0..self.len {
            unsafe {
                if f(&*self.data.add(i)) {
                    return true;
                }
            }
        }
        false
    }

    /// Returns a slice containing the entire vector
    pub fn as_slice(&self) -> &[T] {
        if self.len == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }

    /// Returns a mutable slice containing the entire vector
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.len == 0 {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }

    /// Helper to grow the backing memory capacity
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();

        if size == 0 {
            self.capacity = usize::MAX; // Zero-sized types don't require allocations
            return;
        }

        let new_layout = Layout::from_size_align_unchecked(new_capacity * size, align);
        let new_data = if self.capacity == 0 {
            alloc(new_layout) as *mut T
        } else {
            let old_layout = Layout::from_size_align_unchecked(self.capacity * size, align);
            realloc(self.data as *mut u8, old_layout, new_capacity * size) as *mut T
        };

        if new_data.is_null() {
            panic!("Memory allocation failed during Vec growth!");
        }

        self.data = new_data;
        self.capacity = new_capacity;
    }
    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter {
            data: self.data,
            len: self.len,
            index: 0,
            _marker: core::marker::PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut {
            data: self.data,
            len: self.len,
            index: 0,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Custom manual trait implementations to bypass predefined generic bounds
impl<T> Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "Index out of bounds!");
        unsafe { &*self.data.add(index) }
    }
}

impl<T> IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len, "Index out of bounds!");
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 && !self.data.is_null() {
            let size = core::mem::size_of::<T>();
            if size > 0 {
                // Drop existing initialized items
                for i in 0..self.len {
                    unsafe {
                        core::ptr::drop_in_place(self.data.add(i));
                    }
                }
                let align = core::mem::align_of::<T>();
                unsafe {
                    dealloc(
                        self.data as *mut u8,
                        Layout::from_size_align_unchecked(self.capacity * size, align),
                    );
                }
            }
        }
    }
}

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                new_vec.push((*self.data.add(i)).clone());
            }
        }
        new_vec
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..self.len {
            unsafe {
                list.entry(&*self.data.add(i));
            }
        }
        list.finish()
    }
}

/// Custom iterator structure
pub struct IntoIter<T> {
    vec: Vec<T>,
    index: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            let item = unsafe { core::ptr::read(self.vec.data.add(self.index)) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            vec: self,
            index: 0,
        }
    }
}

pub struct VecIter<'a, T> {
    data: *const T,
    len: usize,
    index: usize,
    _marker: core::marker::PhantomData<&'a T>,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { &*self.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct VecIterMut<'a, T> {
    data: *mut T,
    len: usize,
    index: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for VecIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { &mut *self.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = VecIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = VecIterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_vec_operations() {
        let mut vec = Vec::new();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);

        vec.push(10);
        vec.push(20);
        vec.push(30);
        assert_eq!(vec.len(), 3);
        assert!(!vec.is_empty());

        assert_eq!(vec[0], 10);
        assert_eq!(vec[1], 20);
        assert_eq!(vec[2], 30);

        assert_eq!(vec.pop(), Some(30));
        assert_eq!(vec.len(), 2);

        // Test remove
        assert_eq!(vec.remove(0), 10);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 20);
    }

    #[test]
    fn test_custom_vec_clone_and_debug() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);

        let cloned = vec.clone();
        assert_eq!(cloned.len(), 2);
        assert_eq!(cloned[0], 1);

        let debug_str = alloc::format!("{:?}", vec);
        assert_eq!(debug_str, "[1, 2]");
    }

    #[test]
    fn test_custom_vec_contains() {
        let mut vec = Vec::new();
        vec.push(42);
        assert!(vec.contains(|&x| x == 42));
        assert!(!vec.contains(|&x| x == 100));
    }
}
