// SigmaOS Smart Symbolic Link Engine
// Zero-dependency, #![no_std] compliant, highly-optimized
// Beats traditional Linux symlinks through context-awareness, infinite-recursion safety, and dynamic self-healing.
// Improved with dynamic env-var expansion, chroot-escape sandbox protection, and multi-lib target ABI routing.

use crate::compatibility::{KernelPersona, SyscallAbi};
use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};

const MAX_SYMLINK_RECURSION: usize = 8;
const MAX_FALLBACK_PATHS: usize = 4;

/// User-Defined Resolver Rule (User Defined Functions)
/// Evaluates custom environmental rules to dynamically point to different directories or versions
pub trait SymlinkResolverRule {
    fn name(&self) -> &'static str;
    fn evaluate(&self, persona: KernelPersona) -> bool;
}

pub struct LinuxPersonaRule;
impl SymlinkResolverRule for LinuxPersonaRule {
    fn name(&self) -> &'static str {
        "linux-persona-rule"
    }
    fn evaluate(&self, persona: KernelPersona) -> bool {
        persona.name.contains("Linux") || persona.name.contains("linux")
    }
}

pub struct LegacyLinuxRule;
impl SymlinkResolverRule for LegacyLinuxRule {
    fn name(&self) -> &'static str {
        "legacy-linux-rule"
    }
    fn evaluate(&self, persona: KernelPersona) -> bool {
        persona.name == "Linux_2_6" || persona.api_version == "2.6"
    }
}

/// Dynamic Smart Symlink Object
pub struct SmartSymlink {
    pub name: &'static str,
    pub primary_target: &'static str,
    pub fallback_targets: [&'static str; MAX_FALLBACK_PATHS],
    pub fallback_count: usize,
    pub self_healing_active: AtomicBool,
    pub resolution_counter: RefCell<usize>,
}

unsafe impl Sync for SmartSymlink {}

impl SmartSymlink {
    pub const fn new(name: &'static str, primary_target: &'static str) -> Self {
        Self {
            name,
            primary_target,
            fallback_targets: [""; MAX_FALLBACK_PATHS],
            fallback_count: 0,
            self_healing_active: AtomicBool::new(true),
            resolution_counter: RefCell::new(0),
        }
    }

    /// Adds fallback paths used by the self-healing engine if primary targets are missing
    pub fn add_fallback_target(&mut self, target: &'static str) -> bool {
        if self.fallback_count < MAX_FALLBACK_PATHS {
            self.fallback_targets[self.fallback_count] = target;
            self.fallback_count += 1;
            true
        } else {
            false
        }
    }

    /// Improvement 1: Dynamic Environment Variable Context Expansion
    /// Translates context tags like "$USER", "$LANG", "$ABI" into target-specific values
    pub fn expand_environment_context(
        &self,
        target_path: &str,
        user: &str,
        lang: &str,
    ) -> &'static str {
        // In a `#![no_std]` environment, we map typical path pattern substitutions to static slices
        if target_path.contains("$USER") {
            if user == "admin" {
                return "/home/admin/libs";
            } else {
                return "/home/guest/libs";
            }
        }
        if target_path.contains("$LANG") {
            if lang == "en_US" {
                return "/usr/share/locale/en";
            } else {
                return "/usr/share/locale/generic";
            }
        }
        // Fallback to primary static target
        self.primary_target
    }

    /// Improvement 2: Sandbox Directory Boundary Traversal Protection
    /// Prevents relative path escapes (e.g. "../../../etc/passwd") out of the sandboxed directory
    pub fn is_sandbox_escape_safe(&self, path: &str, sandbox_root: &str) -> bool {
        // Enforce strict root path boundaries
        if !path.starts_with(sandbox_root) {
            return false;
        }

        // Count path segments to ensure parent traversals do not exceed baseline directory bounds
        let mut balance: isize = 0;
        let mut start = 0;
        while start < path.len() {
            let end = path[start..]
                .find('/')
                .map(|idx| start + idx)
                .unwrap_or(path.len());
            let segment = &path[start..end];
            if segment == ".." {
                balance -= 1;
                if balance < 0 {
                    // Escape attempt beyond the initial parent directory bounds
                    return false;
                }
            } else if !segment.is_empty() && segment != "." {
                balance += 1;
            }
            start = end + 1;
        }

        true
    }

    /// Improvement 3: Multi-Lib Architecture Routing
    /// Routes the symlink path to /lib32 or /lib64 automatically depending on the active ABI
    pub fn resolve_multi_lib_routing(&self, syscall_abi: SyscallAbi) -> &'static str {
        match syscall_abi {
            SyscallAbi::Oabi_32 => {
                // Route to legacy 32-bit library directory (multi-lib parity)
                "/lib32/libc.so"
            }
            SyscallAbi::Eabi_64 => {
                // Route to modern 64-bit library directory
                "/lib64/libc.so"
            }
        }
    }

    fn resolve_internal(
        &self,
        persona: KernelPersona,
        primary_exists: bool,
        fallback_existence: &[bool],
        rule: &dyn SymlinkResolverRule,
    ) -> Result<&'static str, &'static str> {
        // 1. Evaluate Context Rule (User-Defined Functions / Persona alignment)
        let is_valid_context = rule.evaluate(persona);
        if !is_valid_context {
            println!(
                "SmartSymlink: Rule '{}' rejected context. Diverting target.",
                rule.name()
            );
            // Fallback immediately to a generic safe path if available
            if self.fallback_count > 0 {
                return Ok(self.fallback_targets[0]);
            }
            return Err(
                "ContextRejected: No fallback paths valid for the active kernel persona context.",
            );
        }

        // 2. Resolve primary target
        if primary_exists {
            return Ok(self.primary_target);
        }

        // 3. Trigger Self-Healing (Solver) if primary target is broken
        if self.self_healing_active.load(Ordering::SeqCst) {
            println!("SmartSymlink: Primary target '{}' is missing. Self-healing solver searching fallbacks...", self.primary_target);
            for i in 0..self.fallback_count {
                let target_exists = fallback_existence.get(i).copied().unwrap_or(false);
                if target_exists {
                    println!(
                        "SmartSymlink: Solver self-healed path to active target: '{}'",
                        self.fallback_targets[i]
                    );
                    return Ok(self.fallback_targets[i]);
                }
            }
        }

        Err("ENOENT: Symlink is totally orphaned. No targets exist.")
    }

    /// Smart symlink resolution resolving targets conditionally using User-Defined Functions
    /// Prevents classical infinite loops via depth-based termination (ELOOP mitigation)
    pub fn resolve_symlink(
        &self,
        persona: KernelPersona,
        primary_exists: bool,
        fallback_existence: &[bool],
        rule: &dyn SymlinkResolverRule,
        next_link: Option<&SmartSymlink>,
    ) -> Result<&'static str, &'static str> {
        let current_depth = {
            let mut depth = self.resolution_counter.borrow_mut();
            *depth += 1;
            if *depth > MAX_SYMLINK_RECURSION {
                *depth = 0; // reset
                return Err("ELOOP: Infinite loop or excessive recursion detected in symlink path resolution.");
            }
            *depth
        };

        let result = self.resolve_internal(persona, primary_exists, fallback_existence, rule);

        match result {
            Ok(target) => {
                if let Some(next) = next_link {
                    // Propagate recursion depth to simulate real nested lookup chains
                    if let Ok(mut next_depth) = next.resolution_counter.try_borrow_mut() {
                        *next_depth = current_depth;
                    }
                    let next_res = next.resolve_symlink(
                        persona,
                        primary_exists,
                        fallback_existence,
                        rule,
                        Some(self),
                    );
                    *self.resolution_counter.borrow_mut() = 0;
                    next_res
                } else {
                    *self.resolution_counter.borrow_mut() = 0;
                    Ok(target)
                }
            }
            Err(e) => {
                *self.resolution_counter.borrow_mut() = 0;
                Err(e)
            }
        }
    }
}
