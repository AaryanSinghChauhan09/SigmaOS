// SPDX-License-Identifier: MIT
#![cfg_attr(target_os = "none", no_std)]
#![allow(dead_code, non_snake_case)]

// SigmaOS: Σ SigmaOS — sigma_make: Sovereign Build System
// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.
// All types hand-defined. OOP via struct + impl + trait patterns.

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Module: StaticVec ──────────────────────────────────────────────────────

#[derive(Copy, Clone)]
pub struct StaticVec<T: Copy, const N: usize> {
    data: [Option<T>; N],
    len: usize,
}

impl<T: Copy, const N: usize> Default for StaticVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy, const N: usize> StaticVec<T, N> {
    pub const fn new() -> Self {
        Self {
            data: [None; N],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.len >= N {
            return Err("StaticVec is full");
        }
        self.data[self.len] = Some(item);
        self.len += 1;
        Ok(())
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len {
            self.data[idx].as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx < self.len {
            self.data[idx].as_mut()
        } else {
            None
        }
    }
}

// ─── Module: SigmaOS::sigma_make ────────────────────────────────────────────

/// SigmaTarget — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SigmaTarget {
    pub name: [u8; 32],
    pub dep_count: SigmaI32,
    pub command: [u8; 256],
    pub is_built: SigmaBool,
}

impl SigmaTarget {
    pub fn new(name_str: &[u8], command_str: &[u8]) -> Self {
        let mut name = [0u8; 32];
        let name_len = name_str.len().min(31);
        name[..name_len].copy_from_slice(&name_str[..name_len]);

        let mut command = [0u8; 256];
        let cmd_len = command_str.len().min(255);
        command[..cmd_len].copy_from_slice(&command_str[..cmd_len]);

        Self {
            name,
            dep_count: 0,
            command,
            is_built: false,
        }
    }

    pub fn name_as_str(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        &self.name[..len]
    }
}

/// The main compilation execution and dependency tracking engine for sigma_make
pub struct SigmaMakeEngine {
    pub targets: StaticVec<SigmaTarget, 16>,
    /// Tuple of (parent_target_idx, child_target_idx)
    pub dependencies: StaticVec<(SigmaUsize, SigmaUsize), 32>,
}

impl Default for SigmaMakeEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaMakeEngine {
    pub const fn new() -> Self {
        Self {
            targets: StaticVec::new(),
            dependencies: StaticVec::new(),
        }
    }

    /// Register a build target (e.g. "kernel_elf", "driver_block_o")
    pub fn register_target(&mut self, name: &[u8], command: &[u8]) -> Result<usize, &'static str> {
        let target = SigmaTarget::new(name, command);
        self.targets.push(target)?;
        Ok(self.targets.len() - 1)
    }

    /// Set a compilation dependency relation (parent target depends on child target)
    pub fn add_dependency(
        &mut self,
        parent_idx: usize,
        child_idx: usize,
    ) -> Result<(), &'static str> {
        self.dependencies.push((parent_idx, child_idx))?;

        // Update parent target's dep count
        if let Some(target) = self.targets.get_mut(parent_idx) {
            target.dep_count += 1;
        }

        Ok(())
    }

    /// Triggers recursive dependency build compilation
    pub fn build_target(&mut self, target_idx: usize) -> Result<(), &'static str> {
        // 1. Build all child dependencies first (recursive resolution)
        for i in 0..self.dependencies.len() {
            if let Some(&(parent, child)) = self.dependencies.get(i) {
                if parent == target_idx {
                    // Recurse to compile child first
                    self.build_target(child)?;
                }
            }
        }

        // 2. Mark this target as built
        if let Some(target) = self.targets.get_mut(target_idx) {
            target.is_built = true;
        }

        Ok(())
    }
}

/// # Safety
/// Caller must ensure `dest` slice has sufficient allocated capacity.
pub unsafe fn str_copy_slice(src: &[u8], dest: &mut [u8]) {
    let len = src.len().min(dest.len());
    dest[..len].copy_from_slice(&src[..len]);
}

static mut GLOBAL_MAKE: SigmaMakeEngine = SigmaMakeEngine::new();

/// # Safety
/// Empty C-ABI string copy symbol helper.
#[no_mangle]
pub unsafe extern "C" fn str_copy() {}

/// # Safety
/// Caller must ensure single-threaded execution when modifying global make engine static.
#[no_mangle]
pub unsafe extern "C" fn sigma_make_register_c_target() {
    unsafe {
        let _ = (&mut *core::ptr::addr_of_mut!(GLOBAL_MAKE))
            .register_target(b"c_target", b"gcc c_target.c -o c_target");
    }
}

fn main() {}

// ─── Module: Static Unit Tests ──────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_dependency_resolved_compiles() {
        let mut make = SigmaMakeEngine::new();

        // 1. Register build targets
        let driver_o = make
            .register_target(b"driver_block.o", b"rustc --emit=obj driver_block.rs")
            .unwrap();
        let kernel_o = make
            .register_target(b"kernel.o", b"rustc --emit=obj kernel.rs")
            .unwrap();
        let kernel_elf = make
            .register_target(b"kernel.elf", b"ld kernel.o driver_block.o -o kernel.elf")
            .unwrap();

        // 2. Setup dependency relationships
        // kernel_elf depends on kernel_o and driver_o
        make.add_dependency(kernel_elf, kernel_o).unwrap();
        make.add_dependency(kernel_elf, driver_o).unwrap();

        assert_eq!(make.targets.get(kernel_elf).unwrap().dep_count, 2);
        assert!(!make.targets.get(kernel_elf).unwrap().is_built);
        assert!(!make.targets.get(kernel_o).unwrap().is_built);
        assert!(!make.targets.get(driver_o).unwrap().is_built);

        // 3. Compile/Build parent target
        make.build_target(kernel_elf).unwrap();

        // All targets must be fully built now
        assert!(make.targets.get(kernel_elf).unwrap().is_built);
        assert!(make.targets.get(kernel_o).unwrap().is_built);
        assert!(make.targets.get(driver_o).unwrap().is_built);
    }
}
