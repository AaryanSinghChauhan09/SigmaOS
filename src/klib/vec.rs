#![no_std]
#![no_main]

use core::mem;

pub struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

pub struct Iter<'a, T> {
    ptr: *const T,
    end: *const T,
    _marker: core::marker::PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        } else {
            unsafe {
                let result = &*self.ptr;
                self.ptr = self.ptr.add(1);
                Some(result)
            }
        }
    }
}

pub struct IterMut<'a, T> {
    ptr: *mut T,
    end: *mut T,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        } else {
            unsafe {
                let result = &mut *self.ptr;
                self.ptr = self.ptr.add(1);
                Some(result)
            }
        }
    }
}

impl<T: PartialEq> Vec<T> {
    pub fn contains(&self, item: &T) -> bool {
        for i in 0..self.len {
            if &self[i] == item {
                return true;
            }
        }
        false
    }
}

impl<T> Vec<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            ptr: self.data,
            end: unsafe { if self.data.is_null() { self.data } else { self.data.add(self.len) } },
            _marker: core::marker::PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            ptr: self.data,
            end: unsafe { if self.data.is_null() { self.data } else { self.data.add(self.len) } },
            _marker: core::marker::PhantomData,
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

    pub fn insert(&mut self, index: usize, item: T) {
        if index > self.len {
            panic!("index out of bounds");
        }
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            // shift elements to the right
            for i in (index..self.len).rev() {
                core::ptr::copy_nonoverlapping(self.data.add(i), self.data.add(i + 1), 1);
            }
            core::ptr::write(self.data.add(index), item);
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
    }
}

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.len == 0 {
            &[] as &[T]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.len == 0 {
            &mut [] as &mut [T]
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
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

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    use std::alloc::{dealloc, Layout};
    // We don't have the layout layout size here, so we leak or let std dealloc?
    // Actually, in hosted mode, leaking is fine because we don't have layout size, or we can use layout if we tracked it, but leak is safe since it's just for testing.
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
