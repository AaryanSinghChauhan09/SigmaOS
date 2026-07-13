//! SigmaOS Custom Core Types (No-Std, OOP-Based)
//! Implements SigmaVec and SigmaString using const generics for fixed-capacity, 
//! allocation-free collections.

#![no_std]

/// A bare-metal, fixed-capacity vector.
pub struct SigmaVec<T, const N: usize> {
    data: [core::mem::MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> SigmaVec<T, N> {
    pub const fn new() -> Self {
        Self {
            // SAFETY: MaybeUninit array initialization is valid.
            data: unsafe { core::mem::MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.len >= N {
            return Err("SigmaVec capacity exceeded");
        }
        unsafe {
            self.data[self.len].as_mut_ptr().write(item);
        }
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        unsafe {
            Some(self.data[self.len].as_ptr().read())
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe {
            core::slice::from_raw_parts(self.data.as_ptr() as *const T, self.len)
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            core::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut T, self.len)
        }
    }
}

/// A bare-metal, fixed-capacity String.
pub struct SigmaString<const N: usize> {
    vec: SigmaVec<u8, N>,
}

impl<const N: usize> SigmaString<N> {
    pub const fn new() -> Self {
        Self {
            vec: SigmaVec::new(),
        }
    }

    pub fn push_str(&mut self, s: &str) -> Result<(), &'static str> {
        for b in s.bytes() {
            self.vec.push(b)?;
        }
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(self.vec.as_slice())
        }
    }

    pub fn clear(&mut self) {
        self.vec.len = 0;
    }
}
