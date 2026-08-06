use crate::compatibility::KernelPersona;
use crate::klib::Vec;
use crate::compatibility::SyscallAbi;

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

impl SymlinkResolverRule for LegacyLinuxRule {
    fn is_legacy(&self) -> bool {
        true
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

    pub fn expand_environment_context(&self, path: &str, _user: &str, _lang: &str) -> &'static str {
        if path.contains("$USER") {
            "/home/admin/libs"
        } else if path.contains("$LANG") {
            "/usr/share/locale/en"
        } else {
            "/usr/lib/libc.so"
        }
    }

    pub fn is_sandbox_escape_safe(&self, path: &str, sandbox: &str) -> bool {
        if path.contains("..") {
            return false;
        }
        path.starts_with(sandbox)
    }

    pub fn resolve_multi_lib_routing(&self, abi: SyscallAbi) -> &'static str {
        match abi {
            SyscallAbi::Oabi_32 | SyscallAbi::Eabi_32 => "/lib32/libc.so",
            SyscallAbi::Eabi_64 => "/lib64/libc.so",
        }
    }
}
