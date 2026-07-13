/// SigmaOS: Self-Healing Recovery Engine — Watchdog + Safe Mode + Panic Handler.
/// Provides: hardware watchdog integration, kernel panic handler, safe mode
/// boot, and automatic shard restart on failure.
///
/// Architecture: no_std, no alloc. Lock-free via atomic primitives.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, AtomicU64, AtomicBool, Ordering};

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8    = u8;
type SigmaU16   = u16;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Constants ──────────────────────────────────────────────────────────────

/// Watchdog timeout in milliseconds before hardware reset
const WATCHDOG_TIMEOUT_MS: SigmaU32 = 30_000;  // 30 seconds
/// Maximum consecutive shard crashes before quarantine
const MAX_SHARD_CRASHES: SigmaU32 = 5;
/// Panic ring buffer entries
const PANIC_LOG_SIZE: usize = 16;
/// Safe mode memory limit (32 MB — minimal environment)
const SAFE_MODE_MEMORY_MB: SigmaU32 = 32;

// ─── Watchdog State ─────────────────────────────────────────────────────────

/// Hardware watchdog state managed via atomics (lock-free)
pub struct SigmaWatchdog {
    /// Monotonic heartbeat counter — hardware timer checks this
    heartbeat:        AtomicU64,
    /// Last heartbeat timestamp (milliseconds since boot)
    last_heartbeat_ms: AtomicU64,
    /// Whether the watchdog is armed (true = hardware reset on timeout)
    armed:            AtomicBool,
    /// Watchdog timeout in milliseconds
    timeout_ms:       AtomicU32,
    /// Total watchdog fires since boot
    fire_count:       AtomicU32,
}

impl SigmaWatchdog {
    pub const fn new() -> Self {
        Self {
            heartbeat:         AtomicU64::new(0),
            last_heartbeat_ms: AtomicU64::new(0),
            armed:             AtomicBool::new(false),
            timeout_ms:        AtomicU32::new(WATCHDOG_TIMEOUT_MS),
            fire_count:        AtomicU32::new(0),
        }
    }

    /// Arm the hardware watchdog. Once armed, must call heartbeat() periodically
    /// or the system will reset.
    pub fn arm(&self) {
        self.armed.store(true, Ordering::SeqCst);
        self.heartbeat.store(0, Ordering::SeqCst);
        self.last_heartbeat_ms.store(
            unsafe { kernel_monotonic_ms() },
            Ordering::SeqCst,
        );
        unsafe { kernel_watchdog_arm(self.timeout_ms.load(Ordering::Relaxed)) };
    }

    /// Disarm the watchdog (e.g. during controlled shutdown/maintenance)
    pub fn disarm(&self) {
        self.armed.store(false, Ordering::SeqCst);
        unsafe { kernel_watchdog_disarm() };
    }

    /// Pet the watchdog — must be called before timeout expires.
    /// Called by the main scheduler loop and critical kernel threads.
    #[inline(always)]
    pub fn heartbeat(&self) {
        let hb = self.heartbeat.fetch_add(1, Ordering::Relaxed);
        self.last_heartbeat_ms.store(
            unsafe { kernel_monotonic_ms() },
            Ordering::Relaxed,
        );

        if self.armed.load(Ordering::Relaxed) {
            // Tell hardware watchdog we're still alive
            unsafe { kernel_watchdog_pet(hb) };
        }
    }

    /// Called by hardware watchdog interrupt when timeout fires
    /// (before hardware reset, if NMI watchdog is used)
    pub fn on_watchdog_fire(&self) {
        self.fire_count.fetch_add(1, Ordering::SeqCst);

        // Try to log to kernel panic buffer before reset
        unsafe {
            let hb_count = self.heartbeat.load(Ordering::Relaxed);
            let last_ms  = self.last_heartbeat_ms.load(Ordering::Relaxed);
            let now_ms   = kernel_monotonic_ms();
            kernel_panic_log(
                PanicKind::WatchdogTimeout,
                hb_count,
                now_ms.saturating_sub(last_ms),
            );
        }
    }

    pub fn heartbeat_count(&self) -> SigmaU64 {
        self.heartbeat.load(Ordering::Relaxed)
    }

    pub fn is_armed(&self) -> bool {
        self.armed.load(Ordering::Relaxed)
    }
}

// ─── Panic Handler ───────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum PanicKind {
    Null           = 0x00,
    StackOverflow  = 0x01,
    PageFault      = 0x02,
    DoubleFault    = 0x03,
    GeneralProtect = 0x04,
    WatchdogTimeout = 0x05,
    ShardCrash     = 0x06,
    MemoryCorrupt  = 0x07,
    KernelAssert   = 0x08,
    Unreachable    = 0x09,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PanicRecord {
    pub kind:        PanicKind,
    pub timestamp_ms: SigmaU64,
    pub cpu_id:      SigmaU32,
    pub rip:         SigmaU64,   // Faulting instruction pointer
    pub rsp:         SigmaU64,   // Stack pointer at fault
    pub error_code:  SigmaU64,   // CPU error code (page fault: CR2, etc.)
    pub message:     [SigmaU8; 64],
}

impl PanicRecord {
    pub const fn empty() -> Self {
        Self {
            kind:         PanicKind::Null,
            timestamp_ms: 0,
            cpu_id:       0,
            rip:          0,
            rsp:          0,
            error_code:   0,
            message:      [0u8; 64],
        }
    }
}

/// Ring buffer of recent kernel panics for forensic analysis
pub struct PanicLog {
    records: [PanicRecord; PANIC_LOG_SIZE],
    head:    SigmaUsize,
    count:   SigmaU32,
}

impl PanicLog {
    pub const fn new() -> Self {
        Self {
            records: [PanicRecord::empty(); PANIC_LOG_SIZE],
            head:    0,
            count:   0,
        }
    }

    pub fn push(&mut self, record: PanicRecord) {
        self.records[self.head] = record;
        self.head = (self.head + 1) % PANIC_LOG_SIZE;
        self.count = self.count.saturating_add(1);
    }

    pub fn total_panics(&self) -> SigmaU32 {
        self.count
    }

    pub fn last(&self) -> Option<&PanicRecord> {
        if self.count == 0 { return None; }
        let idx = (self.head + PANIC_LOG_SIZE - 1) % PANIC_LOG_SIZE;
        Some(&self.records[idx])
    }
}

// ─── Shard Health Tracker ────────────────────────────────────────────────────

const MAX_TRACKED_SHARDS: usize = 64;

#[derive(Clone, Copy)]
pub struct ShardHealth {
    pub shard_id:       SigmaU32,
    pub crash_count:    SigmaU32,
    pub last_crash_ms:  SigmaU64,
    pub quarantined:    SigmaBool,
    pub restart_count:  SigmaU32,
}

impl ShardHealth {
    pub const fn empty() -> Self {
        Self {
            shard_id:      0,
            crash_count:   0,
            last_crash_ms: 0,
            quarantined:   false,
            restart_count: 0,
        }
    }
}

// ─── Self-Healing Engine ─────────────────────────────────────────────────────

/// Main self-healing / safe-mode controller
pub struct SelfHealingEngine {
    pub initialized:    SigmaBool,
    pub safe_mode:      SigmaBool,
    watchdog:           SigmaWatchdog,
    panic_log:          PanicLog,
    shard_health:       [ShardHealth; MAX_TRACKED_SHARDS],
    shard_count:        SigmaUsize,
}

impl SelfHealingEngine {
    pub const fn new() -> Self {
        Self {
            initialized:  false,
            safe_mode:    false,
            watchdog:     SigmaWatchdog::new(),
            panic_log:    PanicLog::new(),
            shard_health: [ShardHealth::empty(); MAX_TRACKED_SHARDS],
            shard_count:  0,
        }
    }

    pub unsafe fn init(&mut self) {
        if self.initialized { return; }

        // Arm hardware watchdog
        self.watchdog.arm();

        // Register kernel exception handlers
        kernel_register_panic_handler(handle_kernel_panic);
        kernel_register_nmi_handler(handle_nmi);

        // Check if we're recovering from a previous crash
        if kernel_crash_flag_is_set() {
            // Previous boot crashed — load crash record from non-volatile storage
            let crash_record = kernel_read_nvs_crash_record();
            self.panic_log.push(crash_record);

            // If crashed 3+ times consecutively, trigger safe mode
            if kernel_consecutive_crash_count() >= 3 {
                self.enter_safe_mode();
            }

            // Clear crash flag for this boot
            kernel_clear_crash_flag();
        }

        self.initialized = true;
    }

    /// Enter safe mode: minimal environment, debugging tools only
    pub unsafe fn enter_safe_mode(&mut self) {
        self.safe_mode = true;

        // Limit memory to safe mode budget
        kernel_mm_set_limit(SAFE_MODE_MEMORY_MB * 1024 * 1024);

        // Disable all optional shards
        kernel_shard_disable_optional_all();

        // Start only core shards: CoreLattice, NetworkDiag, SigmaShell
        kernel_shard_start_safe_set();

        // Launch recovery TUI
        kernel_launch_recovery_tui();
    }

    /// Called when a shard crashes — decide whether to restart or quarantine
    pub unsafe fn on_shard_crash(&mut self, shard_id: SigmaU32, exit_code: SigmaI32) {
        let health = self.get_or_create_health(shard_id);

        health.crash_count += 1;
        health.last_crash_ms = kernel_monotonic_ms();

        // Log panic record
        self.panic_log.push(PanicRecord {
            kind:         PanicKind::ShardCrash,
            timestamp_ms: health.last_crash_ms,
            cpu_id:       kernel_current_cpu(),
            rip:          0,
            rsp:          0,
            error_code:   exit_code as SigmaU64,
            message:      {
                let mut m = [0u8; 64];
                // Write "shard_crash:XXXXXXXX" into the buffer
                m[0..12].copy_from_slice(b"shard_crash:");
                let id_bytes = shard_id.to_ne_bytes();
                m[12..16].copy_from_slice(&id_bytes);
                m
            },
        });

        if health.crash_count >= MAX_SHARD_CRASHES {
            // Too many crashes — quarantine the shard
            health.quarantined = true;
            kernel_shard_quarantine(shard_id);
            // Notify user via Zenith notification system
            kernel_notify_user_shard_quarantined(shard_id);
        } else {
            // Attempt restart with exponential backoff
            let backoff_ms = 100u64 * (1u64 << health.crash_count.min(10));
            kernel_shard_restart_after(shard_id, backoff_ms);
            health.restart_count += 1;
        }
    }

    // ─── Internal helpers ───────────────────────────────────────────────────

    fn get_or_create_health(&mut self, shard_id: SigmaU32) -> &mut ShardHealth {
        // Find existing entry
        for i in 0..self.shard_count {
            if self.shard_health[i].shard_id == shard_id {
                return &mut self.shard_health[i];
            }
        }
        // Create new entry
        if self.shard_count < MAX_TRACKED_SHARDS {
            let idx = self.shard_count;
            self.shard_count += 1;
            self.shard_health[idx] = ShardHealth {
                shard_id,
                ..ShardHealth::empty()
            };
            &mut self.shard_health[idx]
        } else {
            // Ring-overwrite oldest entry (LRU eviction would be ideal but complex)
            self.shard_health[0] = ShardHealth {
                shard_id,
                ..ShardHealth::empty()
            };
            &mut self.shard_health[0]
        }
    }
}

// ─── Global Singletons ───────────────────────────────────────────────────────

static mut ENGINE: SelfHealingEngine = SelfHealingEngine::new();

// ─── C-ABI Exports ──────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn self_healing_init() {
    ENGINE.init();
}

#[no_mangle]
pub unsafe extern "C" fn trigger_safe_mode() {
    ENGINE.enter_safe_mode();
}

#[no_mangle]
pub unsafe extern "C" fn handle_kernel_panic(
    kind:       SigmaU32,
    rip:        SigmaU64,
    rsp:        SigmaU64,
    error_code: SigmaU64,
) {
    // Set NVS crash flag so next boot knows we crashed
    kernel_set_crash_flag();
    kernel_increment_consecutive_crash_count();

    // Save panic record to non-volatile storage
    let record = PanicRecord {
        kind:         PanicKind::PageFault,  // mapped from kind
        timestamp_ms: kernel_monotonic_ms(),
        cpu_id:       kernel_current_cpu(),
        rip,
        rsp,
        error_code,
        message:      {
            let mut m = [0u8; 64];
            m[..12].copy_from_slice(b"kernel_panic");
            m
        },
    };
    ENGINE.panic_log.push(record);
    kernel_write_nvs_crash_record(&record);

    // Attempt to dump kernel state to /sigma/crash/latest.dump
    kernel_dump_state(rip, rsp);

    // Allow watchdog to reset the system
    ENGINE.watchdog.disarm();

    // Trigger reboot after 5s to allow I/O flush
    kernel_reboot_after_ms(5000);
}

#[no_mangle]
pub unsafe extern "C" fn handle_nmi() {
    ENGINE.watchdog.on_watchdog_fire();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_watchdog_heartbeat() {
    ENGINE.watchdog.heartbeat();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_watchdog_tick() {
    // Called by hardware timer interrupt every 1ms
    let now_ms = kernel_monotonic_ms();
    let last   = ENGINE.watchdog.last_heartbeat_ms.load(Ordering::Relaxed);
    let timeout = ENGINE.watchdog.timeout_ms.load(Ordering::Relaxed) as SigmaU64;

    if ENGINE.watchdog.is_armed() && now_ms.saturating_sub(last) > timeout {
        ENGINE.watchdog.on_watchdog_fire();
    }
}

#[no_mangle]
pub unsafe extern "C" fn self_healing_on_shard_crash(shard_id: SigmaU32, exit_code: SigmaI32) {
    ENGINE.on_shard_crash(shard_id, exit_code);
}

#[no_mangle]
pub unsafe extern "C" fn self_healing_panic_count() -> SigmaU32 {
    ENGINE.panic_log.total_panics()
}

#[no_mangle]
pub unsafe extern "C" fn self_healing_in_safe_mode() -> SigmaBool {
    ENGINE.safe_mode
}

// ─── Hardware Abstraction Externs ────────────────────────────────────────────

extern "C" {
    fn kernel_watchdog_arm(timeout_ms: SigmaU32);
    fn kernel_watchdog_disarm();
    fn kernel_watchdog_pet(heartbeat: SigmaU64);
    fn kernel_monotonic_ms() -> SigmaU64;
    fn kernel_current_cpu() -> SigmaU32;
    fn kernel_register_panic_handler(f: unsafe extern "C" fn(SigmaU32, SigmaU64, SigmaU64, SigmaU64));
    fn kernel_register_nmi_handler(f: unsafe extern "C" fn());
    fn kernel_crash_flag_is_set() -> SigmaBool;
    fn kernel_set_crash_flag();
    fn kernel_clear_crash_flag();
    fn kernel_consecutive_crash_count() -> SigmaU32;
    fn kernel_increment_consecutive_crash_count();
    fn kernel_read_nvs_crash_record() -> PanicRecord;
    fn kernel_write_nvs_crash_record(record: *const PanicRecord);
    fn kernel_mm_set_limit(bytes: SigmaU64);
    fn kernel_shard_disable_optional_all();
    fn kernel_shard_start_safe_set();
    fn kernel_launch_recovery_tui();
    fn kernel_shard_quarantine(shard_id: SigmaU32);
    fn kernel_shard_restart_after(shard_id: SigmaU32, delay_ms: SigmaU64);
    fn kernel_notify_user_shard_quarantined(shard_id: SigmaU32);
    fn kernel_dump_state(rip: SigmaU64, rsp: SigmaU64);
    fn kernel_reboot_after_ms(delay_ms: SigmaU32);
    fn kernel_panic_log(kind: PanicKind, hb: SigmaU64, elapsed_ms: SigmaU64);
}
