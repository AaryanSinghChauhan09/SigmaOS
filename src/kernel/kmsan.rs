//! # Kernel Memory Sanitizer (KMSAN)
//!
//! KMSAN detects uses of **uninitialized memory** in kernel code. Unlike KASAN
//! (which uses shadow bytes to flag invalid addresses), KMSAN tracks the
//! **origin** of every bit of kernel memory to determine whether it has been
//! explicitly written before it is read.
//!
//! ## Core design
//! KMSAN maintains two parallel shadow regions for every byte of kernel memory:
//!
//! 1. **Shadow map** — a bit-for-bit copy of kernel memory where a `1` bit
//!    means the corresponding bit is **uninitialized** and a `0` bit means it
//!    is initialized. This is sometimes called the "taint" map.
//!
//! 2. **Origin map** — a 4-byte origin ID per 4-byte aligned kernel word.
//!    The origin ID encodes *where* the uninitialized data came from
//!    (allocation site, stack frame, or copy source).
//!
//! ## Shadow propagation
//! KMSAN instruments every load/store and every kernel helper that moves
//! data (memcpy, memset, copy_from_user, …). Propagation rules:
//!
//! - `memset(dst, 0, n)` → shadow[dst..dst+n] = 0 (initialized)
//! - `memcpy(dst, src, n)` → shadow[dst] = shadow[src], origin[dst] = origin[src]
//! - Arithmetic on initialized values → result is initialized
//! - Arithmetic involving uninitialized values → result is uninitialized
//!
//! ## Stack frame instrumentation
//! At function entry, KMSAN poisons the entire stack frame (sets shadow = 0xFF).
//! When a local variable is initialized, its shadow bytes are cleared.
//! When `copy_to_user` or a network transmit path reads a kernel buffer, KMSAN
//! checks that the shadow is zero over the entire range.
//!
//! ## Origin tracking
//! Each origin ID maps to an `OriginEntry` that stores:
//! - The allocation/stack site call chain
//! - A creation timestamp
//! - A textual description ("heap alloc", "stack var", "copy_from_user", …)
//!
//! ## References
//! - `Documentation/dev-tools/kmsan.rst` in the Linux kernel tree
//! - Stepanov & Serebryany, "MemorySanitizer: fast detector of uninitialized
//!   memory use in C++", CGO 2015

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Mutex;

// ── Origin ID type ────────────────────────────────────────────────────────────

/// A unique 32-bit origin ID. `0` means "initialized / no origin".
pub type OriginId = u32;

/// Global counter for allocating origin IDs.
static NEXT_ORIGIN_ID: AtomicU32 = AtomicU32::new(1);

/// Allocate a fresh, unique origin ID.
pub fn alloc_origin_id() -> OriginId {
    NEXT_ORIGIN_ID.fetch_add(1, Ordering::Relaxed)
}

// ── Origin kinds ──────────────────────────────────────────────────────────────

/// The category of memory that produced this origin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OriginKind {
    /// Uninitialized bytes from a heap allocation (`kmalloc`, `vmalloc`, …).
    HeapAlloc,
    /// Uninitialized bytes from a stack variable.
    StackVar,
    /// Data copied from user space (`copy_from_user`, `get_user`, …).
    CopyFromUser,
    /// Data from hardware (MMIO read, DMA, port I/O).
    HardwareRead,
    /// Inter-process copy (pipe, socket receive buffer).
    IpcCopy,
    /// Origin propagated through a chain of operations.
    Derived,
}

// ── Origin entry ──────────────────────────────────────────────────────────────

/// Full metadata for one KMSAN origin.
#[derive(Debug, Clone)]
pub struct OriginEntry {
    /// Unique ID for this origin.
    pub id: OriginId,
    /// How this uninitialized memory was produced.
    pub kind: OriginKind,
    /// Program counter of the allocation / variable introduction site.
    pub creation_pc: usize,
    /// Brief label for kernel log output.
    pub label: &'static str,
    /// Stack backtrace at origin creation (up to 16 frames).
    pub stack_trace: [usize; 16],
    /// Monotonic timestamp (nanoseconds) when this origin was created.
    pub timestamp_ns: u64,
}

// ── KMSAN shadow map ──────────────────────────────────────────────────────────

/// The shadow map holds one bit per kernel byte (packed into `u8` bytes).
///
/// A shadow bit of **1** means the corresponding kernel byte is *uninitialized*.
/// A shadow bit of **0** means it is *initialized*.
///
/// For simplicity we use one full `u8` per kernel byte (byte-granular shadow).
pub struct KmsanShadowMap {
    /// shadow[i] == 0xFF → all 8 bits of kernel byte i are uninitialized.
    /// shadow[i] == 0x00 → all 8 bits are initialized.
    map: Vec<u8>,
    /// Base kernel virtual address covered by this map.
    base_addr: usize,
    /// Total number of kernel bytes covered.
    covered_bytes: usize,
}

impl KmsanShadowMap {
    /// Create a shadow map for a kernel VA range, initially fully poisoned
    /// (all memory considered uninitialized).
    pub fn new(base_addr: usize, covered_bytes: usize) -> Self {
        Self {
            map: vec![0xFF; covered_bytes], // Start poisoned
            base_addr,
            covered_bytes,
        }
    }

    #[inline]
    fn offset(&self, addr: usize) -> Option<usize> {
        let off = addr.wrapping_sub(self.base_addr);
        if off < self.covered_bytes { Some(off) } else { None }
    }

    /// Mark a range of kernel memory as **initialized** (clear shadow bits).
    pub fn mark_initialized(&mut self, addr: usize, size: usize) {
        if let Some(start) = self.offset(addr) {
            let end = (start + size).min(self.map.len());
            for b in &mut self.map[start..end] {
                *b = 0x00;
            }
        }
    }

    /// Mark a range of kernel memory as **uninitialized** (set shadow bits).
    pub fn mark_uninitialized(&mut self, addr: usize, size: usize) {
        if let Some(start) = self.offset(addr) {
            let end = (start + size).min(self.map.len());
            for b in &mut self.map[start..end] {
                *b = 0xFF;
            }
        }
    }

    /// Propagate shadow from `src` to `dst` (as in `memcpy`).
    pub fn propagate(&mut self, dst: usize, src: usize, size: usize) {
        // We need to copy within the same backing vec; use a temp buffer.
        let src_off = match self.offset(src) {
            Some(o) => o,
            None => return,
        };
        let dst_off = match self.offset(dst) {
            Some(o) => o,
            None => return,
        };
        let src_end = (src_off + size).min(self.map.len());
        let src_end = src_end.min(src_off + size);
        let copy_len = src_end - src_off;
        let src_shadow: Vec<u8> = self.map[src_off..src_off + copy_len].to_vec();
        let dst_end = (dst_off + copy_len).min(self.map.len());
        let actual = dst_end - dst_off;
        self.map[dst_off..dst_off + actual].copy_from_slice(&src_shadow[..actual]);
    }

    /// Check if any byte in `[addr, addr+size)` is uninitialized.
    /// Returns the first uninitialized offset, if any.
    pub fn check_initialized(&self, addr: usize, size: usize) -> Option<usize> {
        let start = self.offset(addr)?;
        let end = (start + size).min(self.map.len());
        for i in start..end {
            if self.map[i] != 0 {
                return Some(addr + (i - start));
            }
        }
        None
    }
}

// ── KMSAN origin map ──────────────────────────────────────────────────────────

/// The origin map stores one `OriginId` per 4-byte aligned kernel word.
pub struct KmsanOriginMap {
    /// origin_ids[word_index] = origin of bytes at base + word_index*4.
    origin_ids: Vec<OriginId>,
    base_addr: usize,
    word_count: usize,
}

impl KmsanOriginMap {
    /// Create an origin map for `byte_size` kernel bytes starting at `base_addr`.
    pub fn new(base_addr: usize, byte_size: usize) -> Self {
        let word_count = (byte_size + 3) / 4;
        Self {
            origin_ids: vec![0; word_count],
            base_addr,
            word_count,
        }
    }

    #[inline]
    fn word_index(&self, addr: usize) -> Option<usize> {
        let off = addr.wrapping_sub(self.base_addr);
        let idx = off / 4;
        if idx < self.word_count { Some(idx) } else { None }
    }

    /// Set the origin of a word-aligned kernel address.
    pub fn set_origin(&mut self, addr: usize, origin: OriginId) {
        if let Some(idx) = self.word_index(addr) {
            self.origin_ids[idx] = origin;
        }
    }

    /// Get the origin ID for a kernel address.
    pub fn get_origin(&self, addr: usize) -> OriginId {
        self.word_index(addr)
            .map(|i| self.origin_ids[i])
            .unwrap_or(0)
    }

    /// Propagate origins from `src` range to `dst` range (word-by-word).
    pub fn propagate(&mut self, dst: usize, src: usize, byte_size: usize) {
        let words = (byte_size + 3) / 4;
        let copied: Vec<OriginId> = (0..words)
            .map(|i| self.get_origin(src + i * 4))
            .collect();
        for (i, &oid) in copied.iter().enumerate() {
            self.set_origin(dst + i * 4, oid);
        }
    }
}

// ── Stack frame descriptor ────────────────────────────────────────────────────

/// Describes one stack frame that KMSAN has instrumented.
///
/// At function entry the frame is fully poisoned; individual variable
/// initializations clear the corresponding shadow bytes.
#[derive(Debug, Clone)]
pub struct KmsanStackFrame {
    /// Stack pointer at frame entry.
    pub frame_sp: usize,
    /// Total size of this stack frame (from compiler annotation).
    pub frame_size: usize,
    /// Origin ID assigned to uninitialized locals in this frame.
    pub frame_origin: OriginId,
    /// Textual function name (for reports).
    pub function_name: &'static str,
}

// ── KMSAN report ──────────────────────────────────────────────────────────────

/// A KMSAN use-of-uninitialized-memory report.
#[derive(Debug, Clone)]
pub struct KmsanReport {
    /// The kernel address of the first uninitialized byte read.
    pub fault_addr: usize,
    /// Size of the read that triggered the report.
    pub access_size: usize,
    /// Program counter of the violating instruction.
    pub fault_pc: usize,
    /// Origin ID indicating where this data came from.
    pub origin_id: OriginId,
    /// Human-readable description of the origin.
    pub origin_desc: String,
    /// Stack trace of the violating access.
    pub stack_trace: [usize; 16],
}

impl KmsanReport {
    /// Format the report for kernel log output.
    pub fn format(&self) -> String {
        let mut out = String::new();
        out.push_str("==================================================================\n");
        out.push_str("BUG: KMSAN: use of uninitialized memory\n");
        out.push_str(&format!(
            "READ of size {} at addr {:#018x}\n",
            self.access_size, self.fault_addr
        ));
        out.push_str(&format!("Fault PC: {:#018x}\n", self.fault_pc));
        out.push_str(&format!(
            "Origin: [id={}] {}\n",
            self.origin_id, self.origin_desc
        ));
        out.push_str("\nStack trace:\n");
        for (i, &frame) in self.stack_trace.iter().enumerate() {
            if frame == 0 {
                break;
            }
            out.push_str(&format!("  #{i:02}: {frame:#018x}\n"));
        }
        out.push_str("==================================================================\n");
        out
    }
}

// ── KMSAN statistics ──────────────────────────────────────────────────────────

/// Runtime statistics for the KMSAN subsystem.
#[derive(Debug, Default)]
pub struct KmsanStats {
    /// Total use-of-uninitialized-memory reports issued.
    pub total_reports: AtomicU64,
    /// Number of heap allocations tracked.
    pub heap_allocs: AtomicU64,
    /// Number of stack frames instrumented.
    pub stack_frames: AtomicU64,
    /// Number of `copy_from_user` calls checked.
    pub copy_from_user_checks: AtomicU64,
}

// ── Main KMSAN engine ─────────────────────────────────────────────────────────

/// Top-level KMSAN subsystem engine.
pub struct KmsanEngine {
    /// Shadow map (uninitialized = 0xFF bits).
    pub shadow: KmsanShadowMap,
    /// Origin map (4-byte granularity).
    pub origin_map: KmsanOriginMap,
    /// Origin ID → metadata table.
    pub origin_table: Mutex<HashMap<OriginId, OriginEntry>>,
    /// Runtime statistics.
    pub stats: KmsanStats,
    /// Recent reports ring buffer.
    pub recent_reports: Mutex<Vec<KmsanReport>>,
    /// Maximum reports to retain.
    pub max_reports: usize,
    /// Whether checking is currently enabled.
    pub enabled: bool,
}

impl KmsanEngine {
    /// Initialise KMSAN over the given kernel VA range.
    pub fn new(base_addr: usize, covered_bytes: usize) -> Self {
        Self {
            shadow: KmsanShadowMap::new(base_addr, covered_bytes),
            origin_map: KmsanOriginMap::new(base_addr, covered_bytes),
            origin_table: Mutex::new(HashMap::new()),
            stats: KmsanStats::default(),
            recent_reports: Mutex::new(Vec::new()),
            max_reports: 64,
            enabled: true,
        }
    }

    /// Register a new origin and return its ID.
    pub fn register_origin(
        &self,
        kind: OriginKind,
        creation_pc: usize,
        label: &'static str,
    ) -> OriginId {
        let id = alloc_origin_id();
        let entry = OriginEntry {
            id,
            kind,
            creation_pc,
            label,
            stack_trace: [0; 16], // TODO: capture real stack
            timestamp_ns: 0,       // TODO: read TSC
        };
        if let Ok(mut table) = self.origin_table.lock() {
            table.insert(id, entry);
        }
        id
    }

    /// Called when the slab allocator creates a new object.
    ///
    /// Marks the object memory as uninitialized and assigns a heap origin.
    pub fn on_heap_alloc(&mut self, addr: usize, size: usize, alloc_pc: usize) {
        if !self.enabled {
            return;
        }
        let origin = self.register_origin(OriginKind::HeapAlloc, alloc_pc, "heap-alloc");
        self.shadow.mark_uninitialized(addr, size);
        // Set origin for every 4-byte word in the allocation
        let words = (size + 3) / 4;
        for i in 0..words {
            self.origin_map.set_origin(addr + i * 4, origin);
        }
        self.stats.heap_allocs.fetch_add(1, Ordering::Relaxed);
    }

    /// Called when a slab object is freed (re-poison the region).
    pub fn on_heap_free(&mut self, addr: usize, size: usize) {
        if !self.enabled {
            return;
        }
        self.shadow.mark_uninitialized(addr, size);
    }

    /// Called at function entry to poison the stack frame.
    ///
    /// `sp` is the stack pointer after the frame is set up; `frame_size` is the
    /// total size of local variables in this frame (from compiler metadata).
    pub fn on_stack_frame_enter(
        &mut self,
        sp: usize,
        frame_size: usize,
        function_name: &'static str,
    ) -> KmsanStackFrame {
        if !self.enabled {
            return KmsanStackFrame {
                frame_sp: sp,
                frame_size,
                frame_origin: 0,
                function_name,
            };
        }
        let origin = self.register_origin(OriginKind::StackVar, 0, "stack-var");
        self.shadow.mark_uninitialized(sp, frame_size);
        let words = (frame_size + 3) / 4;
        for i in 0..words {
            self.origin_map.set_origin(sp + i * 4, origin);
        }
        self.stats.stack_frames.fetch_add(1, Ordering::Relaxed);
        KmsanStackFrame {
            frame_sp: sp,
            frame_size,
            frame_origin: origin,
            function_name,
        }
    }

    /// Called at function exit to re-poison the stack frame (prevents
    /// use-after-return of stack data by callees or signal handlers).
    pub fn on_stack_frame_exit(&mut self, frame: &KmsanStackFrame) {
        if !self.enabled {
            return;
        }
        self.shadow.mark_uninitialized(frame.frame_sp, frame.frame_size);
    }

    /// Instrumentation hook for `memset(dst, val, size)`.
    ///
    /// Marks `size` bytes at `dst` as initialized (since any explicit write,
    /// even to 0, initializes the memory).
    pub fn on_memset(&mut self, dst: usize, size: usize) {
        if !self.enabled {
            return;
        }
        self.shadow.mark_initialized(dst, size);
    }

    /// Instrumentation hook for `memcpy(dst, src, size)`.
    ///
    /// Propagates both shadow and origin from `src` to `dst`.
    pub fn on_memcpy(&mut self, dst: usize, src: usize, size: usize) {
        if !self.enabled {
            return;
        }
        self.shadow.propagate(dst, src, size);
        self.origin_map.propagate(dst, src, size);
    }

    /// Instrumentation hook for `copy_from_user(dst, user_src, size)`.
    ///
    /// User-space data is treated as initialized by KMSAN (it passed through
    /// `access_ok` and the copy machinery). Mark `dst` as initialized.
    pub fn on_copy_from_user(&mut self, dst: usize, size: usize) {
        if !self.enabled {
            return;
        }
        self.shadow.mark_initialized(dst, size);
        self.stats.copy_from_user_checks.fetch_add(1, Ordering::Relaxed);
    }

    /// Check that `[addr, addr+size)` is entirely initialized before a use.
    ///
    /// Returns `Ok(())` if clean, or `Err(KmsanReport)` if uninitialized bytes
    /// are found.
    pub fn check_use(
        &mut self,
        addr: usize,
        size: usize,
        fault_pc: usize,
    ) -> Result<(), KmsanReport> {
        if !self.enabled {
            return Ok(());
        }
        if let Some(bad_addr) = self.shadow.check_initialized(addr, size) {
            let origin_id = self.origin_map.get_origin(bad_addr);
            let origin_desc = {
                let table = self.origin_table.lock().unwrap();
                table
                    .get(&origin_id)
                    .map(|e| format!("{} at pc={:#x}", e.label, e.creation_pc))
                    .unwrap_or_else(|| "<unknown>".into())
            };

            let report = KmsanReport {
                fault_addr: bad_addr,
                access_size: size,
                fault_pc,
                origin_id,
                origin_desc,
                stack_trace: [0; 16], // TODO: capture real stack
            };

            self.stats.total_reports.fetch_add(1, Ordering::Relaxed);

            if let Ok(mut ring) = self.recent_reports.lock() {
                if ring.len() >= self.max_reports {
                    ring.remove(0);
                }
                ring.push(report.clone());
            }

            return Err(report);
        }
        Ok(())
    }

    /// Dump all recent reports (for panic / debug output).
    pub fn dump_reports(&self) {
        if let Ok(ring) = self.recent_reports.lock() {
            for r in ring.iter() {
                let _ = r.format(); // TODO: route through printk
            }
        }
    }
}
