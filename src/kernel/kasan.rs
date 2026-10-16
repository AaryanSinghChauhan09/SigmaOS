//! # Kernel Address Sanitizer (KASAN)
//!
//! KASAN is a dynamic memory-safety error detector for the kernel. It uses a
//! **shadow memory** technique: for every 8 bytes of kernel address space,
//! 1 byte of shadow memory records whether those 8 bytes are fully accessible
//! (0x00), partially accessible (1-7), or poisoned (negative sentinel values).
//!
//! ## Shadow memory layout
//! ```
//! Shadow address = (kernel_addr >> 3) + KASAN_SHADOW_OFFSET
//! ```
//! A shadow byte value of `0x00` means all 8 bytes are valid.  
//! Values `0x01..=0x07` mean the first N bytes are valid (partial objects).  
//! Negative values (0x80–0xFF when treated as i8) are **poison** sentinels:
//!
//! | Sentinel  | Meaning                              |
//! |-----------|--------------------------------------|
//! | 0xFA      | Left redzone (heap allocation)       |
//! | 0xFB      | Right redzone (heap allocation)      |
//! | 0xFC      | Freed / quarantine                   |
//! | 0xF1      | Stack left redzone                   |
//! | 0xF2      | Stack mid redzone                    |
//! | 0xF3      | Stack right redzone                  |
//! | 0xE9      | Use-after-return                     |
//! | 0xE0      | Use-after-scope                      |
//! | 0xFF      | Global redzone                       |
//!
//! ## Detection modes
//! - **Generic KASAN**: fully software-based, ~2× memory overhead.
//! - **SW-Tag KASAN**: uses 8-bit pointer tags in the top byte (ARMv8 TBI).
//! - **HW-Tag KASAN**: leverages ARM Memory Tagging Extension (MTE).
//!
//! ## Redzone layout
//! Each heap allocation is surrounded by redzones:
//! ```
//! [ left_redzone | allocation_data | right_redzone ]
//! ```
//! The redzones are poisoned so any out-of-bounds read/write is caught.
//!
//! ## Quarantine list
//! After `kfree()`, objects are placed in a quarantine list instead of
//! immediately being returned to the allocator. This extends the window during
//! which use-after-free accesses can be detected. The quarantine is drained
//! when it exceeds a configurable byte threshold.

#![allow(dead_code)]

use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

// ── Shadow memory constants ───────────────────────────────────────────────────

/// Shift applied to a kernel address to obtain the shadow byte index.
/// One shadow byte covers 8 kernel bytes (2^3 = 8).
pub const KASAN_SHADOW_SCALE_SHIFT: usize = 3;

/// Canonical shadow offset. On x86-64 the shadow map starts here.
pub const KASAN_SHADOW_OFFSET: usize = 0xdffffc0000000000;

// ── Shadow byte poison sentinels ──────────────────────────────────────────────

/// Shadow sentinel values (stored as `u8` but interpreted as `i8` in checks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ShadowByte {
    /// All 8 bytes in this granule are fully addressable.
    Valid = 0x00,
    /// Heap object left redzone.
    HeapLeftRedzone = 0xFA,
    /// Heap object right redzone.
    HeapRightRedzone = 0xFB,
    /// Freed object in quarantine.
    FreedObject = 0xFC,
    /// Stack left redzone.
    StackLeftRedzone = 0xF1,
    /// Stack middle redzone (between stack variables).
    StackMidRedzone = 0xF2,
    /// Stack right redzone (past the last variable).
    StackRightRedzone = 0xF3,
    /// Use-after-return poisoning.
    UseAfterReturn = 0xE9,
    /// Use-after-scope poisoning.
    UseAfterScope = 0xE0,
    /// Global variable redzone.
    GlobalRedzone = 0xFF,
}

impl ShadowByte {
    /// Returns `true` if this sentinel indicates a poisoned granule.
    pub fn is_poisoned(self) -> bool {
        self != ShadowByte::Valid
    }
}

// ── Redzone configuration ────────────────────────────────────────────────────

/// Minimum redzone size in bytes (must be a multiple of the shadow granule = 8).
pub const KASAN_REDZONE_SIZE: usize = 32;

/// Configuration for KASAN redzone insertion.
#[derive(Debug, Clone)]
pub struct RedzoneConfig {
    /// Number of bytes in the left redzone.
    pub left_size: usize,
    /// Number of bytes in the right redzone.
    pub right_size: usize,
    /// Whether to insert redzones around stack variables.
    pub stack_redzones: bool,
    /// Whether to insert redzones around global variables.
    pub global_redzones: bool,
}

impl Default for RedzoneConfig {
    fn default() -> Self {
        Self {
            left_size: KASAN_REDZONE_SIZE,
            right_size: KASAN_REDZONE_SIZE,
            stack_redzones: true,
            global_redzones: true,
        }
    }
}

// ── Heap allocation metadata ──────────────────────────────────────────────────

/// Metadata stored alongside every KASAN-tracked heap allocation.
#[derive(Debug, Clone)]
pub struct KasanAllocationMeta {
    /// Virtual address of the usable allocation (after left redzone).
    pub user_addr: usize,
    /// Requested allocation size in bytes.
    pub user_size: usize,
    /// Actual allocation size including redzones.
    pub total_size: usize,
    /// Allocation site program counter (for reporting).
    pub alloc_pc: usize,
    /// Deallocation site program counter (set on kfree).
    pub free_pc: usize,
    /// Stack trace depth saved at alloc time.
    pub alloc_stack: [usize; 16],
    /// Stack trace depth saved at free time.
    pub free_stack: [usize; 16],
    /// Whether this allocation is currently in the quarantine list.
    pub in_quarantine: bool,
}

// ── Quarantine list ───────────────────────────────────────────────────────────

/// Maximum total bytes held in quarantine before a drain occurs.
pub const QUARANTINE_MAX_BYTES: usize = 64 * 1024 * 1024; // 64 MiB

/// The quarantine list holds freed objects that are not yet returned to the
/// slab allocator, keeping their shadow bytes poisoned so UAF is detectable.
pub struct KasanQuarantine {
    /// Queue of freed allocation metadata, FIFO.
    queue: VecDeque<KasanAllocationMeta>,
    /// Total bytes currently in quarantine.
    total_bytes: usize,
    /// Maximum bytes before automatic drain.
    max_bytes: usize,
    /// Counter of objects drained so far.
    drain_count: AtomicU64,
}

impl KasanQuarantine {
    /// Create a new quarantine with a configurable capacity.
    pub fn new(max_bytes: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            total_bytes: 0,
            max_bytes,
            drain_count: AtomicU64::new(0),
        }
    }

    /// Enqueue a freed allocation into quarantine.
    ///
    /// If the quarantine exceeds `max_bytes`, older entries are drained first.
    pub fn put(&mut self, meta: KasanAllocationMeta) {
        self.total_bytes += meta.total_size;
        self.queue.push_back(meta);
        while self.total_bytes > self.max_bytes {
            self.drain_one();
        }
    }

    /// Remove the oldest entry from quarantine, unpoison its shadow, and
    /// conceptually return it to the allocator.
    pub fn drain_one(&mut self) -> Option<KasanAllocationMeta> {
        if let Some(meta) = self.queue.pop_front() {
            self.total_bytes = self.total_bytes.saturating_sub(meta.total_size);
            self.drain_count.fetch_add(1, Ordering::Relaxed);
            // TODO: unpoison shadow bytes for `meta.user_addr..meta.user_addr+meta.total_size`
            Some(meta)
        } else {
            None
        }
    }

    /// Return the number of bytes currently held in quarantine.
    pub fn bytes_held(&self) -> usize {
        self.total_bytes
    }
}

// ── KASAN error report ────────────────────────────────────────────────────────

/// The kind of memory access that triggered the KASAN report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessType {
    Read,
    Write,
}

/// A formatted KASAN error report.
#[derive(Debug, Clone)]
pub struct KasanReport {
    /// The faulting kernel virtual address.
    pub fault_addr: usize,
    /// Size of the bad access (1, 2, 4, 8, or N for memcpy).
    pub access_size: usize,
    /// Whether this was a read or write.
    pub access_type: AccessType,
    /// Shadow byte value observed at the fault address.
    pub shadow_val: u8,
    /// Program counter of the faulting instruction.
    pub fault_pc: usize,
    /// Brief human-readable description of the error type.
    pub error_type: &'static str,
    /// Stack backtrace of the faulting task (up to 16 frames).
    pub stack_trace: [usize; 16],
    /// Allocation metadata if the object was KASAN-tracked.
    pub alloc_meta: Option<KasanAllocationMeta>,
}

impl KasanReport {
    /// Format the report as a multi-line kernel log string (like Linux's
    /// `print_address_description`).
    pub fn format(&self) -> String {
        let err = self.error_type;
        let addr = self.fault_addr;
        let size = self.access_size;
        let access = match self.access_type {
            AccessType::Read => "READ",
            AccessType::Write => "WRITE",
        };
        let shadow = self.shadow_val;

        let mut out = String::new();
        out.push_str("==================================================================\n");
        out.push_str(&format!(
            "BUG: KASAN: {err} in kernel at address {addr:#018x}\n"
        ));
        out.push_str(&format!("{access} of size {size} at addr {addr:#018x}\n"));
        out.push_str(&format!("Shadow byte: {shadow:#04x}\n"));
        out.push_str(&format!("Fault PC:    {:#018x}\n", self.fault_pc));
        out.push_str("\nStack trace:\n");
        for (i, &frame) in self.stack_trace.iter().enumerate() {
            if frame == 0 {
                break;
            }
            out.push_str(&format!("  #{i:02}: {frame:#018x}\n"));
        }
        if let Some(ref m) = self.alloc_meta {
            out.push_str(&format!(
                "\nAllocated by task at {:#018x}, size={}\n",
                m.alloc_pc, m.user_size
            ));
            if m.in_quarantine {
                out.push_str(&format!("Freed by task at {:#018x}\n", m.free_pc));
            }
        }
        out.push_str("==================================================================\n");
        out
    }
}

// ── Shadow memory map ─────────────────────────────────────────────────────────

/// Simulated shadow memory map (in a real kernel this would be a statically
/// mapped region of physical memory).
pub struct KasanShadowMap {
    /// Backing storage for the shadow bytes (indexed by shadow offset).
    /// In hardware this is a 1/8-size mirror of the kernel VA space.
    map: Vec<u8>,
    /// Base of the kernel virtual address range being shadowed.
    kernel_base: usize,
    /// Total kernel VA bytes being covered.
    kernel_size: usize,
}

impl KasanShadowMap {
    /// Create a shadow map for a kernel VA range.
    pub fn new(kernel_base: usize, kernel_size: usize) -> Self {
        let shadow_bytes = kernel_size >> KASAN_SHADOW_SCALE_SHIFT;
        Self {
            map: vec![0u8; shadow_bytes],
            kernel_base,
            kernel_size,
        }
    }

    /// Convert a kernel virtual address to its shadow byte index.
    #[inline]
    pub fn shadow_index(&self, addr: usize) -> Option<usize> {
        let offset = addr.wrapping_sub(self.kernel_base);
        if offset >= self.kernel_size {
            return None;
        }
        Some(offset >> KASAN_SHADOW_SCALE_SHIFT)
    }

    /// Read the shadow byte for a given kernel address.
    pub fn read_shadow(&self, addr: usize) -> Option<u8> {
        self.shadow_index(addr).map(|idx| self.map[idx])
    }

    /// Poison a range of kernel addresses with the given sentinel.
    pub fn poison(&mut self, addr: usize, size: usize, sentinel: ShadowByte) {
        let start_idx = match self.shadow_index(addr) {
            Some(i) => i,
            None => return,
        };
        let granules = (size + 7) >> KASAN_SHADOW_SCALE_SHIFT;
        for i in start_idx..(start_idx + granules).min(self.map.len()) {
            self.map[i] = sentinel as u8;
        }
    }

    /// Unpoison (mark accessible) a range of kernel addresses.
    pub fn unpoison(&mut self, addr: usize, size: usize) {
        let start_idx = match self.shadow_index(addr) {
            Some(i) => i,
            None => return,
        };
        let full_granules = size >> KASAN_SHADOW_SCALE_SHIFT;
        let partial = (size & 7) as u8;
        for i in start_idx..(start_idx + full_granules).min(self.map.len()) {
            self.map[i] = 0x00;
        }
        if partial > 0 && start_idx + full_granules < self.map.len() {
            self.map[start_idx + full_granules] = partial;
        }
    }
}

// ── KASAN statistics ──────────────────────────────────────────────────────────

/// Runtime statistics collected by KASAN.
#[derive(Debug, Default)]
pub struct KasanStats {
    pub heap_oob_reads: AtomicU64,
    pub heap_oob_writes: AtomicU64,
    pub use_after_free_reads: AtomicU64,
    pub use_after_free_writes: AtomicU64,
    pub stack_oob: AtomicU64,
    pub global_oob: AtomicU64,
    pub total_reports: AtomicU64,
}

// ── Main KASAN engine ─────────────────────────────────────────────────────────

/// Top-level KASAN subsystem engine.
pub struct KasanEngine {
    /// Shadow memory map.
    pub shadow: KasanShadowMap,
    /// Quarantine list for freed objects.
    pub quarantine: KasanQuarantine,
    /// Redzone configuration.
    pub redzone_cfg: RedzoneConfig,
    /// Runtime statistics.
    pub stats: KasanStats,
    /// Recent reports ring buffer (last N reports).
    pub recent_reports: Mutex<VecDeque<KasanReport>>,
    /// Maximum reports to keep in memory.
    pub max_reports: usize,
    /// Whether KASAN checking is currently enabled.
    pub enabled: bool,
    /// Total allocations instrumented.
    pub alloc_count: AtomicUsize,
}

impl KasanEngine {
    /// Initialise KASAN over the given kernel VA range.
    pub fn new(kernel_base: usize, kernel_size: usize) -> Self {
        Self {
            shadow: KasanShadowMap::new(kernel_base, kernel_size),
            quarantine: KasanQuarantine::new(QUARANTINE_MAX_BYTES),
            redzone_cfg: RedzoneConfig::default(),
            stats: KasanStats::default(),
            recent_reports: Mutex::new(VecDeque::new()),
            max_reports: 64,
            enabled: true,
            alloc_count: AtomicUsize::new(0),
        }
    }

    /// Called by the allocator **after** a successful allocation.
    ///
    /// Poisons the left and right redzones and unpoisons the usable region.
    pub fn on_alloc(&mut self, user_addr: usize, user_size: usize, alloc_pc: usize) {
        if !self.enabled {
            return;
        }
        let left = self.redzone_cfg.left_size;
        let right = self.redzone_cfg.right_size;

        // Poison left redzone (at user_addr - left)
        let left_addr = user_addr.wrapping_sub(left);
        self.shadow
            .poison(left_addr, left, ShadowByte::HeapLeftRedzone);

        // Unpoison usable region
        self.shadow.unpoison(user_addr, user_size);

        // Poison right redzone (at user_addr + user_size)
        let right_addr = user_addr + user_size;
        self.shadow
            .poison(right_addr, right, ShadowByte::HeapRightRedzone);

        self.alloc_count.fetch_add(1, Ordering::Relaxed);
        let _ = alloc_pc; // TODO: record in per-object metadata slab
    }

    /// Called by `kfree` **before** returning memory to the allocator.
    ///
    /// Poisons the entire allocation (including original data region) and
    /// enqueues it in the quarantine list.
    pub fn on_free(&mut self, user_addr: usize, user_size: usize, free_pc: usize) {
        if !self.enabled {
            return;
        }
        self.shadow
            .poison(user_addr, user_size, ShadowByte::FreedObject);

        let meta = KasanAllocationMeta {
            user_addr,
            user_size,
            total_size: user_size + self.redzone_cfg.left_size + self.redzone_cfg.right_size,
            alloc_pc: 0, // TODO: retrieve from per-object metadata
            free_pc,
            alloc_stack: [0; 16],
            free_stack: [0; 16],
            in_quarantine: true,
        };
        self.quarantine.put(meta);
    }

    /// Main KASAN **check** hook — called by compiler-instrumented load/store
    /// instructions before every memory access.
    ///
    /// Returns `Ok(())` if the access is valid, or `Err(KasanReport)` if a
    /// violation is detected.
    pub fn check_access(
        &mut self,
        addr: usize,
        size: usize,
        access_type: AccessType,
        fault_pc: usize,
    ) -> Result<(), KasanReport> {
        if !self.enabled {
            return Ok(());
        }

        let shadow_val = match self.shadow.read_shadow(addr) {
            Some(v) => v,
            None => return Ok(()), // address outside shadowed region
        };

        // A shadow byte of 0x00 means fully valid.
        // Positive non-zero means "only first N bytes valid" — check boundary.
        // Negative (>= 0x80 as u8) means poisoned.
        let is_poisoned = if shadow_val == 0 {
            false
        } else if shadow_val < 8 {
            // Partial granule: poisoned if access touches bytes >= shadow_val
            let offset_in_granule = addr & 7;
            offset_in_granule + size > shadow_val as usize
        } else {
            true // Fully poisoned granule
        };

        if !is_poisoned {
            return Ok(());
        }

        let error_type = Self::classify_error(shadow_val);

        // Update statistics
        match (access_type, shadow_val) {
            (AccessType::Read, 0xFC) => {
                self.stats
                    .use_after_free_reads
                    .fetch_add(1, Ordering::Relaxed);
            }
            (AccessType::Write, 0xFC) => {
                self.stats
                    .use_after_free_writes
                    .fetch_add(1, Ordering::Relaxed);
            }
            (AccessType::Read, 0xFA | 0xFB) => {
                self.stats.heap_oob_reads.fetch_add(1, Ordering::Relaxed);
            }
            (AccessType::Write, 0xFA | 0xFB) => {
                self.stats.heap_oob_writes.fetch_add(1, Ordering::Relaxed);
            }
            (_, 0xF1..=0xF3) => {
                self.stats.stack_oob.fetch_add(1, Ordering::Relaxed);
            }
            (_, 0xFF) => {
                self.stats.global_oob.fetch_add(1, Ordering::Relaxed);
            }
            _ => {}
        }
        self.stats.total_reports.fetch_add(1, Ordering::Relaxed);

        let report = KasanReport {
            fault_addr: addr,
            access_size: size,
            access_type,
            shadow_val,
            fault_pc,
            error_type,
            stack_trace: [0; 16], // TODO: unwind stack
            alloc_meta: None,     // TODO: look up object metadata
        };

        if let Ok(mut ring) = self.recent_reports.lock() {
            if ring.len() >= self.max_reports {
                ring.pop_front();
            }
            ring.push_back(report.clone());
        }

        Err(report)
    }

    /// Classify an error based on the shadow byte sentinel value.
    fn classify_error(shadow_val: u8) -> &'static str {
        match shadow_val {
            0xFA | 0xFB => "heap-buffer-overflow",
            0xFC => "use-after-free",
            0xF1 | 0xF2 | 0xF3 => "stack-out-of-bounds",
            0xE9 => "use-after-return",
            0xE0 => "use-after-scope",
            0xFF => "global-out-of-bounds",
            _ => "unknown-kasan-error",
        }
    }

    /// Disable KASAN checking (e.g. during boot or critical sections).
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Re-enable KASAN checking.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Print all recent KASAN reports to the kernel log (stub).
    pub fn dump_reports(&self) {
        if let Ok(ring) = self.recent_reports.lock() {
            for report in ring.iter() {
                // TODO: emit via printk / kernel logger
                let _ = report.format();
            }
        }
    }
}
