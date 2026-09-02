// SigmaOS Smart Symbolic Link Engine
// Zero-dependency, #![no_std] compliant, highly-optimized
// Beats traditional Linux symlinks through context-awareness, infinite-recursion safety, and dynamic self-healing.

use crate::compatibility::{KernelPersona, SyscallAbi};
use crate::klib::HashMap;
use alloc::string::{String, ToString};
use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};

const MAX_SYMLINK_RECURSION: usize = 8;
const MAX_FALLBACK_PATHS: usize = 4;

/// User-Defined Resolver Rule (User Defined Functions)
/// Evaluates custom environmental rules to dynamically point to different directories or versions
pub trait SymlinkResolverRule: Sync {
    fn name(&self) -> &'static str;
    fn evaluate(&self, persona: KernelPersona) -> bool;
}

pub struct LinuxPersonaRule;
impl SymlinkResolverRule for LinuxPersonaRule {
    fn name(&self) -> &'static str {
        "linux-persona-rule"
    }
    fn evaluate(&self, persona: KernelPersona) -> bool {
        persona.name().contains("Linux") || persona.name().contains("linux")
    }
}

pub struct LegacyLinuxRule;
impl SymlinkResolverRule for LegacyLinuxRule {
    fn name(&self) -> &'static str {
        "legacy-linux-rule"
    }
    fn evaluate(&self, persona: KernelPersona) -> bool {
        persona.name() == "linux_2_6" || persona.api_version() == 20006
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
    /// Internal environment-context map used to expand `$TOKEN` paths.
    env_map: HashMap<String, String>,
}

unsafe impl Sync for SmartSymlink {}

impl SmartSymlink {
    pub fn new(name: &'static str, primary_target: &'static str) -> Self {
        let mut env_map = HashMap::new();
        env_map.insert("USER".to_string(), "/home/{user}/libs".to_string());
        env_map.insert("LANG".to_string(), "/usr/share/locale/{lang}".to_string());
        Self {
            name,
            primary_target,
            fallback_targets: [""; MAX_FALLBACK_PATHS],
            fallback_count: 0,
            self_healing_active: AtomicBool::new(true),
            resolution_counter: RefCell::new(0),
            env_map,
        }
    }

    /// Resolves `$TOKEN` style environment variables embedded in a path using the
    /// internal context map. The `user` and `lang` parameters render the
    /// `{user}` and `{lang}` placeholders inside the mapped templates.
    pub fn expand_environment_context(&self, path: &str, user: &str, lang: &str) -> String {
        if let Some(dollar) = path.find('$') {
            let rest = &path[dollar + 1..];
            let end = rest
                .find(|c: char| !(c.is_alphanumeric() || c == '_'))
                .unwrap_or(rest.len());
            let token = &rest[..end];
            if let Some(template) = self.env_map.get(&token.to_string()) {
                let lang_prefix = lang.split('_').next().unwrap_or(lang);
                return template
                    .replace("{user}", user)
                    .replace("{lang}", lang_prefix);
            }
        }
        path.to_string()
    }

    /// Returns `true` only when `path` stays inside `sandbox_root` and does not
    /// attempt to traverse outside of it (rejects `..` and absolute escapes).
    pub fn is_sandbox_escape_safe(&self, path: &str, sandbox_root: &str) -> bool {
        if !path.starts_with(sandbox_root) {
            return false;
        }
        if path.contains("..") {
            return false;
        }
        true
    }

    /// Maps a target syscall ABI to its multiarch library routing path.
    pub fn resolve_multi_lib_routing(&self, abi: SyscallAbi) -> String {
        match abi {
            SyscallAbi::Oabi32 | SyscallAbi::Eabi32 => "/lib32/libc.so".to_string(),
            SyscallAbi::Eabi64 => "/lib64/libc.so".to_string(),
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

        let result =
            self.resolve_internal(persona.clone(), primary_exists, fallback_existence, rule);

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
