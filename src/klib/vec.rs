
use core::hash::{Hash, Hasher};
use core::mem;

pub type SigmaVec<T> = Vec<T>;

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T: PartialEq> Vec<T> {
    pub fn contains(&self, item: &T) -> bool {
        for i in 0..self.len {
            // SAFETY: `i` is always in `0..self.len`, and `self.data` points to a valid,
            // initialised allocation of at least `self.len` elements.
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
        // SAFETY: `size` is non-zero (capacity > 0 and size_of::<T>() >= 1 for non-ZST).
        // The returned pointer is either null (checked by callers) or valid for `size` bytes.
        let new_data = unsafe { alloc(size) as *mut T };
        Vec {
            data: new_data,
            len: 0,
            capacity,
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        if self.len + additional > self.capacity {
            // SAFETY: delegated to `grow_to`, which validates allocation before use.
            unsafe {
                self.grow_to(self.len + additional);
            }
        }
    }

    pub fn truncate(&mut self, new_len: usize) {
        // SAFETY: `self.len - 1` is always in range because the while loop
        // condition guards against underflow, and `self.data` is valid for
        // indices `0..self.len`.
        unsafe {
            while self.len > new_len {
                self.len -= 1;
                core::ptr::drop_in_place(self.data.add(self.len));
            }
        }
    }

    pub fn push(&mut self, item: T) {
        // SAFETY: After `grow()` succeeds, `self.capacity > self.len`, so
        // writing at `self.data.add(self.len)` is within the allocation.
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
            // SAFETY: `self.data` is non-null when `self.len > 0` (ensured by `push`/`grow`),
            // is properly aligned for `T`, and covers exactly `self.len` initialised elements.
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.len == 0 {
            &mut []
        } else {
            // SAFETY: Same guarantees as `as_slice`; additionally no other
            // reference to the same elements exists while this `&mut self` is live.
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
        // SAFETY: `index < self.len` is checked above; `self.data + index` is within
        // the allocation. `copy_nonoverlapping` shifts elements left one position —
        // valid because `i+1 <= self.len - 1` inside the loop.
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
            // SAFETY: `i` is in `0..self.len`; all accesses are within bounds.
            // Elements that do not pass the predicate are dropped in place before
            // `write_idx` advances, preventing double-free.
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
        // SAFETY: `i` is in `0..self.len`; each element is dropped exactly once
        // and `self.len` is set to 0 so the destructor won't double-free.
        unsafe {
            for i in 0..self.len {
                core::ptr::drop_in_place(self.data.add(i));
            }
            self.len = 0;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            // SAFETY: `index < self.len` guarantees the pointer is within bounds
            // and the element is initialised.
            unsafe { Some(&*self.data.add(index)) }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            // SAFETY: Same as `get`; additionally the exclusive borrow of `self`
            // ensures no aliasing.
            unsafe { Some(&mut *self.data.add(index)) }
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            // SAFETY: `self.len > 0` so `self.len - 1` is a valid, initialised index.
            // After decrement, the element at that position is logically removed.
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
        // SAFETY: After optional growth, `self.capacity > self.len`.  The loop shifts
        // elements to the right making space at `index`.  All accesses are within
        // `0..=self.len` which is covered by the allocation.
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

    /// Optimized by Bolt ⚡: replaces element-by-element push loops with a single capacity
    /// reservation and bulk memory copy (`copy_nonoverlapping`), converting O(N) reallocations
    /// and checks into an O(1) bulk SIMD/memcpy write.
    pub fn extend_from_slice(&mut self, other: &[T])
    where
        T: Copy,
    {
        if other.is_empty() {
            return;
        }
        self.reserve(other.len());
        // SAFETY: `reserve` ensures `self.data.add(self.len)` through
        // `self.data.add(self.len + other.len())` is within the allocation.
        // `T: Copy` means no drop glue is needed for the source elements.
        unsafe {
            core::ptr::copy_nonoverlapping(other.as_ptr(), self.data.add(self.len), other.len());
            self.len += other.len();
        }
    }

    // SAFETY contract for `grow` and `grow_to`:
    // • The allocation returned by `alloc` is valid for `new_capacity * size_of::<T>()` bytes.
    // • If `new_data` is null the grow is silently skipped (out-of-memory path; callers
    //   should handle or panic after the call).
    // • Old elements are moved via `copy_nonoverlapping` (valid because they are `T`-aligned)
    //   and the old allocation is freed only after the copy.
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        self.grow_to(new_capacity);
    }

    unsafe fn grow_to(&mut self, new_capacity: usize) {
        // SAFETY: `new_capacity * size_of::<T>()` is the correct byte count.
        // If the allocator returns null we leave the vec unchanged (capacity stays).
        let new_byte_size = new_capacity * mem::size_of::<T>();
        if new_byte_size == 0 {
            return;
        }
        let new_data = alloc(new_byte_size) as *mut T;
        if !new_data.is_null() {
            if self.capacity > 0 && !self.data.is_null() && self.len > 0 {
                // Bulk copy: O(1) single memcpy instead of element-by-element loop.
                // SAFETY: `self.data..self.data+self.len` and `new_data..new_data+self.len`
                // do not overlap because they come from distinct allocations.
                core::ptr::copy_nonoverlapping(self.data, new_data, self.len);
            }
            if self.capacity > 0 && !self.data.is_null() {
                #[cfg(not(target_os = "none"))]
                free_sized(self.data as *mut u8, self.capacity * mem::size_of::<T>());
                #[cfg(target_os = "none")]
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
            // SAFETY: `self.index < self.vec.len` guarantees the element is
            // initialised. `ptr::read` moves ownership to the caller; the
            // element at `self.index` must not be dropped again (ensured by
            // the `Drop` impl which only drops `self.index..self.vec.len`).
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
        // SAFETY: Elements in `self.index..self.vec.len` have not been moved
        // out yet (they were not yielded by `next`) and must be dropped.
        // Setting `self.vec.len = 0` prevents the Vec's own destructor from
        // dropping them a second time.
        unsafe {
            for i in self.index..self.vec.len {
                core::ptr::drop_in_place(self.vec.data.add(i));
            }
            self.vec.len = 0;
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
            // SAFETY: `self.index < self.vec.len()` guarantees bounds; the
            // lifetime `'a` is tied to the originating `&Vec`, ensuring no
            // concurrent mutation.
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
            // SAFETY: `self.index < self.len` guarantees bounds.
            // `PhantomData<&'a mut T>` encodes the exclusive-borrow invariant;
            // no two `next()` calls yield overlapping references.
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
        // SAFETY: `index < self.len` is checked above.
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        // SAFETY: `index < self.len` is checked above; exclusive borrow prevents aliasing.
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 && !self.data.is_null() {
            // SAFETY: Each element in `0..self.len` is initialised and dropped exactly
            // once here.  The allocation is then freed.  `self.capacity > 0 &&
            // !self.data.is_null()` guards against double-free on zero-capacity vecs.
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                #[cfg(not(target_os = "none"))]
                free_sized(self.data as *mut u8, self.capacity * core::mem::size_of::<T>());
                #[cfg(target_os = "none")]
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
            // SAFETY: `self.start` is in `original_start..self.end`, which is a
            // sub-range of `0..self.vec.len`.  `ptr::read` transfers ownership;
            // the `Drop` impl for `Drain` will not drop these elements again.
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
        // SAFETY: Elements from `self.start` to `self.end` that were not
        // consumed by `next()` are dropped here.  Then the tail
        // (`self.end..self.vec.len`) is shifted left to close the gap.
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

/// Allocate `size` bytes with 8-byte alignment.
/// On hosted targets uses the global allocator; on bare-metal delegates to the
/// kernel's C allocator via FFI.
///
/// # Safety
/// Caller must ensure `size > 0`.  The returned pointer must be freed with
/// the corresponding `free()` call once done.
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::{alloc as std_alloc, Layout};
    // Layout::from_size_align can only fail if align is not a power of two or
    // size overflows; both conditions are impossible here (align=8, size>0).
    let layout = Layout::from_size_align(size, 8).expect("invalid layout");
    std_alloc::alloc(layout)
}

/// Free memory previously returned by `alloc(size)`.
/// On hosted targets this calls the global dealloc with the same layout; on
/// bare-metal it forwards to the kernel's C free via FFI.
///
/// # Safety
/// `ptr` must have been returned by `alloc(size)` with the same `size`, and
/// must not be used after this call.
#[cfg(not(target_os = "none"))]
unsafe fn free_sized(ptr: *mut u8, size: usize) {
    use std::{dealloc, Layout};
    if !ptr.is_null() && size > 0 {
        let layout = Layout::from_size_align(size, 8).expect("invalid layout");
        dealloc(ptr, layout);
    }
}

/// Bare-metal target: all allocation/free is handled by the kernel C runtime.
#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
