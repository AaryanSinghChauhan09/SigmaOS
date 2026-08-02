#![allow(clippy::all, warnings)]

use core::mem;

pub struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

impl<T> Vec<T> {
    #[allow(clippy::new_without_default)]
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

    pub fn insert(&mut self, index: usize, element: T) {
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
            core::ptr::write(self.data.add(index), element);
            self.len += 1;
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

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
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
            unsafe {
                if &*self.data.add(i) == item {
                    return true;
                }
            }
        }
        false
    }

    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
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
                }
            }
        }
        self.len = write_idx;
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8, self.capacity * mem::size_of::<T>());
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            new_vec.push(self[i].clone());
        }
        new_vec
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&**self, f)
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
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

impl<T, U> PartialEq<[U]> for Vec<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U]) -> bool {
        if self.len != other.len() {
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

impl<T, U, const N: usize> PartialEq<[U; N]> for Vec<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U; N]) -> bool {
        self.eq(&other[..])
    }
}

impl<T, U> PartialEq<std::vec::Vec<U>> for Vec<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &std::vec::Vec<U>) -> bool {
        self.eq(&other[..])
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

pub struct VecIntoIter<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
    index: usize,
}

impl<T> Iterator for VecIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(self.index));
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
        if self.capacity > 0 && !self.data.is_null() {
            unsafe {
                for i in self.index..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8, self.capacity * mem::size_of::<T>());
            }
        }
    }
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = VecIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        let ptr = self.data;
        let len = self.len;
        let cap = self.capacity;
        core::mem::forget(self);
        VecIntoIter {
            data: ptr,
            len,
            capacity: cap,
            index: 0,
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 && !self.data.is_null() {
            for i in 0..self.len {
                unsafe {
                    core::ptr::drop_in_place(self.data.add(i));
                }
            }
            unsafe {
                free(self.data as *mut u8, self.capacity * mem::size_of::<T>());
            }
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8, size: usize) {
    use std::alloc::{dealloc as std_dealloc, Layout};
    if !ptr.is_null() && size > 0 {
        let layout = Layout::from_size_align(size, 8).unwrap();
        std_dealloc(ptr, layout);
    }
}

#[cfg(target_os = "none")]
unsafe fn free(ptr: *mut u8, size: usize) {
    extern "C" {
        fn free(ptr: *mut u8);
    }
    let _ = size;
    free(ptr);
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
}
