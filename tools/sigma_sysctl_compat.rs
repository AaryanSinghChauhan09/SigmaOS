//! SigmaOS Sysctl Compatibility & Dynamic Kernel Parameter Tuner
//! Implements runtime query and modification of kernel and VM tuning parameters.
//! Inspired by Linux sysctl and BSD sysctl utilities.

#![no_std]
#![allow(dead_code)]

#[cfg(not(target_os = "none"))]
extern crate alloc;
#[cfg(not(target_os = "none"))]
use alloc::string::{String, ToString};
#[cfg(not(target_os = "none"))]
use alloc::vec::Vec;
#[cfg(not(target_os = "none"))]
use alloc::format;

/// Represents a single sysctl kernel parameter
#[derive(Debug, Clone)]
pub struct SysctlParameter {
    pub name: String,
    pub value: String,
    pub description: &'static str,
    pub read_only: bool,
}

/// Dynamic sysctl manager engine
pub struct SysctlEngine {
    pub parameters: Vec<SysctlParameter>,
}

impl Default for SysctlEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SysctlEngine {
    pub fn new() -> Self {
        let mut params = Vec::new();

        // 1. VM/memory parameters
        params.push(SysctlParameter {
            name: "vm.swappiness".to_string(),
            value: "60".to_string(),
            description: "Control relative weight given to page cache vs swap memory.",
            read_only: false,
        });

        params.push(SysctlParameter {
            name: "vm.max_map_count".to_string(),
            value: "65530".to_string(),
            description: "Maximum number of memory map areas a process may have.",
            read_only: false,
        });

        // 2. Kernel parameters
        params.push(SysctlParameter {
            name: "kernel.panic_on_oops".to_string(),
            value: "1".to_string(),
            description: "Determine whether the kernel panics on oops condition.",
            read_only: false,
        });

        params.push(SysctlParameter {
            name: "kernel.version".to_string(),
            value: "6.5.6-sigma-pqc".to_string(),
            description: "The compiled microkernel release version.",
            read_only: true, // Immutable system string
        });

        // 3. Network parameters
        params.push(SysctlParameter {
            name: "net.ipv4.ip_forward".to_string(),
            value: "0".to_string(),
            description: "Enable or disable forwarding of IPv4 network packets.",
            read_only: false,
        });

        // 4. Filesystem parameters
        params.push(SysctlParameter {
            name: "fs.file-max".to_string(),
            value: "100000".to_string(),
            description: "The maximum number of open file descriptors allowed.",
            read_only: false,
        });

        SysctlEngine {
            parameters: params,
        }
    }

    /// Read the string value of a kernel parameter
    pub fn read_param(&self, name: &str) -> Result<String, &'static str> {
        for param in self.parameters.iter() {
            if param.name == name {
                return Ok(param.value.clone());
            }
        }
        Err("Sysctl: Specified kernel parameter not found.")
    }

    /// Dynamically write/tune a kernel parameter value at runtime
    pub fn write_param(&mut self, name: &str, new_value: &str) -> Result<(), &'static str> {
        for param in self.parameters.iter_mut() {
            if param.name == name {
                if param.read_only {
                    return Err("Sysctl: Cannot modify read-only kernel parameters.");
                }
                param.value = new_value.to_string();
                return Ok(());
            }
        }
        Err("Sysctl: Parameter not found.")
    }

    /// Lists all parameters in the sysctl catalog (e.g. sysctl -a)
    pub fn list_all_parameters(&self) -> Vec<String> {
        let mut list = Vec::new();
        for param in self.parameters.iter() {
            list.push(format!("{} = {}", param.name, param.value));
        }
        list
    }
}

// Allocator shim for no_std environments
#[cfg(target_os = "none")]
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

#[cfg(target_os = "none")]
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
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
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

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysctl_read_write_operations() {
        let mut engine = SysctlEngine::new();

        // 1. Read default values
        assert_eq!(engine.read_param("vm.swappiness").unwrap(), "60");
        assert_eq!(engine.read_param("net.ipv4.ip_forward").unwrap(), "0");

        // 2. Tune parameters at runtime
        engine.write_param("vm.swappiness", "10").unwrap();
        engine.write_param("net.ipv4.ip_forward", "1").unwrap();

        assert_eq!(engine.read_param("vm.swappiness").unwrap(), "10");
        assert_eq!(engine.read_param("net.ipv4.ip_forward").unwrap(), "1");
    }

    #[test]
    fn test_sysctl_immutable_parameter_protection() {
        let mut engine = SysctlEngine::new();

        // Reading is successful
        assert_eq!(engine.read_param("kernel.version").unwrap(), "6.5.6-sigma-pqc");

        // Attempting to write a read-only variable must yield PermissionDenied / Error
        let res = engine.write_param("kernel.version", "9.9.9");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Sysctl: Cannot modify read-only kernel parameters.");
    }

    #[test]
    fn test_sysctl_parameter_enumeration() {
        let engine = SysctlEngine::new();
        let all = engine.list_all_parameters();
        assert_eq!(all.len(), 6);
        assert!(all[0].contains("vm.swappiness = 60"));
    }
}
