// Stack-Based Zero-Allocation Helper for SigmaOS Kernel Core
// Location: src/kernel/core/stack_alloc.rs

// #![no_std]
use core::mem::MaybeUninit;

pub struct StackBuffer<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T: Copy, const N: usize> StackBuffer<T, N> {
    pub const fn new() -> Self {
        StackBuffer {
            data: [MaybeUninit::uninit(); N],
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), ()> {
        if self.len < N {
            self.data[self.len] = MaybeUninit::new(item);
            self.len += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index < self.len {
            unsafe { Some(self.data[index].assume_init()) }
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}

#[inline(always)]
pub unsafe fn stack_alloc<T: Copy, const N: usize>() -> [T; N] {
    MaybeUninit::uninit().assume_init()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_buffer_zero_alloc() {
        let mut buf = StackBuffer::<u32, 16>::new();
        assert_eq!(buf.capacity(), 16);
        assert_eq!(buf.len(), 0);

        buf.push(100).expect("push 100");
        buf.push(200).expect("push 200");

        assert_eq!(buf.len(), 2);
        assert_eq!(buf.get(0), Some(100));
        assert_eq!(buf.get(1), Some(200));
    }
}
