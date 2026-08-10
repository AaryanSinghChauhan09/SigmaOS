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

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectError {
    InitFailed,
    RegisterFailed,
    NotFound,
    AlreadyRegistered,
    CapabilityDenied,
}

pub struct KRef {
    count: AtomicUsize,
}

impl KRef {
    #[allow(clippy::new_without_default)]
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
    base: KObject,
    device_id: u16,
    vendor_id: u16,
    device_type: String,
    driver_name: Option<String>,
    capabilities: Vec<u64>,
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
