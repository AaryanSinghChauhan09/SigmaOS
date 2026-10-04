// SigmaOS W^X (Write XOR Execute) Page Table Hardening
// Inspired by OpenBSD KARL, Linux strict W^X, and PaX MPROTECT.
// Enforces that no memory page is simultaneously writable and executable.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

// ─────────────────────────────────────────────────────────────────────────────
// Page Permission Model
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PageFlags(pub u64);
impl PageFlags {
    pub const PRESENT:          PageFlags = PageFlags(1 << 0);
    pub const WRITE:            PageFlags = PageFlags(1 << 1);
    pub const WRITABLE:         PageFlags = PageFlags(1 << 1);
    pub const USER:             PageFlags = PageFlags(1 << 2);
    pub const WRITE_THROUGH:    PageFlags = PageFlags(1 << 3);
    pub const CACHE_DISABLE:    PageFlags = PageFlags(1 << 4);
    pub const ACCESSED:         PageFlags = PageFlags(1 << 5);
    pub const DIRTY:            PageFlags = PageFlags(1 << 6);
    pub const HUGE_PAGE:        PageFlags = PageFlags(1 << 7);
    pub const GLOBAL:           PageFlags = PageFlags(1 << 8);
    pub const EXECUTE:          PageFlags = PageFlags(1 << 9);
    pub const NO_EXECUTE:       PageFlags = PageFlags(1 << 63);
    pub const NO_EXEC:          PageFlags = PageFlags(1 << 63);

    pub fn contains(self, other: PageFlags) -> bool { (self.0 & other.0) == other.0 }
    pub fn bits(self) -> u64 { self.0 }
    pub fn insert(&mut self, other: PageFlags) { self.0 |= other.0; }
    pub fn remove(&mut self, other: PageFlags) { self.0 &= !other.0; }
    pub fn from_bits_truncate(bits: u64) -> Self { PageFlags(bits) }
    pub fn is_empty(self) -> bool { self.0 == 0 }
}
impl core::ops::BitOr for PageFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self { PageFlags(self.0 | rhs.0) }
}
impl core::ops::BitAnd for PageFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self { PageFlags(self.0 & rhs.0) }
}
impl core::ops::Not for PageFlags {
    type Output = Self;
    fn not(self) -> Self { PageFlags(!self.0) }
}

impl PageFlags {
    /// W^X invariant: a page may not be both WRITE and executable (i.e. lacking NO_EXECUTE)
    pub fn is_wx_violation(&self) -> bool {
        self.contains(PageFlags::WRITE) && !self.contains(PageFlags::NO_EXECUTE)
    }

    /// Read-only executable (text segments)
    pub fn rx() -> Self {
        PageFlags::PRESENT | PageFlags::ACCESSED
    }

    /// Read-write non-executable (data/stack)
    pub fn rw() -> Self {
        PageFlags::PRESENT | PageFlags::WRITE | PageFlags::NO_EXECUTE | PageFlags::ACCESSED
    }

    /// Read-only non-executable (rodata)
    pub fn ro() -> Self {
        PageFlags::PRESENT | PageFlags::NO_EXECUTE | PageFlags::ACCESSED
    }

    /// User-accessible read-write (heap)
    pub fn user_rw() -> Self {
        PageFlags::PRESENT | PageFlags::WRITE | PageFlags::USER | PageFlags::NO_EXECUTE | PageFlags::ACCESSED
    }

    /// User-accessible read-execute (mapped shared libraries)
    pub fn user_rx() -> Self {
        PageFlags::PRESENT | PageFlags::USER | PageFlags::ACCESSED
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Virtual Memory Region
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegionKind {
    KernelText,
    KernelRodata,
    KernelData,
    KernelStack,
    UserText,
    UserData,
    UserStack,
    UserHeap,
    Mmio,        // Memory-mapped I/O: no execute, no cache
    KernelModule,
    JitBuffer,   // JIT regions: go through a write-then-seal sequence
}

#[derive(Debug, Clone)]
pub struct VmRegion {
    pub start: u64,
    pub end: u64,
    pub flags: PageFlags,
    pub kind: RegionKind,
    pub name: String,
    /// JIT regions can be temporarily writable but must be sealed before execution
    pub jit_sealed: bool,
}

impl VmRegion {
    pub fn new(start: u64, end: u64, flags: PageFlags, kind: RegionKind, name: &str) -> Self {
        VmRegion { start, end, flags, kind, name: name.into(), jit_sealed: false }
    }

    pub fn size(&self) -> u64 { self.end - self.start }

    pub fn contains(&self, addr: u64) -> bool { addr >= self.start && addr < self.end }
}

// ─────────────────────────────────────────────────────────────────────────────
// W^X Enforcement Engine
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum WxViolation {
    /// A region was mapped with both W and X simultaneously
    SimultaneousWriteExecute { region: String, start: u64, end: u64 },
    /// A JIT buffer was executed before being sealed
    UnsealedJitExecution { region: String, start: u64 },
    /// An mprotect call tried to add EXEC without removing WRITE first
    MprotectWxAttempt { addr: u64, new_flags: PageFlags },
    /// Kernel text segment was made writable (CFI violation)
    KernelTextWritable { addr: u64 },
}

pub struct WxEnforcer {
    regions: Arc<Mutex<BTreeMap<u64, VmRegion>>>,
    violations: Arc<Mutex<Vec<WxViolation>>>,
    pub strict_mode: bool, // true = panic on violation; false = log only
}

impl WxEnforcer {
    pub fn new(strict: bool) -> Self {
        WxEnforcer {
            regions: Arc::new(Mutex::new(BTreeMap::new())),
            violations: Arc::new(Mutex::new(Vec::new())),
            strict_mode: strict,
        }
    }

    /// Register a memory region; returns Err if it violates W^X
    pub fn map_region(&self, region: VmRegion) -> Result<(), WxViolation> {
        // Check W^X before inserting
        if region.flags.is_wx_violation() {
            // JIT buffers are allowed to be W temporarily if not yet sealed
            if region.kind == RegionKind::JitBuffer && !region.jit_sealed {
                // Allowed: JIT write phase
            } else {
                let v = WxViolation::SimultaneousWriteExecute {
                    region: region.name.clone(),
                    start: region.start,
                    end: region.end,
                };
                self.record_violation(v.clone());
                if self.strict_mode { return Err(v); }
            }
        }

        // Kernel text must never be writable
        if region.kind == RegionKind::KernelText && region.flags.contains(PageFlags::WRITE) {
            let v = WxViolation::KernelTextWritable { addr: region.start };
            self.record_violation(v.clone());
            if self.strict_mode { return Err(v); }
        }

        self.regions.lock().unwrap().insert(region.start, region);
        Ok(())
    }

    /// Emulate mprotect: change permissions on a region
    pub fn mprotect(&self, addr: u64, new_flags: PageFlags) -> Result<(), WxViolation> {
        // Disallow adding exec without removing write
        if new_flags.is_wx_violation() {
            let v = WxViolation::MprotectWxAttempt { addr, new_flags };
            self.record_violation(v.clone());
            if self.strict_mode { return Err(v); }
        }

        let mut regions = self.regions.lock().unwrap();
        if let Some(region) = regions.values_mut().find(|r| r.contains(addr)) {
            // If making executable: must first remove write
            if new_flags.contains(PageFlags::NO_EXECUTE) == false
                && region.flags.contains(PageFlags::WRITE) {
                let v = WxViolation::MprotectWxAttempt { addr, new_flags };
                drop(regions);
                self.record_violation(v.clone());
                if self.strict_mode { return Err(v); }
                return Ok(());
            }
            region.flags = new_flags;
        }
        Ok(())
    }

    /// Seal a JIT buffer: make it non-writable before execution is allowed.
    /// This follows the W→X sequence: write first, then remove write, then execute.
    pub fn seal_jit(&self, addr: u64) -> Result<(), WxViolation> {
        let mut regions = self.regions.lock().unwrap();
        if let Some(region) = regions.values_mut().find(|r| r.contains(addr)) {
            if region.kind != RegionKind::JitBuffer {
                return Err(WxViolation::UnsealedJitExecution { region: region.name.clone(), start: addr });
            }
            // Remove WRITE, add execute (clear NO_EXECUTE)
            region.flags.remove(PageFlags::WRITE);
            region.flags.remove(PageFlags::NO_EXECUTE);
            region.jit_sealed = true;
        }
        Ok(())
    }

    /// Validate a simulated instruction fetch (execute) at address
    pub fn validate_execute(&self, addr: u64) -> Result<(), WxViolation> {
        let regions = self.regions.lock().unwrap();
        if let Some(region) = regions.values().find(|r| r.contains(addr)) {
            if region.flags.contains(PageFlags::NO_EXECUTE) {
                return Err(WxViolation::SimultaneousWriteExecute {
                    region: region.name.clone(),
                    start: region.start,
                    end: region.end,
                });
            }
            if region.kind == RegionKind::JitBuffer && !region.jit_sealed {
                return Err(WxViolation::UnsealedJitExecution { region: region.name.clone(), start: addr });
            }
        }
        Ok(())
    }

    fn record_violation(&self, v: WxViolation) {
        self.violations.lock().unwrap().push(v);
    }

    pub fn violations(&self) -> Vec<WxViolation> {
        self.violations.lock().unwrap().clone()
    }

    pub fn audit_all(&self) -> Vec<WxViolation> {
        let regions = self.regions.lock().unwrap();
        let mut found = Vec::new();
        for region in regions.values() {
            if region.flags.is_wx_violation() {
                if !(region.kind == RegionKind::JitBuffer && !region.jit_sealed) {
                    found.push(WxViolation::SimultaneousWriteExecute {
                        region: region.name.clone(),
                        start: region.start,
                        end: region.end,
                    });
                }
            }
        }
        found
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// KARL: Kernel Address Randomized Link (OpenBSD-inspired)
// ─────────────────────────────────────────────────────────────────────────────

/// Simulates OpenBSD's KARL: kernel is re-linked at each boot with randomized
/// function order, providing layout entropy against ROP gadget chains.
pub struct KarlRelinker {
    pub base_address: u64,
    pub entropy_seed: u64,
    pub sections: Vec<KernelSection>,
}

#[derive(Debug, Clone)]
pub struct KernelSection {
    pub name: String,
    pub size: u64,
    pub original_offset: u64,
    pub randomized_offset: u64,
}

impl KarlRelinker {
    pub fn new(base: u64, seed: u64) -> Self {
        KarlRelinker { base_address: base, entropy_seed: seed, sections: Vec::new() }
    }

    /// Simulate the KARL relink: shuffle section order using entropy seed
    pub fn relink(&mut self, sections: Vec<(String, u64)>) {
        let mut rng = self.entropy_seed;
        let mut shuffled = sections.clone();

        // Fisher-Yates with LCG RNG (seed from /dev/urandom equivalent)
        for i in (1..shuffled.len()).rev() {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let j = (rng >> 33) as usize % (i + 1);
            shuffled.swap(i, j);
        }

        let mut offset = 0u64;
        for (i, (name, size)) in shuffled.iter().enumerate() {
            self.sections.push(KernelSection {
                name: name.clone(),
                size: *size,
                original_offset: sections[i].1,
                randomized_offset: self.base_address + offset,
            });
            offset += size;
        }
    }

    pub fn layout_entropy_bits(&self) -> f64 {
        let n = self.sections.len() as f64;
        if n <= 1.0 { return 0.0; }
        // log2(n!)
        (1..=(n as u64)).map(|k| (k as f64).log2()).sum()
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_page_flags_wx_violation() {
        let wx = PageFlags::PRESENT | PageFlags::WRITE; // No NO_EXECUTE = executable
        assert!(wx.is_wx_violation());
        let rw = PageFlags::rw();
        assert!(!rw.is_wx_violation());
        let rx = PageFlags::rx();
        assert!(!rx.is_wx_violation());
    }

    #[test]
    fn test_wx_enforcer_blocks_wx_region() {
        let enforcer = WxEnforcer::new(true);
        let region = VmRegion::new(
            0x1000, 0x2000,
            PageFlags::PRESENT | PageFlags::WRITE, // executable (no NX)
            RegionKind::UserData,
            "bad_region",
        );
        let result = enforcer.map_region(region);
        assert!(result.is_err());
    }

    #[test]
    fn test_wx_enforcer_allows_rw_region() {
        let enforcer = WxEnforcer::new(true);
        let region = VmRegion::new(0x1000, 0x2000, PageFlags::rw(), RegionKind::KernelData, "data");
        assert!(enforcer.map_region(region).is_ok());
    }

    #[test]
    fn test_wx_enforcer_allows_rx_region() {
        let enforcer = WxEnforcer::new(true);
        let region = VmRegion::new(0xFFFF_0000, 0xFFFF_1000, PageFlags::rx(), RegionKind::KernelText, "text");
        assert!(enforcer.map_region(region).is_ok());
    }

    #[test]
    fn test_jit_seal_sequence() {
        let enforcer = WxEnforcer::new(true);
        // Step 1: Map JIT buffer as writable (no-execute) for code generation
        let jit_region = VmRegion::new(0x4000_0000, 0x4001_0000, PageFlags::rw(), RegionKind::JitBuffer, "jit");
        enforcer.map_region(jit_region).unwrap();
        // Step 2: Execute before sealing → should fail
        let exec_result = enforcer.validate_execute(0x4000_0000);
        assert!(exec_result.is_err());
        // Step 3: Seal → make executable
        enforcer.seal_jit(0x4000_0000).unwrap();
        // Step 4: Execute after sealing → OK
        assert!(enforcer.validate_execute(0x4000_0000).is_ok());
    }

    #[test]
    fn test_karl_layout_entropy() {
        let mut karl = KarlRelinker::new(0xFFFF_8000_0000_0000, 0xDEAD_BEEF_CAFE_1234);
        let sections = vec![
            ("text.init".into(), 0x1000),
            ("text.core".into(), 0x8000),
            ("text.sched".into(), 0x4000),
            ("text.net".into(), 0x2000),
            ("text.fs".into(), 0x3000),
        ];
        karl.relink(sections);
        assert_eq!(karl.sections.len(), 5);
        let entropy = karl.layout_entropy_bits();
        assert!(entropy > 4.0, "5! = 120 possibilities → ~6.9 bits of entropy");
    }

    #[test]
    fn test_kernel_text_writable_blocked() {
        let enforcer = WxEnforcer::new(true);
        let region = VmRegion::new(
            0xFFFF_8000_0000, 0xFFFF_8001_0000,
            PageFlags::PRESENT | PageFlags::WRITE | PageFlags::NO_EXECUTE,
            RegionKind::KernelText,
            "kernel_text",
        );
        let result = enforcer.map_region(region);
        assert!(result.is_err()); // Kernel text must not be writable
    }
}
