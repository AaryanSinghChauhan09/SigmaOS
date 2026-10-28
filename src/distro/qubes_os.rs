// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - Qubes OS security domain manager
// Principles: PROCESS ISOLATION, LEAST PRIVILEGE, SECURITY DOMAINS, DEFENSIVE PROGRAMMING
use alloc::{collections::BTreeMap, string::String, vec::Vec};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityDomain {
    Vault = 0,
    Green = 1,
    Yellow = 2,
    Blue = 3,
    Red = 4,
}
impl SecurityDomain {
    pub fn color(&self) -> u32 {
        match self {
            Self::Red => 0xFF0000,
            Self::Yellow => 0xFFFF00,
            Self::Green => 0x00FF00,
            Self::Blue => 0x0000FF,
            Self::Vault => 0x808080,
        }
    }
    pub fn label(&self) -> &'static str {
        match self {
            Self::Red => "Untrusted",
            Self::Yellow => "Personal",
            Self::Green => "Work",
            Self::Blue => "Banking",
            Self::Vault => "Vault",
        }
    }
}
#[derive(Debug, Clone)]
pub struct QubeVm {
    pub name: String,
    pub domain: SecurityDomain,
    pub disposable: bool,
    pub mem_mb: u32,
    pub vcpus: u8,
}
pub struct QubesManager {
    pub qubes: BTreeMap<String, QubeVm>,
    pub next_disposable_id: u32,
}
impl QubesManager {
    pub fn new() -> Self {
        Self {
            qubes: BTreeMap::new(),
            next_disposable_id: 1,
        }
    }
    pub fn create_qube(
        &mut self,
        name: &str,
        domain: SecurityDomain,
        mem_mb: u32,
    ) -> Result<(), &'static str> {
        if self.qubes.contains_key(name) {
            return Err("qube already exists");
        }
        self.qubes.insert(
            name.into(),
            QubeVm {
                name: name.into(),
                domain,
                disposable: false,
                mem_mb,
                vcpus: 2,
            },
        );
        Ok(())
    }
    pub fn create_disposable_vm(&mut self, template: &str) -> Result<String, &'static str> {
        let base = self.qubes.get(template).ok_or("template not found")?;
        let name = alloc::format!("disp{}", self.next_disposable_id);
        self.next_disposable_id += 1;
        let domain = base.domain;
        let mem = base.mem_mb;
        self.qubes.insert(
            name.clone(),
            QubeVm {
                name: name.clone(),
                domain,
                disposable: true,
                mem_mb: mem,
                vcpus: 1,
            },
        );
        Ok(name)
    }
    pub fn destroy_qube(&mut self, name: &str) -> Result<(), &'static str> {
        self.qubes.remove(name).ok_or("not found").map(|_| ())
    }
    pub fn get_border_color(&self, name: &str) -> Option<u32> {
        self.qubes.get(name).map(|q| q.domain.color())
    }
    pub fn send_file(&self, from: &str, to: &str, _path: &str) -> Result<(), &'static str> {
        let qf = self.qubes.get(from).ok_or("source not found")?;
        let qt = self.qubes.get(to).ok_or("dest not found")?;
        if qf.domain > qt.domain {
            return Err("cannot send from higher to lower trust domain");
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_destroy() {
        let mut m = QubesManager::new();
        m.create_qube("personal", SecurityDomain::Yellow, 2048)
            .unwrap();
        assert!(m.qubes.contains_key("personal"));
        m.destroy_qube("personal").unwrap();
        assert!(m.qubes.is_empty());
    }
    #[test]
    fn test_disposable_vm() {
        let mut m = QubesManager::new();
        m.create_qube("template", SecurityDomain::Red, 1024)
            .unwrap();
        let disp = m.create_disposable_vm("template").unwrap();
        assert!(m.qubes[&disp].disposable);
    }
    #[test]
    fn test_border_color() {
        let mut m = QubesManager::new();
        m.create_qube("vault", SecurityDomain::Vault, 512).unwrap();
        assert_eq!(m.get_border_color("vault"), Some(0x808080));
    }
}
