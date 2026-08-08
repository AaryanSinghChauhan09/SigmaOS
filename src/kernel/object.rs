// SigmaOS Windows/Linux/BSD-Inspired Advanced Object Manager (Obp)
// Implements advanced Object Manager namespaces, symbolic link translation,
// driver entry contexts, dynamic unloading, and Non-Paged Pool memory tracking.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(not(test))]
use core::sync::atomic::{AtomicUsize, Ordering};
#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(not(test))]
use crate::klib::HashMap;
#[cfg(test)]
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectError {
    InitFailed,
    RegisterFailed,
    NotFound,
    AlreadyRegistered,
    CapabilityDenied,
    BufferTooSmall,
}

pub struct KRef {
    count: AtomicUsize,
}

impl KRef {
    pub fn new() -> Self {
        KRef {
            count: AtomicUsize::new(1),
        }
    }

    pub fn acquire(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn release(&self) -> bool {
        self.count.fetch_sub(1, Ordering::SeqCst) == 1
    }

    pub fn get(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

pub trait KernelObject: Send + Sync {
    fn name(&self) -> &str;
    fn set_name(&mut self, name: &str);
    fn parent(&self) -> Option<&dyn KernelObject>;
    fn set_parent(&mut self, parent: Option<&dyn KernelObject>);
    fn children(&self) -> Vec<&dyn KernelObject>;
    fn add_child(&mut self, child: &dyn KernelObject);
    fn remove_child(&mut self, child_name: &str) -> Option<Box<dyn KernelObject>>;
    fn kref(&self) -> &KRef;
    fn as_any(&self) -> &dyn core::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any;
    fn sysfs_attrs(&self) -> Vec<&str>;
    fn sysfs_show(&self, attr: &str) -> Option<String>;
    fn sysfs_store(&mut self, attr: &str, value: &str) -> Result<(), ObjectError>;
}

pub struct KObject {
    name: String,
    parent: Option<*const dyn KernelObject>,
    children: Vec<*const dyn KernelObject>,
    kref: KRef,
}

unsafe impl Send for KObject {}
unsafe impl Sync for KObject {}

impl KObject {
    pub fn new(name: &str) -> Self {
        KObject {
            name: name.to_string(),
            parent: None,
            children: Vec::new(),
            kref: KRef::new(),
        }
    }
}

impl KernelObject for KObject {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn parent(&self) -> Option<&dyn KernelObject> {
        self.parent.map(|p| unsafe { &*p })
    }

    fn set_parent(&mut self, parent: Option<&dyn KernelObject>) {
        self.parent = parent.map(|p| unsafe {
            core::mem::transmute::<&dyn KernelObject, &'static dyn KernelObject>(p)
                as *const dyn KernelObject
        });
    }

    fn children(&self) -> Vec<&dyn KernelObject> {
        self.children
            .iter()
            .filter_map(|c| unsafe { c.as_ref() })
            .collect()
    }

    fn add_child(&mut self, child: &dyn KernelObject) {
        self.children.push(unsafe {
            core::mem::transmute::<&dyn KernelObject, &'static dyn KernelObject>(child)
                as *const dyn KernelObject
        });
    }

    fn remove_child(&mut self, child_name: &str) -> Option<Box<dyn KernelObject>> {
        if let Some(idx) = self
            .children
            .iter()
            .position(|c| unsafe { c.as_ref() }.map_or(false, |child| child.name() == child_name))
        {
            let child_ptr = self.children.remove(idx);
            unsafe { Some(Box::from_raw(child_ptr as *mut dyn KernelObject)) }
        } else {
            None
        }
    }

    fn kref(&self) -> &KRef {
        &self.kref
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }

    fn sysfs_attrs(&self) -> Vec<&str> {
        Vec::new()
    }

    fn sysfs_show(&self, _attr: &str) -> Option<String> {
        None
    }

    fn sysfs_store(&mut self, _attr: &str, _value: &str) -> Result<(), ObjectError> {
        Err(ObjectError::CapabilityDenied)
    }
}

impl Drop for KObject {
    fn drop(&mut self) {
        while self.children.pop().is_some() {}
    }
}

pub struct DeviceObject {
    pub base: KObject,
    pub device_id: u16,
    pub vendor_id: u16,
    pub device_type: String,
    pub driver_name: Option<String>,
    pub capabilities: Vec<u64>,
}

impl DeviceObject {
    pub fn new(name: &str, device_id: u16, vendor_id: u16) -> Self {
        DeviceObject {
            base: KObject::new(name),
            device_id,
            vendor_id,
            device_type: String::new(),
            driver_name: None,
            capabilities: Vec::new(),
        }
    }

    pub fn device_id(&self) -> u16 {
        self.device_id
    }

    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    pub fn set_device_type(&mut self, dtype: &str) {
        self.device_type = dtype.to_string();
    }

    pub fn device_type(&self) -> &str {
        &self.device_type
    }

    pub fn set_driver(&mut self, driver: &str) {
        self.driver_name = Some(driver.to_string());
    }

    pub fn driver_name(&self) -> Option<&str> {
        self.driver_name.as_deref()
    }

    pub fn add_capability(&mut self, cap: u64) {
        if !self.capabilities.contains(&cap) {
            self.capabilities.push(cap);
        }
    }

    pub fn has_capability(&self, cap: u64) -> bool {
        self.capabilities.contains(&cap)
    }
}

// ==========================================
// Windows NT-Style Object Manager Subsystem
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtObjectType {
    Directory,
    Device,
    SymbolicLink,
    Driver,
    Section,
}

#[derive(Clone, Debug)]
pub struct NtObject {
    pub name: String,
    pub object_type: NtObjectType,
    pub target_path: Option<String>, // Symbolic link pointing to real object
}

#[derive(Clone)]
pub struct NtObjectDirectory {
    pub name: String,
    pub objects: HashMap<String, NtObject>,
    pub subdirectories: HashMap<String, NtObjectDirectory>,
}

pub struct NtObjectManager {
    pub root: NtObjectDirectory,
}

impl NtObjectManager {
    pub fn new() -> Self {
        let mut root = NtObjectDirectory {
            name: String::from("\\"),
            objects: HashMap::new(),
            subdirectories: HashMap::new(),
        };
        // Pre-populate standard Windows-style directories
        root.subdirectories.insert(
            String::from("Device"),
            NtObjectDirectory {
                name: String::from("Device"),
                objects: HashMap::new(),
                subdirectories: HashMap::new(),
            },
        );
        root.subdirectories.insert(
            String::from("DosDevices"),
            NtObjectDirectory {
                name: String::from("DosDevices"),
                objects: HashMap::new(),
                subdirectories: HashMap::new(),
            },
        );
        NtObjectManager { root }
    }

    /// Insert an object into the object manager namespace at a specific path (e.g. "\Device\Keyboard")
    pub fn insert_object(&mut self, path: &str, obj: NtObject) -> Result<(), &'static str> {
        let parts: Vec<&str> = path.split('\\').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            return Err("Invalid path");
        }

        let mut current_dir = &mut self.root;
        for i in 0..parts.len() - 1 {
            let part = parts[i];
            if !current_dir.subdirectories.contains_key(part) {
                current_dir.subdirectories.insert(
                    part.to_string(),
                    NtObjectDirectory {
                        name: part.to_string(),
                        objects: HashMap::new(),
                        subdirectories: HashMap::new(),
                    },
                );
            }
            current_dir = current_dir.subdirectories.get_mut(part).unwrap();
        }

        let name = parts.last().unwrap().to_string();
        current_dir.objects.insert(name, obj);
        Ok(())
    }

    /// Retrieve an object by its absolute path, resolving symbolic links/aliases recursively
    pub fn lookup_object(&self, path: &str) -> Option<NtObject> {
        let parts: Vec<&str> = path.split('\\').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            return None;
        }

        let mut current_dir = &self.root;
        for i in 0..parts.len() - 1 {
            let part = parts[i];
            current_dir = current_dir.subdirectories.get(part)?;
        }

        let name = *parts.last().unwrap();
        let obj = current_dir.objects.get(name)?;

        if obj.object_type == NtObjectType::SymbolicLink {
            if let Some(ref target) = obj.target_path {
                return self.lookup_object(target);
            }
        }
        Some(obj.clone())
    }
}

impl Default for NtObjectManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// Windows-inspired Non-Paged Pool Memory & Driver Loading Subsystem
// ==========================================

pub struct NonPagedPoolMemory {
    pub total_bytes: usize,
    pub allocated_bytes: usize,
    pub allocations: HashMap<u64, usize>, // Addr -> size
    next_free_addr: u64,
}

impl NonPagedPoolMemory {
    pub fn new(capacity: usize) -> Self {
        NonPagedPoolMemory {
            total_bytes: capacity,
            allocated_bytes: 0,
            allocations: HashMap::new(),
            next_free_addr: 0xFFFF_C000_0000_0000, // Non-paged pool canonical base (x64)
        }
    }

    pub fn allocate(&mut self, size: usize) -> Result<u64, &'static str> {
        if self.allocated_bytes + size > self.total_bytes {
            return Err("OUT_OF_NON_PAGED_POOL_MEMORY");
        }
        let addr = self.next_free_addr;
        self.allocations.insert(addr, size);
        self.allocated_bytes += size;
        self.next_free_addr += size as u64;
        Ok(addr)
    }

    pub fn free(&mut self, addr: u64) -> Result<(), &'static str> {
        let size = self.allocations.remove(&addr).ok_or("Invalid memory address")?;
        self.allocated_bytes -= size;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverState {
    Loaded,
    Running,
    Unloaded,
}

pub struct DriverEntry {
    pub driver_name: String,
    pub registry_path: String,
    pub non_paged_pool_addr: u64,
    pub driver_size: usize,
    pub state: DriverState,
}

impl DriverEntry {
    pub fn new(name: &str, registry_path: &str, pool: &mut NonPagedPoolMemory, size: usize) -> Result<Self, &'static str> {
        let addr = pool.allocate(size)?;
        Ok(DriverEntry {
            driver_name: name.to_string(),
            registry_path: registry_path.to_string(),
            non_paged_pool_addr: addr,
            driver_size: size,
            state: DriverState::Loaded,
        })
    }

    pub fn start(&mut self) {
        self.state = DriverState::Running;
    }

    pub fn unload(&mut self, pool: &mut NonPagedPoolMemory) -> Result<(), &'static str> {
        if self.state == DriverState::Unloaded {
            return Err("Driver already unloaded");
        }
        pool.free(self.non_paged_pool_addr)?;
        self.state = DriverState::Unloaded;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nt_object_manager_directories_and_symlinks() {
        let mut manager = NtObjectManager::new();

        // 1. Create a real device object and insert it into \Device\Keyboard
        let keyboard_dev = NtObject {
            name: String::from("Keyboard"),
            object_type: NtObjectType::Device,
            target_path: None,
        };
        manager.insert_object("\\Device\\Keyboard", keyboard_dev).unwrap();

        // 2. Create a symbolic link in \DosDevices\KeyboardAlias pointing to \Device\Keyboard
        let keyboard_link = NtObject {
            name: String::from("KeyboardAlias"),
            object_type: NtObjectType::SymbolicLink,
            target_path: Some(String::from("\\Device\\Keyboard")),
        };
        manager.insert_object("\\DosDevices\\KeyboardAlias", keyboard_link).unwrap();

        // 3. Look up \DosDevices\KeyboardAlias and verify it resolves to the real Keyboard Device object
        let resolved = manager.lookup_object("\\DosDevices\\KeyboardAlias").unwrap();
        assert_eq!(resolved.name, "Keyboard");
        assert_eq!(resolved.object_type, NtObjectType::Device);
    }

    #[test]
    fn test_non_paged_pool_allocation() {
        let mut pool = NonPagedPoolMemory::new(1024);
        assert_eq!(pool.allocated_bytes, 0);

        let addr1 = pool.allocate(256).unwrap();
        assert_eq!(addr1, 0xFFFF_C000_0000_0000);
        assert_eq!(pool.allocated_bytes, 256);

        let addr2 = pool.allocate(512).unwrap();
        assert_eq!(addr2, 0xFFFF_C000_0000_0100);
        assert_eq!(pool.allocated_bytes, 768);

        // Allocating beyond capacity should fail
        assert!(pool.allocate(512).is_err());

        // Free allocation
        pool.free(addr1).unwrap();
        assert_eq!(pool.allocated_bytes, 512);
    }

    #[test]
    fn test_driver_entry_and_dynamic_unloading() {
        let mut pool = NonPagedPoolMemory::new(4096);
        let mut driver = DriverEntry::new(
            "AcpiBattery",
            "\\Registry\\Machine\\System\\CurrentControlSet\\Services\\AcpiBattery",
            &mut pool,
            2048,
        ).unwrap();

        assert_eq!(driver.state, DriverState::Loaded);
        assert_eq!(driver.non_paged_pool_addr, 0xFFFF_C000_0000_0000);

        driver.start();
        assert_eq!(driver.state, DriverState::Running);

        driver.unload(&mut pool).unwrap();
        assert_eq!(driver.state, DriverState::Unloaded);
        assert_eq!(pool.allocated_bytes, 0);
    }
}
