// OOP-based Filesystem Support abstractions for SigmaOS
// Based on 100-Improvement-Ideas.md #49: permission-gated VFS and security checks

use crate::security::CapabilityToken;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    Ext4,
    Btrfs,
    ZFS,
    Fat32,
    SigmaFS,
}

pub trait Filesystem {
    fn id(&self) -> usize;
    fn fs_type(&self) -> FilesystemType;
    fn mount(&mut self) -> Result<(), &'static str>;
    fn unmount(&mut self) -> Result<(), &'static str>;
}

pub struct SimpleFilesystem {
    pub id: usize,
    pub fs_type: AtomicUsize,
    pub is_mounted: bool,
}

impl SimpleFilesystem {
    pub fn new(id: usize) -> Self {
        SimpleFilesystem {
            id,
            fs_type: AtomicUsize::new(0),
            is_mounted: false,
        }
    }
}

impl Filesystem for SimpleFilesystem {
    fn id(&self) -> usize {
        self.id
    }
    fn fs_type(&self) -> FilesystemType {
        match self.fs_type.load(Ordering::SeqCst) {
            1 => FilesystemType::Btrfs,
            2 => FilesystemType::ZFS,
            3 => FilesystemType::Fat32,
            4 => FilesystemType::SigmaFS,
            _ => FilesystemType::Ext4,
        }
    }

    fn mount(&mut self) -> Result<(), &'static str> {
        self.is_mounted = true;
        Ok(())
    }

    fn unmount(&mut self) -> Result<(), &'static str> {
        self.is_mounted = false;
        Ok(())
    }
}

pub struct SecureFilesystemWrapper<F: Filesystem> {
    pub inner: F,
    pub capability_gate: CapabilityToken,
}

impl<F: Filesystem> SecureFilesystemWrapper<F> {
    pub fn new(inner: F, cap: CapabilityToken) -> Self {
        SecureFilesystemWrapper {
            inner,
            capability_gate: cap,
        }
    }

    pub fn mount_secure(&mut self, provided_token: &CapabilityToken) -> Result<(), &'static str> {
        // Enforce secure verification of privilege tokens
        if provided_token.bits() & 1 << 2 != 0 {
            self.inner.mount()
        } else {
            Err("VFS Security Violation: Unprivileged mount block execution")
        }
    }
}

struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
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
    fn is_empty(&self) -> bool {
        self.len == 0
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;
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

extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vfs_secure_mount() {
        let fs = SimpleFilesystem::new(12);
        let cap = CapabilityToken::new(); // unprivileged
        let mut wrapper = SecureFilesystemWrapper::new(fs, cap);

        let provided_token = CapabilityToken::new().allow_read("/mount");
        assert!(wrapper.mount_secure(&provided_token).is_ok());
        assert!(wrapper.inner.is_mounted);
    }
}
