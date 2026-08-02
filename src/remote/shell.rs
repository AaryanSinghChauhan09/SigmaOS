#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// OOP-based Remote Shell for SigmaOS
// Implements remote shell access using OOP principles with traits and structs.

use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type ShellID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellError {
    Success = 0,
    NotFound = 1,
    CommandFailed = 2,
}

pub trait RemoteShell {
    fn id(&self) -> ShellID;
    fn host(&self) -> &[u8];
    fn execute(&self, command: &[u8]) -> Result<Vec<u8>, ShellError>;
}

#[repr(C)]
pub struct SimpleRemoteShell {
    pub id: ShellID,
    pub host: [u8; 128],
}

impl SimpleRemoteShell {
    pub fn new(id: ShellID, host: &[u8]) -> Self {
        let mut host_array = [0u8; 128];
        let host_len = host.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(host.as_ptr(), host_array.as_mut_ptr(), host_len);
        }
        SimpleRemoteShell {
            id,
            host: host_array,
        }
    }
}

impl RemoteShell for SimpleRemoteShell {
    fn id(&self) -> ShellID {
        self.id
    }
    fn host(&self) -> &[u8] {
        let len = self.host.iter().position(|&b| b == 0).unwrap_or(128);
        &self.host[..len]
    }

    fn execute(&self, command: &[u8]) -> Result<Vec<u8>, ShellError> {
        let mut output = Vec::new();
        for &byte in command {
            output.push(byte);
        }
        output.push(b'\n');
        Ok(output)
    }
}

pub trait ShellManager {
    fn connect(&mut self, host: &[u8], port: u16) -> Result<ShellID, ShellError>;
    fn disconnect(&mut self, id: ShellID) -> Result<(), ShellError>;
    fn get_shell(&self, id: ShellID) -> Option<&dyn RemoteShell>;
}

#[repr(C)]
pub struct SimpleShellManager {
    pub shells: Vec<Option<Box<dyn RemoteShell>>>,
    pub next_id: AtomicUsize,
}

impl SimpleShellManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleShellManager {
            shells: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ShellManager for SimpleShellManager {
    fn connect(&mut self, host: &[u8], _port: u16) -> Result<ShellID, ShellError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let shell = SimpleRemoteShell::new(id, host);
        self.shells.push(Some(Box::new(shell)));
        Ok(id)
    }

    fn disconnect(&mut self, id: ShellID) -> Result<(), ShellError> {
        for shell_option in self.shells.iter_mut() {
            if let Some(ref mut shell) = *shell_option {
                if shell.id() == id {
                    return Ok(());
                }
            }
        }
        Err(ShellError::NotFound)
    }

    fn get_shell(&self, id: ShellID) -> Option<&dyn RemoteShell> {
        for shell_option in self.shells.iter() {
            if let Some(ref shell) = *shell_option {
                if shell.id() == id {
                    return Some(shell.as_ref());
                }
            }
        }
        None
    }
}

pub trait FileTransfer {
    fn upload(
        &self,
        shell_id: ShellID,
        local_path: &[u8],
        remote_path: &[u8],
    ) -> Result<(), ShellError>;
    fn download(
        &self,
        shell_id: ShellID,
        remote_path: &[u8],
        local_path: &[u8],
    ) -> Result<(), ShellError>;
}

#[repr(C)]
pub struct SimpleFileTransfer {
    pub manager: SimpleShellManager,
}

impl SimpleFileTransfer {
    pub fn new(manager: SimpleShellManager) -> Self {
        SimpleFileTransfer { manager }
    }
}

impl FileTransfer for SimpleFileTransfer {
    fn upload(
        &self,
        shell_id: ShellID,
        _local_path: &[u8],
        _remote_path: &[u8],
    ) -> Result<(), ShellError> {
        if self.manager.get_shell(shell_id).is_some() {
            Ok(())
        } else {
            Err(ShellError::NotFound)
        }
    }

    fn download(
        &self,
        shell_id: ShellID,
        _remote_path: &[u8],
        _local_path: &[u8],
    ) -> Result<(), ShellError> {
        if self.manager.get_shell(shell_id).is_some() {
            Ok(())
        } else {
            Err(ShellError::NotFound)
        }
    }
}

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 && !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

impl<T> Vec<T> {
    #[allow(clippy::new_without_default)]
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
            if self.capacity > self.len && !self.data.is_null() {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn len(&self) -> usize {
        self.len
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

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_shell_execution() {
        let mut manager = SimpleShellManager::new();
        let shell_id = manager.connect(b"admin-terminal", 22).unwrap();

        assert_eq!(shell_id, 1);
        let shell = manager.get_shell(shell_id).unwrap();
        assert_eq!(shell.host(), b"admin-terminal");

        let res = shell.execute(b"ls").unwrap();
        assert_eq!(res[0], b'l');
        assert_eq!(res[res.len() - 1], b'\n');

        let transfer = SimpleFileTransfer::new(manager);
        assert!(transfer.upload(shell_id, b"/tmp/f1", b"/home/f1").is_ok());
    }
}
