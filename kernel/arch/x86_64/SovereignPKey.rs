/// SigmaOS Sovereign Protection Key (PKey) Manager
/// Migrated from C++ to Rust — no_std, no alloc, no external crates.
/// Enables Single Address Space OS (SASOS) via Intel MPK (Memory Protection Keys).

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU32 = u32;

// ─── PKey Rights ────────────────────────────────────────────────────────────

/// Protection key access rights.
/// Each key controls 2 bits in PKRU: Access Disable (AD) and Write Disable (WD).
struct PKeyRights;
impl PKeyRights {
    /// Full access (no restrictions).
    const FULL_ACCESS: SigmaU32    = 0x00;
    /// Write disabled, reads allowed.
    const WRITE_DISABLE: SigmaU32  = 0x01;
    /// Access disabled (no read or write).
    const ACCESS_DISABLE: SigmaU32 = 0x02;
    /// Fully locked (both AD and WD set).
    const LOCKED: SigmaU32         = 0x03;
}

/// Maximum number of protection keys supported by Intel MPK hardware.
const MAX_PKEYS: usize = 16;

// ─── SovereignPKeyManager ───────────────────────────────────────────────────

/// The Sovereign Protection Key Manager.
/// Manages allocation of hardware memory protection keys and
/// the PKRU (Protection Key Rights for User pages) register.
struct SovereignPKeyManager {
    /// Number of keys currently allocated (0..16).
    active_keys: usize,
    /// Per-key rights (2 bits each, packed into PKRU).
    rights: [SigmaU32; MAX_PKEYS],
    /// Whether the PKey subsystem has been initialized.
    initialized: bool,
}

impl SovereignPKeyManager {
    /// Construct a new PKey manager with no keys allocated.
    const fn new() -> Self {
        Self {
            active_keys: 0,
            rights:      [0; MAX_PKEYS],
            initialized: false,
        }
    }

    /// Initialize the PKey subsystem.
    /// Enables CR4.PKE (bit 22) to activate Memory Protection Keys.
    unsafe fn init(&mut self) {
        // Enable PKE in CR4
        let mut cr4: u64;
        core::arch::asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack));
        cr4 |= 1 << 22; // CR4.PKE
        core::arch::asm!("mov cr4, {}", in(reg) cr4, options(nomem, nostack));

        self.active_keys = 0;
        self.initialized = true;
    }

    /// Allocate the next available protection key.
    /// Returns the key index (0..15) or -1 if all keys are exhausted.
    fn allocate_key(&mut self) -> i32 {
        if self.active_keys >= MAX_PKEYS {
            return -1;
        }
        let key = self.active_keys as i32;
        self.active_keys += 1;
        key
    }

    /// Set the access rights for a specific protection key.
    /// Uses the WRPKRU instruction to update the PKRU register.
    unsafe fn set_protection(&mut self, key: i32, rights: SigmaU32) {
        if key < 0 || key >= MAX_PKEYS as i32 {
            return;
        }
        self.rights[key as usize] = rights;

        // Build the full PKRU value from all key rights
        let mut pkru: SigmaU32 = 0;
        let mut i: usize = 0;
        while i < MAX_PKEYS {
            pkru |= (self.rights[i] & 0x03) << (i * 2);
            i += 1;
        }

        // WRPKRU: EAX = PKRU value, ECX = 0, EDX = 0
        core::arch::asm!(
            "wrpkru",
            in("eax") pkru,
            in("ecx") 0u32,
            in("edx") 0u32,
            options(nomem, nostack),
        );
    }

    /// Read the current PKRU register value.
    unsafe fn read_pkru(&self) -> SigmaU32 {
        let pkru: SigmaU32;
        core::arch::asm!(
            "rdpkru",
            out("eax") pkru,
            in("ecx") 0u32,
            out("edx") _,
            options(nomem, nostack),
        );
        pkru
    }
}

// ─── Global Singleton ───────────────────────────────────────────────────────

static mut PKEY_MANAGER: SovereignPKeyManager = SovereignPKeyManager::new();

// ─── C-ABI Bridge ───────────────────────────────────────────────────────────

/// Initialize the PKey subsystem.
/// Replaces `extern "C" void pkey_init()`.
#[no_mangle]
pub unsafe extern "C" fn pkey_init() {
    PKEY_MANAGER.init();
}

/// Allocate a new protection key.
/// Replaces `extern "C" int pkey_alloc()`.
#[no_mangle]
pub unsafe extern "C" fn pkey_alloc() -> i32 {
    PKEY_MANAGER.allocate_key()
}

/// Set rights for a protection key.
/// Replaces `extern "C" void pkey_set(int key, sigma_u32 rights)`.
#[no_mangle]
pub unsafe extern "C" fn pkey_set(key: i32, rights: u32) {
    PKEY_MANAGER.set_protection(key, rights);
}
