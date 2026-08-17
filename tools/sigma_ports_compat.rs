// SPDX-License-Identifier: Apache-2.0
//! SigmaOS FreeBSD-style Ports Compatibility Tool
//! Safe, zero-dependency, `#![no_std]` compliant utility

#![no_std]

pub struct PortPackage {
    pub name: &'static str,
    pub version: &'static str,
    pub path: &'static str,
    pub compiled: bool,
}

pub struct PortsTree {
    pub ports: [Option<PortPackage>; 8],
    pub installation_count: usize,
}

impl PortsTree {
    pub fn new() -> Self {
        Self {
            ports: [None, None, None, None, None, None, None, None],
            installation_count: 0,
        }
    }

    pub fn add_port(&mut self, name: &'static str, version: &'static str, path: &'static str) -> bool {
        for slot in self.ports.iter_mut() {
            if slot.is_none() {
                *slot = Some(PortPackage {
                    name,
                    version,
                    path,
                    compiled: false,
                });
                return true;
            }
        }
        false
    }

    pub fn compile_and_install(&mut self, name: &'static str) -> bool {
        for port in self.ports.iter_mut().flatten() {
            if port.name == name {
                port.compiled = true;
                self.installation_count += 1;
                return true;
            }
        }
        false
    }
}

impl Default for PortsTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ports_tree_build() {
        let mut tree = PortsTree::new();
        tree.add_port("nginx", "1.21.0", "/usr/ports/www/nginx");

        assert_eq!(tree.installation_count, 0);
        assert!(tree.compile_and_install("nginx"));
        assert_eq!(tree.installation_count, 1);
    }
}
