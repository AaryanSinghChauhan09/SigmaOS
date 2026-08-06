use core::mem;

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}


impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
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
        let size = mem::size_of::<T>();
        if size == 0 {
            self.capacity = self.capacity.checked_add(4).unwrap_or(self.capacity);
            return;
        }
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let align = mem::align_of::<T>();
        let new_data = alloc(new_capacity * size, align) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 && !self.data.is_null() {
                free(self.data as *mut u8, self.capacity * size, align);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
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
                    free(
                        self.data as *mut u8,
                        self.capacity * size,
                        mem::align_of::<T>(),
                    );
                }
            }
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize, align: usize) -> *mut u8 {
    if size == 0 {
        return core::ptr::null_mut();
    }
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, align)
        .unwrap_or_else(|_| Layout::from_size_align(size, 8).unwrap());
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8, size: usize, align: usize) {
    use std::alloc::{dealloc as std_dealloc, Layout};
    if !ptr.is_null() && size > 0 {
        let layout = Layout::from_size_align(size, align)
            .unwrap_or_else(|_| Layout::from_size_align(size, 8).unwrap());
        std_dealloc(ptr, layout);
    }
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize, align: usize) -> *mut u8;
    fn free(ptr: *mut u8, size: usize, align: usize);
}
