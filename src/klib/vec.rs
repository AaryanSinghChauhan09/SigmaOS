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

    pub fn sort_by<F>(&mut self, mut compare: F)
    where
        F: FnMut(&T, &T) -> core::cmp::Ordering,
    {
        if self.len <= 1 {
            return;
        }
        for i in 1..self.len {
            let mut j = i;
            while j > 0 {
                let elem_j = unsafe { &*self.data.add(j) };
                let elem_prev = unsafe { &*self.data.add(j - 1) };
                if compare(elem_prev, elem_j) == core::cmp::Ordering::Greater {
                    unsafe {
                        core::ptr::swap(self.data.add(j - 1), self.data.add(j));
                    }
                    j -= 1;
                } else {
                    break;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
            }
        }
        self.len = 0;
    }

    pub fn as_slice(&self) -> &[T] {
        if self.len == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.len == 0 {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }

    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.len {
            if unsafe { &*self.data.add(i) } == item {
                return true;
            }
        }
        false
    }

    pub fn insert(&mut self, index: usize, item: T) {
        if index > self.len {
            panic!("index out of bounds");
        }
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            for i in (index..self.len).rev() {
                core::ptr::copy_nonoverlapping(self.data.add(i), self.data.add(i + 1), 1);
            }
            core::ptr::write(self.data.add(index), item);
            self.len += 1;
        }
    }

    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        unsafe { core::slice::from_raw_parts(self.data, self.len).iter() }
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

impl<T: PartialEq> PartialEq for Vec<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        for i in 0..self.len {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl<T: Eq + PartialEq> Eq for Vec<T> {}

impl<T: Clone> Clone for Vec<T> {
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

impl<T: core::fmt::Debug> core::fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for item in iter {
            vec.push(item);
        }
        vec
    }
}
