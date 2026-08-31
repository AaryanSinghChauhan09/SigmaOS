use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
// BSD-style sysctl interface for dynamic kernel parameters.
// Supports safe querying and mutation of kernel variables under hierarchical MIB nodes.

use crate::klib::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SysctlValue {
    Int(i32),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct SysctlNode {
    pub value: SysctlValue,
    pub description: String,
    pub writable: bool,
}

pub struct SysctlRegistry {
    pub nodes: HashMap<String, SysctlNode>,
}

impl SysctlRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            nodes: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    fn register_defaults(&mut self) {
        self.register(
            "kern.ostype",
            SysctlValue::String("SigmaOS".to_string()),
            "Operating system type",
            false,
        );
        self.register(
            "kern.osrelease",
            SysctlValue::String("0.1.0".to_string()),
            "Operating system release version",
            false,
        );
        self.register(
            "vm.swappiness",
            SysctlValue::Int(60),
            "Kernel swappiness threshold level",
            true,
        );
        self.register(
            "net.inet.ip.forwarding",
            SysctlValue::Bool(false),
            "Enable IP packet forwarding inside kernel routing table",
            true,
        );
        self.register(
            "kern.maxproc",
            SysctlValue::Int(1024),
            "Maximum number of concurrent system processes",
            true,
        );
    }

    pub fn register(&mut self, mib: &str, value: SysctlValue, description: &str, writable: bool) {
        self.nodes.insert(
            mib.to_string(),
            SysctlNode {
                value,
                description: description.to_string(),
                writable,
            },
        );
    }

    pub fn get(&self, mib: &str) -> Option<&SysctlValue> {
        self.nodes.get(mib).map(|node| &node.value)
    }

    pub fn set(&mut self, mib: &str, new_value: SysctlValue) -> Result<(), &'static str> {
        if let Some(node) = self.nodes.get_mut(mib) {
            if !node.writable {
                return Err("Parameter is read-only!");
            }

            // Ensure type matches
            match (&node.value, &new_value) {
                (SysctlValue::Int(_), SysctlValue::Int(v)) => {
                    let val = *v;
                    if val < 0 && mib == "vm.swappiness" {
                        return Err("Swappiness cannot be negative!");
                    }
                    node.value = SysctlValue::Int(val);
                }
                (SysctlValue::String(_), SysctlValue::String(_)) => {
                    node.value = new_value;
                }
                (SysctlValue::Bool(_), SysctlValue::Bool(_)) => {
                    node.value = new_value;
                }
                _ => return Err("Type mismatch for sysctl parameter!"),
            }
            Ok(())
        } else {
            Err("Sysctl parameter not found!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysctl_defaults() {
        let registry = SysctlRegistry::new();
        assert_eq!(
            registry.get("kern.ostype"),
            Some(&SysctlValue::String("SigmaOS".to_string()))
        );
        assert_eq!(registry.get("vm.swappiness"), Some(&SysctlValue::Int(60)));
        assert_eq!(
            registry.get("net.inet.ip.forwarding"),
            Some(&SysctlValue::Bool(false))
        );
    }

    #[test]
    fn test_sysctl_read_only() {
        let mut registry = SysctlRegistry::new();
        let result = registry.set("kern.ostype", SysctlValue::String("Linux".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_sysctl_write_success() {
        let mut registry = SysctlRegistry::new();
        let result = registry.set("vm.swappiness", SysctlValue::Int(10));
        assert!(result.is_ok());
        assert_eq!(registry.get("vm.swappiness"), Some(&SysctlValue::Int(10)));
    }

    #[test]
    fn test_sysctl_invalid_inputs() {
        let mut registry = SysctlRegistry::new();
        // Negative swappiness validation
        let result = registry.set("vm.swappiness", SysctlValue::Int(-5));
        assert_eq!(result, Err("Swappiness cannot be negative!"));

        // Type mismatch validation
        let result = registry.set("vm.swappiness", SysctlValue::Bool(true));
        assert_eq!(result, Err("Type mismatch for sysctl parameter!"));
    }
}
