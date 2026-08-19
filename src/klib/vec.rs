use core::mem;

pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }

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

    pub fn with_capacity(capacity: usize) -> Self {
        let mut vec = Self::new();
        if capacity > 0 {
            unsafe {
                let data = alloc(capacity * mem::size_of::<T>()) as *mut T;
                if !data.is_null() {
                    vec.data = data;
                    vec.capacity = capacity;
                }
            }
        }
        vec
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(core::ptr::read(self.data.add(self.len)))
            }
        }
    }
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

    pub fn insert(&mut self, index: usize, item: T) {
        assert!(index <= self.len, "index out of bounds");
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

    pub fn clear(&mut self) {
        for i in 0..self.len {
            unsafe { core::ptr::drop_in_place(self.data.add(i)); }
        }
        self.len = 0;
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
        if self.len > 0 { self.get(self.len - 1) } else { None }
    }

    pub fn extend_from_slice(&mut self, other: &[T]) where T: Clone {
        for item in other {
            self.push(item.clone());
        }
    }

    pub fn insert(&mut self, index: usize, item: T) {
        assert!(index <= self.len, "insertion index out of bounds");
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                // shift elements right
                for i in (index..self.len).rev() {
                    core::ptr::copy_nonoverlapping(self.data.add(i), self.data.add(i + 1), 1);
                }
                core::ptr::write(self.data.add(index), item);
                self.len += 1;
            }
        }
    }

    pub fn sort_by<F>(&mut self, mut compare: F) where F: FnMut(&T, &T) -> core::cmp::Ordering {
        // Simple insertion sort - correct, if not the fastest
        for i in 1..self.len {
            let mut j = i;
            while j > 0 {
                unsafe {
                    if compare(&*self.data.add(j - 1), &*self.data.add(j)) == core::cmp::Ordering::Greater {
                        core::ptr::swap(self.data.add(j - 1), self.data.add(j));
                        j -= 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    pub fn dedup(&mut self) where T: PartialEq {
        if self.len <= 1 { return; }
        let mut write = 1;
        for read in 1..self.len {
            unsafe {
                if *self.data.add(read) != *self.data.add(write - 1) {
                    if write != read {
                        core::ptr::copy_nonoverlapping(self.data.add(read), self.data.add(write), 1);
                    }
                    write += 1;
                }
            }
        }
        self.len = write;
    }

    pub fn truncate(&mut self, len: usize) {
        while self.len > len {
            self.len -= 1;
            unsafe { core::ptr::drop_in_place(self.data.add(self.len)); }
        }
    }

    pub fn as_ptr(&self) -> *const T {
        self.data
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        let last = self.len - 1;
        if index != last {
            unsafe { core::ptr::swap(self.data.add(index), self.data.add(last)); }
        }
        self.pop().unwrap()
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        if a != b {
            assert!(a < self.len && b < self.len, "swap index out of bounds");
            unsafe { core::ptr::swap(self.data.add(a), self.data.add(b)); }
        }
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8, self.capacity * mem::size_of::<T>()); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T: PartialEq> PartialEq for Vec<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len { return false; }
        for i in 0..self.len {
            unsafe {
                if *self.data.add(i) != *other.data.add(i) { return false; }
            }
        }
        true
    }
}

impl<T: Eq> Eq for Vec<T> {}

impl<T> FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v = Vec::new();
        for item in iter {
            v.push(item);
        }
        v
    }
}

impl<T: Clone> From<&[T]> for Vec<T> {
    fn from(slice: &[T]) -> Self {
        let mut v = Vec::new();
        for item in slice {
            v.push(item.clone());
        }
        v
    }
}


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
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8, size: usize);
}
