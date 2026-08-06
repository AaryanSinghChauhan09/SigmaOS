// SigmaOS Custom Arc (Atomic Reference Counting)
// Reduces dependency on std::sync::Arc

#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

/// ArcInner - Internal structure for Arc
struct ArcInner<T> {
    count: AtomicUsize,
    data: T,
}

/// Arc - Atomic Reference Counting smart pointer
/// Provides thread-safe shared ownership with reference counting
pub struct Arc<T> {
    ptr: NonNull<ArcInner<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    /// Create a new Arc with the given data
    pub fn new(data: T) -> Self {
        let inner = Box::leak(Box::new(ArcInner {
            count: AtomicUsize::new(1),
            data,
        }));
        
        Arc {
            ptr: NonNull::new(inner).expect("ArcInner pointer should not be null"),
        }
    }

    /// Get the number of strong references to this Arc
    pub fn strong_count(&self) -> usize {
        self.inner().count.load(Ordering::SeqCst)
    }

    /// Get a mutable reference to the inner data
    /// Only safe if this is the only reference
    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        if this.inner().count.load(Ordering::SeqCst) == 1 {
            unsafe {
                Some(&mut this.ptr.as_mut().data)
            }
        } else {
            None
        }
    }

    /// Get the inner ArcInner reference
    fn inner(&self) -> &ArcInner<T> {
        unsafe { self.ptr.as_ref() }
    }

    /// Increment the reference count
    fn increment(&self) {
        self.inner().count.fetch_add(1, Ordering::SeqCst);
    }

    /// Decrement the reference count and drop if zero
    fn decrement(&self) {
        let old_count = self.inner().count.fetch_sub(1, Ordering::SeqCst);
        
        if old_count == 1 {
            // This was the last reference, drop the data
            unsafe {
                let _ = Box::from_raw(self.ptr.as_ptr());
            }
        }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        self.increment();
        Arc {
            ptr: self.ptr,
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        self.decrement();
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.ptr.as_ref().data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc_new() {
        let arc = Arc::new(42);
        assert_eq!(*arc, 42);
    }

    #[test]
    fn test_arc_clone() {
        let arc = Arc::new(42);
        let arc2 = arc.clone();
        assert_eq!(arc.strong_count(), 2);
        assert_eq!(*arc2, 42);
    }

    #[test]
    fn test_arc_drop() {
        let arc = Arc::new(42);
        {
            let _arc2 = arc.clone();
            assert_eq!(arc.strong_count(), 2);
        }
        assert_eq!(arc.strong_count(), 1);
    }

    #[test]
    fn test_arc_get_mut() {
        let mut arc = Arc::new(42);
        assert_eq!(Arc::get_mut(&mut arc), Some(&mut 42));
        
        let arc2 = arc.clone();
        assert_eq!(Arc::get_mut(&mut arc), None);
    }
}