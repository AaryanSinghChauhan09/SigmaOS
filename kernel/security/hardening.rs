// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/security/hardening.rs — Kernel Hardening Features
//
// This module implements kernel hardening features inspired by Linux kernel hardening
// options (CONFIG_HARDENED_USERCOPY, CONFIG_FORTIFY_SOURCE, CONFIG_STACKPROTECTOR, etc.)
// to provide defense-in-depth security.
//
// Key features:
// - Stack protection (canaries, overflow detection)
// - Usercopy hardening
// - Memory randomization (ASLR)
// - Control flow integrity
// - OOP principles with hardening traits
// - No external dependencies

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// ─────────────────────────────────────────────────────────────────────────────
// Hardening Flags
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct HardeningFlags {
    pub stack_protector: bool,
    pub usercopy_hardening: bool,
    pub aslr_enabled: bool,
    pub cfi_enabled: bool,
    pub slab_hardening: bool,
    pub refcount_hardening: bool,
    pub integer_overflow_hardening: bool,
    pub bounds_checking: bool,
}

impl HardeningFlags {
    pub const fn default() -> Self {
        Self {
            stack_protector: true,
            usercopy_hardening: true,
            aslr_enabled: true,
            cfi_enabled: true,
            slab_hardening: true,
            refcount_hardening: true,
            integer_overflow_hardening: true,
            bounds_checking: true,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Stack Canary
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct StackCanary {
    pub value: u64,
    pub initialized: bool,
}

impl StackCanary {
    pub const fn new() -> Self {
        Self {
            value: 0,
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        // Generate random canary value (simplified)
        // In real implementation, would use hardware RNG
        self.value = 0xdeadbeefdeadbeef_u64;
        self.initialized = true;
    }

    pub fn check(&self, current: u64) -> bool {
        self.value == current
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ASLR Configuration
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct AslrConfig {
    pub enabled: bool,
    pub randomize_base: bool,
    pub randomize_heap: bool,
    pub randomize_stack: bool,
    pub randomize_mmap: bool,
    pub randomize PIE: bool,
}

impl AslrConfig {
    pub const fn default() -> Self {
        Self {
            enabled: true,
            randomize_base: true,
            randomize_heap: true,
            randomize_stack: true,
            randomize_mmap: true,
            randomize_PIE: true,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Memory Hardening Trait (OOP Principles)
// ─────────────────────────────────────────────────────────────────────────────

pub trait MemoryHardening {
    fn check_bounds(&self, ptr: usize, size: usize, limit: usize) -> bool;
    fn sanitize_usercopy(&self, src: usize, dst: usize, size: usize) -> bool;
    fn detect_overflow(&self, a: usize, b: usize) -> bool;
    fn randomize_address(&self, base: usize, range: usize) -> usize;
}

// ─────────────────────────────────────────────────────────────────────────────
// Default Memory Hardening Implementation
// ─────────────────────────────────────────────────────────────────────────────

pub struct DefaultMemoryHardening {
    flags: HardeningFlags,
    aslr_config: AslrConfig,
    stack_canary: StackCanary,
    random_seed: u64,
}

impl DefaultMemoryHardening {
    pub const fn new() -> Self {
        Self {
            flags: HardeningFlags::default(),
            aslr_config: AslrConfig::default(),
            stack_canary: StackCanary::new(),
            random_seed: 0,
        }
    }

    pub fn init(&mut self) {
        self.stack_canary.init();
        self.random_seed = self.generate_random_seed();
    }

    fn generate_random_seed(&self) -> u64 {
        // Simplified random seed generation
        // In real implementation, would use hardware RNG
        0x5a5a5a5a5a5a5a5a_u64
    }

    pub fn get_flags(&self) -> HardeningFlags {
        self.flags
    }

    pub fn set_flags(&mut self, flags: HardeningFlags) {
        self.flags = flags;
    }

    pub fn get_aslr_config(&self) -> AslrConfig {
        self.aslr_config
    }

    pub fn set_aslr_config(&mut self, config: AslrConfig) {
        self.aslr_config = config;
    }

    pub fn get_stack_canary(&self) -> StackCanary {
        self.stack_canary
    }
}

impl MemoryHardening for DefaultMemoryHardening {
    fn check_bounds(&self, ptr: usize, size: usize, limit: usize) -> bool {
        if !self.flags.bounds_checking {
            return true; // Skip if bounds checking disabled
        }
        
        // Check for overflow
        if self.detect_overflow(ptr, size) {
            return false;
        }
        
        // Check if within limit
        ptr + size <= limit
    }

    fn sanitize_usercopy(&self, src: usize, dst: usize, size: usize) -> bool {
        if !self.flags.usercopy_hardening {
            return true; // Skip if usercopy hardening disabled
        }
        
        // Check for null pointers
        if src == 0 || dst == 0 {
            return false;
        }
        
        // Check for overflow
        if self.detect_overflow(src, size) || self.detect_overflow(dst, size) {
            return false;
        }
        
        // Check for overlapping regions
        let src_end = src + size;
        let dst_end = dst + size;
        
        if (src >= dst && src < dst_end) || (dst >= src && dst < src_end) {
            return false;
        }
        
        true
    }

    fn detect_overflow(&self, a: usize, b: usize) -> bool {
        if !self.flags.integer_overflow_hardening {
            return false; // Skip if overflow checking disabled
        }
        
        a.checked_add(b).is_none()
    }

    fn randomize_address(&self, base: usize, range: usize) -> usize {
        if !self.aslr_enabled() {
            return base;
        }
        
        // Simple randomization (in real implementation, would use proper RNG)
        let offset = (self.random_seed % range as u64) as usize;
        base + offset
    }
}

impl DefaultMemoryHardening {
    fn aslr_enabled(&self) -> bool {
        self.aslr_config.enabled
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Control Flow Integrity
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct CfiState {
    pub enabled: bool,
    pub indirect_branch_checking: bool,
    pub return_address_checking: bool,
}

impl CfiState {
    pub const fn new() -> Self {
        Self {
            enabled: true,
            indirect_branch_checking: true,
            return_address_checking: true,
        }
    }

    pub fn check_indirect_branch(&self, target: usize, expected: usize) -> bool {
        if !self.enabled || !self.indirect_branch_checking {
            return true;
        }
        
        target == expected
    }

    pub fn check_return_address(&self, ret_addr: usize, expected: usize) -> bool {
        if !self.enabled || !self.return_address_checking {
            return true;
        }
        
        ret_addr == expected
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Refcount Hardening
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct RefcountHardening {
    pub enabled: bool,
    pub saturating: bool,
}

impl RefcountHardening {
    pub const fn new() -> SafeRefcount {
        Self {
            enabled: true,
            saturating: true,
        }
    }

    pub fn increment(&self, count: u32) -> u32 {
        if !self.enabled {
            return count.wrapping_add(1);
        }
        
        if self.saturating {
            count.saturating_add(1)
        } else {
            count.wrapping_add(1)
        }
    }

    pub fn decrement(&self, count: u32) -> u32 {
        if !self.enabled {
            return count.wrapping_sub(1);
        }
        
        if self.saturating {
            count.saturating_sub(1)
        } else {
            count.wrapping_sub(1)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Kernel Hardening Manager
// ─────────────────────────────────────────────────────────────────────────────

pub struct KernelHardeningManager {
    memory_hardening: DefaultMemoryHardening,
    cfi_state: CfiState,
    refcount_hardening: RefcountHardening,
    initialized: bool,
}

impl KernelHardeningManager {
    pub const fn new() -> Self {
        Self {
            memory_hardening: DefaultMemoryHardening::new(),
            cfi_state: CfiState::new(),
            refcount_hardening: RefcountHardening::new(),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.memory_hardening.init();
        self.initialized = true;
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn get_memory_hardening(&self) -> &DefaultMemoryHardening {
        &self.memory_hardening
    }

    pub fn get_memory_hardening_mut(&mut self) -> &mut DefaultMemoryHardening {
        &mut self.memory_hardening
    }

    pub fn get_cfi_state(&self) -> CfiState {
        self.cfi_state
    }

    pub fn set_cfi_state(&mut self, state: CfiState) {
        self.cfi_state = state;
    }

    pub fn get_refcount_hardening(&self) -> RefcountHardening {
        self.refcount_hardening
    }

    pub fn set_refcount_hardening(&mut self, hardening: RefcountHardening) {
        self.refcount_hardening = hardening;
    }

    // Convenience methods
    pub fn check_bounds(&self, ptr: usize, size: usize, limit: usize) -> bool {
        self.memory_hardening.check_bounds(ptr, size, limit)
    }

    pub fn sanitize_usercopy(&self, src: usize, dst: usize, size: usize) -> bool {
        self.memory_hardening.sanitize_usercopy(src, dst, size)
    }

    pub fn detect_overflow(&self, a: usize, b: usize) -> bool {
        self.memory_hardening.detect_overflow(a, b)
    }

    pub fn randomize_address(&self, base: usize, range: usize) -> usize {
        self.memory_hardening.randomize_address(base, range)
    }

    pub fn check_indirect_branch(&self, target: usize, expected: usize) -> bool {
        self.cfi_state.check_indirect_branch(target, expected)
    }

    pub fn check_return_address(&self, ret_addr: usize, expected: usize) -> bool {
        self.cfi_state.check_return_address(ret_addr, expected)
    }

    pub fn increment_refcount(&self, count: u32) -> u32 {
        self.refcount_hardening.increment(count)
    }

    pub fn decrement_refcount(&self, count: u32) -> u32 {
        self.refcount_hardening.decrement(count)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Global singleton
// ─────────────────────────────────────────────────────────────────────────────

static mut HARDENING_MANAGER: KernelHardeningManager = KernelHardeningManager::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_hardening_init() {
    HARDENING_MANAGER.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_hardening_check_bounds(ptr: usize, size: usize, limit: usize) -> bool {
    HARDENING_MANAGER.check_bounds(ptr, size, limit)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_hardening_sanitize_usercopy(src: usize, dst: usize, size: usize) -> bool {
    HARDENING_MANAGER.sanitize_usercopy(src, dst, size)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_hardening_detect_overflow(a: usize, b: usize) -> bool {
    HARDENING_MANAGER.detect_overflow(a, b)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_hardening_randomize_address(base: usize, range: usize) -> usize {
    HARDENING_MANAGER.randomize_address(base, range)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_hardening_check_indirect_branch(target: usize, expected: usize) -> bool {
    HARDENING_MANAGER.check_indirect_branch(target, expected)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_hardening_check_return_address(ret_addr: usize, expected: usize) -> bool {
    HARDENING_MANAGER.check_return_address(ret_addr, expected)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_hardening_increment_refcount(count: u32) -> u32 {
    HARDENING_MANAGER.increment_refcount(count)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_hardening_decrement_refcount(count: u32) -> u32 {
    HARDENING_MANAGER.decrement_refcount(count)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_hardening_set_stack_protector(enabled: bool) {
    let mut flags = HARDENING_MANAGER.get_memory_hardening_mut().get_flags();
    flags.stack_protector = enabled;
    HARDENING_MANAGER.get_memory_hardening_mut().set_flags(flags);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_hardening_set_aslr(enabled: bool) {
    let mut config = HARDENING_MANAGER.get_memory_hardening_mut().get_aslr_config();
    config.enabled = enabled;
    HARDENING_MANAGER.get_memory_hardening_mut().set_aslr_config(config);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_hardening_get_stack_canary() -> u64 {
    HARDENING_MANAGER.get_memory_hardening().get_stack_canary().value
}
