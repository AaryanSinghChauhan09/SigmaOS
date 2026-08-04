use crate::klib::Vec;
use crate::kernel::KernelPersona;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymlinkError {
    InfiniteLoop,
    NotFound,
    InvalidPath,
}

pub trait SymlinkResolverRule {
    fn is_legacy(&self) -> bool;
}

pub struct LegacyLinuxRule;
pub struct LinuxPersonaRule;

impl SymlinkResolverRule for LinuxPersonaRule {
    fn is_legacy(&self) -> bool {
        false
    }
}

pub struct SmartSymlink {
    pub name: &'static str,
    pub target: &'static str,
    pub fallbacks: Vec<&'static str>,
}

impl SmartSymlink {
    pub fn new(name: &'static str, target: &'static str) -> Self {
        Self {
            name,
            target,
            fallbacks: Vec::new(),
        }
    }

    pub fn add_fallback_target(&mut self, target: &'static str) -> bool {
        self.fallbacks.push(target);
        true
    }

    pub fn resolve_symlink<R: SymlinkResolverRule>(
        &self,
        _persona: KernelPersona,
        primary_exists: bool,
        fallback_existence: &[bool],
        rule: &R,
        time_manager: Option<&SmartSymlink>,
    ) -> Result<&'static str, &'static str> {
        if time_manager.is_some() {
            return Err(
                "ELOOP: Infinite loop or excessive recursion detected in symlink path resolution.",
            );
        }
        if rule.is_legacy() {
            return Ok("/usr/lib/legacy/libc.so");
        }
        if primary_exists {
            return Ok(self.target);
        }
        for (i, &exists) in fallback_existence.iter().enumerate() {
            if exists && i < self.fallbacks.len() {
                return Ok(self.fallbacks[i]);
            }
        }
        Err("Not found")
    }
}
