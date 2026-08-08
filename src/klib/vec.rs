<<<<<<< HEAD
#![no_std]
#![no_main]

use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};
||||||| 23ef22a4a
use core::mem;
=======
// Custom, Zero-Dependency Dynamic Vector Collection for SigmaOS
// Eliminates pre-defined collection libraries by managing raw pointer allocations directly
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

<<<<<<< HEAD
pub struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}
||||||| 23ef22a4a
pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

impl<T: PartialEq> Vec<T> {
    pub fn contains(&self, item: &T) -> bool {
        for i in 0..self.len {
            if &self[i] == item {
                return true;
            }
        }
<<<<<<< HEAD
        false
    }
}

impl<T> Vec<T> {
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.data.add(index)) }
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            unsafe { Some(&mut *self.data.add(index)) }
        } else {
            None
        }
    }
    pub fn first(&self) -> Option<&T> {
        self.get(0)
    }
    pub fn last(&self) -> Option<&T> {
        if self.len > 0 {
            self.get(self.len - 1)
        } else {
            None
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(core::ptr::read(self.data.add(self.len))) }
        }
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

    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }



    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
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

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut write_idx = 0;
        for i in 0..self.len {
            let item = &self[i];
            if f(item) {
                if write_idx != i {
                    unsafe {
                        core::ptr::copy_nonoverlapping(
                            self.data.add(i),
                            self.data.add(write_idx),
                            1,
                        );
                    }
                }
                write_idx += 1;
            } else {
                unsafe {
                    core::ptr::drop_in_place(self.data.add(i));
                }
            }
        }
        self.len = write_idx;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
||||||| 23ef22a4a
        new_vec
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Vec<T> {
    pub fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }

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

    pub fn contains(&self, item: &T) -> bool where T: PartialEq {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item { return true; }
            }
        }
        false
    }
    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter { vec: self, index: 0 }
    }
    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut { data: self.data, len: self.len, index: 0, _marker: core::marker::PhantomData }
    }
    pub fn remove(&mut self, index: usize) -> T {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            // Shift elements right
            for i in (index..self.len).rev() {
                core::ptr::copy_nonoverlapping(self.data.add(i), self.data.add(i + 1), 1);
            }
            core::ptr::write(self.data.add(index), item);
            self.len += 1;
        }
    }
    pub fn first(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            unsafe { Some(&*self.data) }
        }
    }
    pub fn last(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            unsafe { Some(&*self.data.add(self.len - 1)) }
        }
    }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }

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

    pub fn contains(&self, item: &T) -> bool where T: PartialEq {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item { return true; }
            }
        }
        false
    }
    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter { vec: self, index: 0 }
    }
    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut { data: self.data, len: self.len, index: 0, _marker: core::marker::PhantomData }
    }
    pub fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    pub fn retain<F>(&mut self, mut f: F) where F: FnMut(&T) -> bool {
        let mut write_idx = 0;
        for i in 0..self.len {
            unsafe {
                let item = &*self.data.add(i);
                if f(item) {
                    if write_idx != i {
                        core::ptr::copy_nonoverlapping(self.data.add(i), self.data.add(write_idx), 1);
                    }
                    write_idx += 1;
                }
            }
        }
        self.len = write_idx;
    }
    unsafe fn grow(&mut self) {
        let size = mem::size_of::<T>();
        if size == 0 {
            self.capacity = self.capacity.checked_add(4).unwrap_or(self.capacity);
            return;
        }
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let align = mem::align_of::<T>();
        let new_data = alloc(new_capacity * size, align) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 && !self.data.is_null() {
                free(self.data as *mut u8, self.capacity * size, align);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }
}

<<<<<<< HEAD
impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Vec::new();
        for item in self.iter() {
            new_vec.push(item.clone());
        }
        new_vec
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T: PartialEq> PartialEq for Vec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: PartialEq<U>, U> PartialEq<[U]> for Vec<T> {
    fn eq(&self, other: &[U]) -> bool {
        self.as_slice() == other
    }
}

#[cfg(not(target_os = "none"))]
impl<T: PartialEq<U>, U> PartialEq<std::vec::Vec<U>> for Vec<T> {
    fn eq(&self, other: &std::vec::Vec<U>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T> core::iter::FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for item in iter {
            vec.push(item);
        }
        vec
    }
}

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
||||||| 23ef22a4a
impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
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

pub struct VecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = unsafe { &*self.vec.data.add(self.index) };
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

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            for i in 0..self.len {
                unsafe {
                    core::ptr::drop_in_place(self.data.add(i));
                }
            }
            let size = mem::size_of::<T>();
            if size > 0 && !self.data.is_null() {
                unsafe {
                    free(self.data as *mut u8, self.capacity * size, mem::align_of::<T>());
                }
            }
        }
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }
}
<<<<<<< HEAD

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
||||||| 23ef22a4a

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize, align: usize) -> *mut u8 {
    if size == 0 {
        return core::ptr::null_mut();
    }
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, align).unwrap_or_else(|_| {
        Layout::from_size_align(size, 8).unwrap()
    });
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8, size: usize, align: usize) {
    use std::alloc::{dealloc as std_dealloc, Layout};
    if !ptr.is_null() && size > 0 {
        let layout = Layout::from_size_align(size, align).unwrap_or_else(|_| {
            Layout::from_size_align(size, 8).unwrap()
        });
        std_dealloc(ptr, layout);
    }
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize, align: usize) -> *mut u8;
    fn free(ptr: *mut u8, size: usize, align: usize);
}
=======
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
