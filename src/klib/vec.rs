#![no_std]

use core::mem;
use core::hash::{Hash, Hasher};

pub type SigmaVec<T> = Vec<T>;

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T: PartialEq> Vec<T> {
    pub fn contains(&self, item: &T) -> bool {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item {
                    return true;
                }
            }
        }
        false
    }
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }
        let size = mem::size_of::<T>() * capacity;
        let new_data = alloc(size) as *mut T;
        Vec {
            data: new_data,
            len: 0,
            capacity,
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        if self.len + additional > self.capacity {
            unsafe { self.grow_to(self.len + additional); }
        }
    }

    pub fn truncate(&mut self, new_len: usize) {
        unsafe {
            while self.len > new_len {
                self.len -= 1;
                core::ptr::drop_in_place(self.data.add(self.len));
            }
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

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
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

    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter {
            vec: self,
            index: 0,
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

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut write_idx = 0;
        for i in 0..self.len {
            unsafe {
                let item = &*self.data.add(i);
                if f(item) {
                    if write_idx != i {
                        core::ptr::copy_nonoverlapping(
                            self.data.add(i),
                            self.data.add(write_idx),
                            1,
                        );
                    }
                    write_idx += 1;
                } else {
                    core::ptr::drop_in_place(self.data.add(i));
                }
            }
        }
        self.len = write_idx;
    }

    pub fn clear(&mut self) {
        unsafe {
            for i in 0..self.len {
                core::ptr::drop_in_place(self.data.add(i));
            }
            self.len = 0;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                Some(core::ptr::read(self.data.add(self.len)))
            }
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
            if index < self.len {
                for i in (index..self.len).rev() {
                    core::ptr::copy_nonoverlapping(self.data.add(i), self.data.add(i + 1), 1);
                }
            }
            core::ptr::write(self.data.add(index), item);
            self.len += 1;
        }
    }

    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T>
    where
        R: core::ops::RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            core::ops::Bound::Included(&x) => x,
            core::ops::Bound::Excluded(&x) => x + 1,
            core::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            core::ops::Bound::Included(&x) => x + 1,
            core::ops::Bound::Excluded(&x) => x,
            core::ops::Bound::Unbounded => self.len,
        };

        Drain {
            vec: self,
            start,
            end,
        }
    }

    pub fn extend_from_slice(&mut self, other: &[T])
    where
        T: Copy,
    {
        for &item in other {
            self.push(item);
        }
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        self.grow_to(new_capacity);
    }

    unsafe fn grow_to(&mut self, new_capacity: usize) {
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            if self.capacity > 0 && !self.data.is_null() {
                for i in 0..self.len {
                    core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
                }
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

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

impl<T: Eq> Eq for Vec<T> {}

impl<T: Hash> Hash for Vec<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len.hash(state);
        for item in self.iter() {
            item.hash(state);
        }
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

impl<T: Ord> PartialOrd for Vec<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for Vec<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
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

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = VecIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        VecIntoIter {
            vec: self,
            index: 0,
        }
    }
}

pub struct VecIntoIter<T> {
    vec: Vec<T>,
    index: usize,
}

impl<T> Iterator for VecIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            unsafe {
                let item = core::ptr::read(self.vec.data.add(self.index));
                self.index += 1;
                Some(item)
            }
        } else {
            None
        }
    }
}

impl<T> Drop for VecIntoIter<T> {
    fn drop(&mut self) {
        unsafe {
            for i in self.index..self.vec.len {
                core::ptr::drop_in_place(self.vec.data.add(i));
            }
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
        if self.capacity > 0 && !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

pub struct Drain<'a, T> {
    vec: &'a mut Vec<T>,
    start: usize,
    end: usize,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            unsafe {
                self.start += 1;
                Some(core::ptr::read(self.vec.data.add(self.start - 1)))
            }
        } else {
            None
        }
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        unsafe {
            for i in self.start..self.end {
                core::ptr::drop_in_place(self.vec.data.add(i));
            }
            let remaining = self.vec.len - self.end;
            for i in 0..remaining {
                core::ptr::copy_nonoverlapping(
                    self.vec.data.add(self.end + i),
                    self.vec.data.add(self.start + i),
                    1,
                );
            }
            self.vec.len -= self.end - self.start;
        }
    }
}

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
