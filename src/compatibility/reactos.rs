/// ReactOS-inspired Windows NT Subsystem Compatibility Layer for SigmaOS
/// Provides Portable Executable (PE) parsing, NT Registry Hive management,
/// and NT Object Manager handle tables.

extern crate alloc;
use core::ptr::{self, NonNull};
use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtStatus {
    Success = 0x00000000,
    InvalidHandle = 0xC0000008,
    ObjectNameNotFound = 0xC0000034,
    InvalidImageFormat = 0xC000007B,
    AccessDenied = 0xC0000022,
}

pub type NtHandle = usize;

/// Standard NT Object Type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtObjectType {
    File = 1,
    Process = 2,
    Thread = 3,
    Key = 4,
}

/// Entry representing an allocated NT handle
#[derive(Debug)]
pub struct NtHandleEntry {
    pub handle: NtHandle,
    pub object_type: NtObjectType,
    pub name: [u8; 32],
}

impl NtHandleEntry {
    pub fn new(handle: NtHandle, object_type: NtObjectType, name: &[u8]) -> Self {
        let mut name_array = [0u8; 32];
        let len = name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }
        NtHandleEntry {
            handle,
            object_type,
            name: name_array,
        }
    }
}

/// Windows NT Object Manager
pub struct NtObjectManager {
    pub handles: Vec<Option<NtHandleEntry>>,
    pub next_handle: AtomicUsize,
}

impl NtObjectManager {
    pub fn new() -> Self {
        NtObjectManager {
            handles: Vec::new(),
            next_handle: AtomicUsize::new(0x10), // Handle values typically start at 0x10
        }
    }

    /// Allocate and register a new NT handle (NtCreateFile/NtCreateProcess equivalent)
    pub fn create_object(&mut self, object_type: NtObjectType, name: &[u8]) -> NtHandle {
        let handle = self.next_handle.fetch_add(4, Ordering::SeqCst); // Handles typically increment by 4
        let entry = NtHandleEntry::new(handle, object_type, name);
        self.handles.push(Some(entry));
        handle
    }

    /// Retrieve an object entry from a handle (NtQueryObject equivalent)
    pub fn lookup_object(&self, handle: NtHandle) -> Result<&NtHandleEntry, NtStatus> {
        for i in 0..self.handles.len() {
            if let Some(ref entry) = self.handles[i] {
                if entry.handle == handle {
                    return Ok(entry);
                }
            }
        }
        Err(NtStatus::InvalidHandle)
    }

    /// Close handle (NtClose equivalent)
    pub fn close_handle(&mut self, handle: NtHandle) -> Result<(), NtStatus> {
        for i in 0..self.handles.len() {
            if let Some(ref entry) = self.handles[i] {
                if entry.handle == handle {
                    self.handles[i] = None;
                    return Ok(());
                }
            }
        }
        Err(NtStatus::InvalidHandle)
    }
}

/// Representation of a Windows Registry Value
pub struct RegistryValue {
    pub name: [u8; 32],
    pub data: [u8; 64],
    pub data_len: usize,
}

impl RegistryValue {
    pub fn new(name: &[u8], data: &[u8]) -> Self {
        let mut name_array = [0u8; 32];
        let mut data_array = [0u8; 64];
        let name_len = name.len().min(31);
        let data_len = data.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(data.as_ptr(), data_array.as_mut_ptr(), data_len);
        }

        RegistryValue {
            name: name_array,
            data: data_array,
            data_len,
        }
    }
}

/// Windows NT Registry Hive System (HKLM/HKCU configuration database)
pub struct RegistryHive {
    pub keys: Vec<Option<[u8; 32]>>,
    pub values: Vec<Option<RegistryValue>>,
}

impl RegistryHive {
    pub fn new() -> Self {
        RegistryHive {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Create registry key (NtCreateKey equivalent)
    pub fn create_key(&mut self, key_name: &[u8]) {
        let mut key_array = [0u8; 32];
        let len = key_name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(key_name.as_ptr(), key_array.as_mut_ptr(), len);
        }
        self.keys.push(Some(key_array));
    }

    /// Set registry value (NtSetValueKey equivalent)
    pub fn set_value(&mut self, name: &[u8], data: &[u8]) {
        let value = RegistryValue::new(name, data);
        self.values.push(Some(value));
    }

    /// Retrieve registry value (NtQueryValueKey equivalent)
    pub fn query_value(&self, name: &[u8]) -> Result<&RegistryValue, NtStatus> {
        for i in 0..self.values.len() {
            if let Some(ref val) = self.values[i] {
                let val_name_len = val.name.iter().position(|&b| b == 0).unwrap_or(32);
                if &val.name[..val_name_len] == name {
                    return Ok(val);
                }
            }
        }
        Err(NtStatus::ObjectNameNotFound)
    }
}

/// Windows Portable Executable (PE) parsing loader
pub struct PortableExecutableLoader;

impl PortableExecutableLoader {
    /// Validates MZ DOS stub and PE signature headers for loading Windows executable/driver binaries
    pub fn validate_pe_image(binary: &[u8]) -> Result<(), NtStatus> {
        if binary.len() < 64 {
            return Err(NtStatus::InvalidImageFormat);
        }

        // Validate MZ header ('M' and 'Z')
        if binary[0] != b'M' || binary[1] != b'Z' {
            return Err(NtStatus::InvalidImageFormat);
        }

        // Extract PE header offset from e_lfanew field (at 0x3C)
        let pe_offset = ((binary[0x3C] as usize)
            | ((binary[0x3D] as usize) << 8)
            | ((binary[0x3E] as usize) << 16)
            | ((binary[0x3F] as usize) << 24));

        if pe_offset + 4 > binary.len() {
            return Err(NtStatus::InvalidImageFormat);
        }

        // Validate PE signature ('P', 'E', 0, 0)
        if binary[pe_offset] != b'P'
            || binary[pe_offset + 1] != b'E'
            || binary[pe_offset + 2] != 0
            || binary[pe_offset + 3] != 0
        {
            return Err(NtStatus::InvalidImageFormat);
        }

        Ok(())
    }
}

struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
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
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = extern_alloc(new_capacity * mem::size_of::<T>()) as *mut T;
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
    fn index(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
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
    #[link_name = "alloc"]
    fn extern_alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nt_object_manager_handles() {
        let mut manager = NtObjectManager::new();
        let h1 = manager.create_object(NtObjectType::File, b"DeviceKeyboard");
        let h2 = manager.create_object(NtObjectType::Process, b"explorer.exe");

        assert_eq!(h1, 0x10);
        assert_eq!(h2, 0x14);

        let entry = manager.lookup_object(h1).unwrap();
        assert_eq!(entry.object_type, NtObjectType::File);

        let mut entry_name = [0u8; 14];
        for i in 0..14 {
            entry_name[i] = entry.name[i];
        }
        assert_eq!(&entry_name, b"DeviceKeyboard");

        assert!(manager.close_handle(h1).is_ok());
        assert_eq!(
            manager.lookup_object(h1).unwrap_err() as usize,
            NtStatus::InvalidHandle as usize
        );
    }

    #[test]
    fn test_registry_hive_queries() {
        let mut hive = RegistryHive::new();
        hive.create_key(b"SOFTWARE\\SigmaOS");
        hive.set_value(b"Theme", b"SovereignDark");

        let val = hive.query_value(b"Theme").unwrap();
        assert_eq!(val.data_len, 13);

        let mut val_data = [0u8; 13];
        for i in 0..13 {
            val_data[i] = val.data[i];
        }
        assert_eq!(&val_data, b"SovereignDark");
    }

    #[test]
    fn test_portable_executable_parsing() {
        // Construct a mock minimal valid Windows PE image buffer
        let mut pe_binary = [0u8; 128];
        pe_binary[0] = b'M';
        pe_binary[1] = b'Z';

        // e_lfanew offset field at 0x3C points to PE header location (0x40)
        pe_binary[0x3C] = 0x40;

        // Write standard PE signature at 0x40: 'P', 'E', 0, 0
        pe_binary[0x40] = b'P';
        pe_binary[0x41] = b'E';
        pe_binary[0x42] = 0;
        pe_binary[0x43] = 0;

        assert!(PortableExecutableLoader::validate_pe_image(&pe_binary).is_ok());

        // Invalid MZ signature
        let mut invalid_pe = pe_binary;
        invalid_pe[0] = b'X';
        assert_eq!(
            PortableExecutableLoader::validate_pe_image(&invalid_pe).unwrap_err() as usize,
            NtStatus::InvalidImageFormat as usize
        );
    }
}
