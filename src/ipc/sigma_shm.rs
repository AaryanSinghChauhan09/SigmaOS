//! SigmaOS POSIX Shared Memory (shm_open / mmap)
//!
//! Sovereign implementation of POSIX shared memory.
//! Inspired by Linux shm_open(3), FreeBSD shm_open(2).
//!
//! Allows multiple processes to share memory regions without pipes.
//! Backed by anonymous tmpfs pages in the kernel.

#![allow(dead_code)]
#![allow(clippy::new_without_default)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// POSIX open flags for shm_open.
pub struct OFlags(pub u32);
impl OFlags {
    pub const O_RDONLY: u32 = 0;
    pub const O_RDWR:   u32 = 2;
    pub const O_CREAT:  u32 = 64;
    pub const O_EXCL:   u32 = 128;
    pub const O_TRUNC:  u32 = 512;
}

/// A shared memory object.
#[derive(Debug, Clone)]
pub struct ShmObject {
    pub name: String,
    pub size: usize,
    /// Reference count (number of open handles)
    pub ref_count: u32,
    /// Owner UID
    pub uid: u32,
    /// Permissions (Unix mode bits)
    pub mode: u16,
    /// Data buffer (in-process simulation)
    pub data: Vec<u8>,
    /// Creation time (ns)
    pub created_ns: u64,
}

impl ShmObject {
    pub fn new(name: &str, size: usize, uid: u32, mode: u16, now_ns: u64) -> Self {
        Self {
            name: name.into(), size, ref_count: 1, uid, mode,
            data: vec![0u8; size], created_ns: now_ns,
        }
    }

    pub fn read(&self, offset: usize, buf: &mut [u8]) -> Result<usize, &'static str> {
        if offset >= self.size { return Ok(0); }
        let end = (offset + buf.len()).min(self.size);
        let n = end - offset;
        buf[..n].copy_from_slice(&self.data[offset..end]);
        Ok(n)
    }

    pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<usize, &'static str> {
        if offset >= self.size { return Err("out of bounds"); }
        let end = (offset + data.len()).min(self.size);
        let n = end - offset;
        self.data[offset..end].copy_from_slice(&data[..n]);
        Ok(n)
    }

    pub fn truncate(&mut self, new_size: usize) {
        self.data.resize(new_size, 0);
        self.size = new_size;
    }
}

/// File descriptor for a shared memory object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShmFd(pub u32);

/// The system-wide shared memory namespace.
pub struct SigmaShmNamespace {
    objects: BTreeMap<String, ShmObject>,
    fds: BTreeMap<ShmFd, String>,
    next_fd: u32,
}

impl SigmaShmNamespace {
    pub fn new() -> Self {
        Self { objects: BTreeMap::new(), fds: BTreeMap::new(), next_fd: 1 }
    }

    /// shm_open — create or open a shared memory object.
    pub fn shm_open(&mut self, name: &str, flags: u32, mode: u16, uid: u32, now_ns: u64)
        -> Result<ShmFd, &'static str>
    {
        let create = (flags & OFlags::O_CREAT) != 0;
        let excl   = (flags & OFlags::O_EXCL)  != 0;

        if !name.starts_with('/') { return Err("EINVAL: name must start with /"); }

        if self.objects.contains_key(name) {
            if create && excl { return Err("EEXIST"); }
            self.objects.get_mut(name).unwrap().ref_count += 1;
        } else {
            if !create { return Err("ENOENT"); }
            self.objects.insert(name.into(), ShmObject::new(name, 0, uid, mode, now_ns));
        }

        let fd = ShmFd(self.next_fd);
        self.next_fd += 1;
        self.fds.insert(fd, name.into());
        Ok(fd)
    }

    /// ftruncate — set size of a shared memory object.
    pub fn ftruncate(&mut self, fd: ShmFd, size: usize) -> Result<(), &'static str> {
        let name = self.fds.get(&fd).ok_or("EBADF")?.clone();
        let obj = self.objects.get_mut(&name).ok_or("ENOENT")?;
        obj.truncate(size);
        Ok(())
    }

    /// mmap simulation — read from mapped region.
    pub fn read(&self, fd: ShmFd, offset: usize, buf: &mut [u8]) -> Result<usize, &'static str> {
        let name = self.fds.get(&fd).ok_or("EBADF")?;
        self.objects.get(name).ok_or("ENOENT")?.read(offset, buf)
    }

    /// mmap simulation — write to mapped region.
    pub fn write(&mut self, fd: ShmFd, offset: usize, data: &[u8]) -> Result<usize, &'static str> {
        let name = self.fds.get(&fd).ok_or("EBADF")?.clone();
        self.objects.get_mut(&name).ok_or("ENOENT")?.write(offset, data)
    }

    /// close — decrement ref count.
    pub fn close(&mut self, fd: ShmFd) -> Result<(), &'static str> {
        let name = self.fds.remove(&fd).ok_or("EBADF")?;
        if let Some(obj) = self.objects.get_mut(&name) {
            obj.ref_count = obj.ref_count.saturating_sub(1);
        }
        Ok(())
    }

    /// shm_unlink — schedule object for deletion when all refs closed.
    pub fn shm_unlink(&mut self, name: &str) -> Result<(), &'static str> {
        let obj = self.objects.get(name).ok_or("ENOENT")?;
        if obj.ref_count == 0 {
            self.objects.remove(name);
        }
        // If ref_count > 0, the object is "unlinked" but kept alive until close
        Ok(())
    }

    pub fn object_count(&self) -> usize { self.objects.len() }
    pub fn get_object(&self, name: &str) -> Option<&ShmObject> { self.objects.get(name) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_write() {
        let mut ns = SigmaShmNamespace::new();
        let fd = ns.shm_open("/test", OFlags::O_CREAT | OFlags::O_RDWR, 0o600, 0, 0).unwrap();
        ns.ftruncate(fd, 4096).unwrap();
        ns.write(fd, 0, b"SigmaOS SHM").unwrap();
        let mut buf = [0u8; 11];
        ns.read(fd, 0, &mut buf).unwrap();
        assert_eq!(&buf, b"SigmaOS SHM");
    }

    #[test]
    fn test_excl_create_fails() {
        let mut ns = SigmaShmNamespace::new();
        ns.shm_open("/excl", OFlags::O_CREAT | OFlags::O_RDWR, 0o600, 0, 0).unwrap();
        assert!(ns.shm_open("/excl", OFlags::O_CREAT | OFlags::O_EXCL, 0o600, 0, 0).is_err());
    }
}
