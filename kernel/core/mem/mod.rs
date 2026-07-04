//! SigmaOS Memory Management
//! Common Allocator Trait and implementations

#![no_std]

use core::ptr::null_mut;

/// A common allocator trait that defines a standard interface
/// for memory allocation and deallocation (OOP: Polymorphism & Abstraction)
pub trait Allocator {
    /// Allocates a block of memory with at least `size` bytes
    fn alloc(&mut self, size: usize) -> *mut u8;
    
    /// Deallocates a previously allocated block
    fn dealloc(&mut self, ptr: *mut u8, size: usize);
}
