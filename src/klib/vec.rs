//! Custom vector implementation for SigmaOS
//! This module provides no_std alternatives to std::vec with reduced dependency on predefined functions

use core::alloc::{Layout, alloc, dealloc};
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ptr;
use core::slice;

/// Custom vector type for SigmaOS with reduced dependency on predefined functions
pub struct SigmaVec<T> {
    ptr: *mut T,
    capacity: usize,
    len: usize,
}

impl<T> SigmaVec<T> {
    /// Create a new empty vector
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            capacity: 0,
            len: 0,
        }
    }
    
    /// Create a new vector with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }
        
        let layout = Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { alloc(layout) as *mut T };
        
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        
        Self {
            ptr,
            capacity,
            len: 0,
        }
    }
    
    /// Get the length of the vector
    pub fn len(&self) -> usize {
        self.len
    }
    
    /// Check if the vector is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    /// Get the capacity of the vector
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Reserve additional capacity
    pub fn reserve(&mut self, additional: usize) {
        if self.capacity - self.len >= additional {
            return;
        }
        
        let new_capacity = self.capacity.max(1) * 2;
        self.resize(new_capacity);
    }
    
    /// Push an element to the vector
    pub fn push(&mut self, item: T) {
        if self.len == self.capacity {
            self.reserve(1);
        }
        
        unsafe {
            ptr::write(self.ptr.add(self.len), item);
        }
        self.len += 1;
    }
    
    /// Pop an element from the vector
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        
        self.len -= 1;
        unsafe {
            Some(ptr::read(self.ptr.add(self.len)))
        }
    }
    
    /// Remove an element at a specific index
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);
        
        unsafe {
            let item = ptr::read(self.ptr.add(index));
            
            // Shift remaining elements
            for i in index..self.len - 1 {
                ptr::copy(self.ptr.add(i + 1), self.ptr.add(i), 1);
            }
            
            self.len -= 1;
            item
        }
    }
    
    /// Insert an element at a specific index
    pub fn insert(&mut self, index: usize, item: T) {
        assert!(index <= self.len);
        
        if self.len == self.capacity {
            self.reserve(1);
        }
        
        unsafe {
            // Shift elements to make space
            for i in (index..self.len).rev() {
                ptr::copy(self.ptr.add(i), self.ptr.add(i + 1), 1);
            }
            
            ptr::write(self.ptr.add(index), item);
            self.len += 1;
        }
    }
    
    /// Clear the vector
    pub fn clear(&mut self) {
        // Drop all elements
        for i in 0..self.len {
            unsafe {
                ptr::drop_in_place(self.ptr.add(i));
            }
        }
        self.len = 0;
    }
    
    /// Resize the vector
    fn resize(&mut self, new_capacity: usize) {
        if new_capacity == 0 {
            self.deallocate();
            self.ptr = ptr::null_mut();
            self.capacity = 0;
            return;
        }
        
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        let new_ptr = unsafe { alloc(new_layout) as *mut T };
        
        if new_ptr.is_null() {
            panic!("Allocation failed");
        }
        
        // Copy existing elements
        unsafe {
            for i in 0..self.len {
                ptr::copy_nonoverlapping(self.ptr.add(i), new_ptr.add(i), 1);
            }
        }
        
        // Deallocate old memory
        self.deallocate();
        
        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
    
    /// Deallocate the vector's memory
    fn deallocate(&mut self) {
        if !self.ptr.is_null() && self.capacity > 0 {
            unsafe {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
    
    /// Get a reference to an element
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe {
                Some(&*self.ptr.add(index))
            }
        } else {
            None
        }
    }
    
    /// Get a mutable reference to an element
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            unsafe {
                Some(&mut *self.ptr.add(index))
            }
        } else {
            None
        }
    }
    
    /// Get a slice of the vector
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }
    
    /// Get a mutable slice of the vector
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr, self.len)
        }
    }
    
    /// Extend the vector with an iterator
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter {
            self.push(item);
        }
    }
    
    /// Truncate the vector to a new length
    pub fn truncate(&mut self, new_len: usize) {
        assert!(new_len <= self.len);
        
        // Drop elements beyond new_len
        for i in new_len..self.len {
            unsafe {
                ptr::drop_in_place(self.ptr.add(i));
            }
        }
        
        self.len = new_len;
    }
}

impl<T> Drop for SigmaVec<T> {
    fn drop(&mut self) {
        self.clear();
        self.deallocate();
    }
}

impl<T> Default for SigmaVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for SigmaVec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = SigmaVec::with_capacity(self.capacity);
        for i in 0..self.len {
            unsafe {
                let item = (&*self.ptr.add(i)).clone();
                new_vec.push(item);
            }
        }
        new_vec
    }
}

impl<T: PartialEq> PartialEq for SigmaVec<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        
        for i in 0..self.len {
            unsafe {
                if &*self.ptr.add(i) != &*other.ptr.add(i) {
                    return false;
                }
            }
        }
        
        true
    }
}

impl<T: Eq> Eq for SigmaVec<T> {}

impl<T> IntoIterator for SigmaVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            vec: ManuallyDrop::new(self),
            index: 0,
        }
    }
}

/// Iterator for SigmaVec
pub struct IntoIter<T> {
    vec: ManuallyDrop<SigmaVec<T>>,
    index: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            let item = unsafe {
                ptr::read(self.vec.ptr.add(self.index))
            };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // Drop remaining elements
        for i in self.index..self.vec.len {
            unsafe {
                ptr::drop_in_place(self.vec.ptr.add(i));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec_creation() {
        let vec: SigmaVec<i32> = SigmaVec::new();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);
        
        let vec = SigmaVec::with_capacity(10);
        assert!(vec.is_empty());
        assert_eq!(vec.capacity(), 10);
    }
    
    #[test]
    fn test_vec_push_pop() {
        let mut vec = SigmaVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), None);
    }
    
    #[test]
    fn test_vec_get() {
        let mut vec = SigmaVec::new();
        vec.push(10);
        vec.push(20);
        vec.push(30);
        
        assert_eq!(vec.get(0), Some(&10));
        assert_eq!(vec.get(1), Some(&20));
        assert_eq!(vec.get(2), Some(&30));
        assert_eq!(vec.get(3), None);
    }
    
    #[test]
    fn test_vec_remove() {
        let mut vec = SigmaVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        
        let removed = vec.remove(1);
        assert_eq!(removed, 2);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&3));
        assert_eq!(vec.get(2), Some(&4));
    }
    
    #[test]
    fn test_vec_insert() {
        let mut vec = SigmaVec::new();
        vec.push(1);
        vec.push(3);
        vec.push(4);
        
        vec.insert(1, 2);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), Some(&3));
        assert_eq!(vec.get(3), Some(&4));
    }
    
    #[test]
    fn test_vec_clear() {
        let mut vec = SigmaVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        vec.clear();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);
    }
    
    #[test]
    fn test_vec_extend() {
        let mut vec = SigmaVec::new();
        vec.extend(vec![1, 2, 3]);
        
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), Some(&3));
    }
}
