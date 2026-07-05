/// SigmaOS — modules/core/kernel/capability.rs
/// 64-bit bitmask capability token system for the Sovereign kernel.
/// Grants/revokes/checks capabilities on shard-to-shard IPC boundaries.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU64   = u64;
type SigmaU32   = u32;
type SigmaUsize = usize;
type SigmaBool  = bool;

// ─── Capability Constants (bit positions) ────────────────────────────────────
// Each bit in a 64-bit token represents one sovereign capability.

pub const CAP_NONE:           SigmaU64 = 0;
pub const CAP_KERNEL_ROOT:    SigmaU64 = 1 << 0;   // Full kernel access (boot only)
pub const CAP_SHARD_SPAWN:    SigmaU64 = 1 << 1;   // Spawn new shards
pub const CAP_SHARD_KILL:     SigmaU64 = 1 << 2;   // Kill foreign shards
pub const CAP_IPC_SEND:       SigmaU64 = 1 << 3;   // Send IPC messages
pub const CAP_IPC_RECV:       SigmaU64 = 1 << 4;   // Receive IPC messages
pub const CAP_MEM_ALLOC:      SigmaU64 = 1 << 5;   // Allocate physical pages
pub const CAP_MEM_MAP:        SigmaU64 = 1 << 6;   // Map physical → virtual
pub const CAP_IRQ_BIND:       SigmaU64 = 1 << 7;   // Register IRQ handlers
pub const CAP_VFS_READ:       SigmaU64 = 1 << 8;   // Read from VFS
pub const CAP_VFS_WRITE:      SigmaU64 = 1 << 9;   // Write to VFS
pub const CAP_VFS_MOUNT:      SigmaU64 = 1 << 10;  // Mount filesystems
pub const CAP_NET_BIND:       SigmaU64 = 1 << 11;  // Bind privileged ports (<1024)
pub const CAP_NET_CONNECT:    SigmaU64 = 1 << 12;  // Outbound network connections
pub const CAP_NET_RECV:       SigmaU64 = 1 << 13;  // Receive raw network packets
pub const CAP_PCI_ACCESS:     SigmaU64 = 1 << 14;  // Direct PCI config space access
pub const CAP_DMA_ALLOC:      SigmaU64 = 1 << 15;  // Allocate DMA-coherent memory
pub const CAP_MODULE_LOAD:    SigmaU64 = 1 << 16;  // Load kernel modules / shards
pub const CAP_MODULE_UNLOAD:  SigmaU64 = 1 << 17;  // Unload kernel modules
pub const CAP_AUDIT_READ:     SigmaU64 = 1 << 18;  // Read audit chain
pub const CAP_AUDIT_WRITE:    SigmaU64 = 1 << 19;  // Append audit entries
pub const CAP_PTRACE:         SigmaU64 = 1 << 20;  // Debug / trace other shards
pub const CAP_CLOCK_SET:      SigmaU64 = 1 << 21;  // Set system clock
pub const CAP_POWER_CTRL:     SigmaU64 = 1 << 22;  // Reboot / shutdown
pub const CAP_CRYPTO_KEY:     SigmaU64 = 1 << 23;  // Access sovereign key store
pub const CAP_SECBOOT_VERIFY: SigmaU64 = 1 << 24;  // Verify secure-boot signatures
pub const CAP_SANDBOX_CREATE: SigmaU64 = 1 << 25;  // Create sandboxed containers
pub const CAP_GPU_ACCESS:     SigmaU64 = 1 << 26;  // GPU framebuffer / compute
pub const CAP_USB_ACCESS:     SigmaU64 = 1 << 27;  // USB host controller
pub const CAP_POLICY_LOAD:    SigmaU64 = 1 << 28;  // Load MAC policy rules
pub const CAP_TELEMETRY:      SigmaU64 = 1 << 29;  // Emit system telemetry
pub const CAP_HYPERVISOR:     SigmaU64 = 1 << 30;  // VT-x / AMD-V VM operations
pub const CAP_USER_31:        SigmaU64 = 1 << 31;  // Reserved for userland use

/// Convenience set: minimal userland app (no kernel / hardware access)
pub const CAP_SET_USERAPP: SigmaU64 =
    CAP_IPC_SEND | CAP_IPC_RECV | CAP_VFS_READ | CAP_VFS_WRITE | CAP_NET_CONNECT;

/// Convenience set: system daemon (network + FS + audit)
pub const CAP_SET_DAEMON: SigmaU64 =
    CAP_SET_USERAPP | CAP_NET_BIND | CAP_NET_RECV | CAP_AUDIT_WRITE | CAP_TELEMETRY;

/// Convenience set: kernel module / driver
pub const CAP_SET_DRIVER: SigmaU64 =
    CAP_PCI_ACCESS | CAP_DMA_ALLOC | CAP_IRQ_BIND | CAP_MEM_ALLOC | CAP_MEM_MAP;

// ─── Capability Token ─────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CapToken {
    /// Bitmask of granted capabilities
    pub caps: SigmaU64,
    /// Shard ID that owns this token
    pub owner_shard: SigmaU32,
    /// Generation counter — incremented on revocation
    pub generation: SigmaU32,
}

impl CapToken {
    /// Create a new token with the given capability bitmask.
    pub const fn new(caps: SigmaU64, owner: SigmaU32) -> Self {
        CapToken { caps, owner_shard: owner, generation: 0 }
    }

    /// Check whether this token holds a specific capability.
    #[inline]
    pub fn has(&self, cap: SigmaU64) -> SigmaBool {
        (self.caps & cap) == cap
    }

    /// Grant additional capabilities (only the kernel may call this).
    #[inline]
    pub fn grant(&mut self, cap: SigmaU64) {
        self.caps |= cap;
    }

    /// Revoke a capability and increment the generation counter.
    #[inline]
    pub fn revoke(&mut self, cap: SigmaU64) {
        self.caps &= !cap;
        self.generation = self.generation.wrapping_add(1);
    }

    /// Derive a child token with a *subset* of our capabilities (cannot escalate).
    pub fn derive(&self, requested: SigmaU64, child_shard: SigmaU32) -> CapToken {
        CapToken {
            caps:        self.caps & requested,  // intersection only
            owner_shard: child_shard,
            generation:  0,
        }
    }
}

// ─── Null / Root tokens ───────────────────────────────────────────────────────

pub const CAP_TOKEN_NULL: CapToken = CapToken { caps: 0, owner_shard: 0, generation: 0 };
pub const CAP_TOKEN_ROOT: CapToken = CapToken { caps: !0u64, owner_shard: 0, generation: 0 };

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

/// Check whether `token` grants `cap`. Returns 1 if yes, 0 if no.
#[no_mangle]
pub unsafe extern "C" fn cap_check(token: *const CapToken, cap: SigmaU64) -> i32 {
    if token.is_null() { return 0; }
    if (*token).has(cap) { 1 } else { 0 }
}

/// Grant `cap` bits to an existing token (kernel-only: caller must hold CAP_KERNEL_ROOT).
#[no_mangle]
pub unsafe extern "C" fn cap_grant(
    caller: *const CapToken,
    target: *mut CapToken,
    cap: SigmaU64,
) -> i32 {
    if caller.is_null() || target.is_null() { return -1; }
    if !(*caller).has(CAP_KERNEL_ROOT) { return -3; }   // EPERM
    (*target).grant(cap);
    0
}

/// Revoke `cap` bits from a token.
#[no_mangle]
pub unsafe extern "C" fn cap_revoke(
    caller: *const CapToken,
    target: *mut CapToken,
    cap: SigmaU64,
) -> i32 {
    if caller.is_null() || target.is_null() { return -1; }
    if !(*caller).has(CAP_KERNEL_ROOT) { return -3; }
    (*target).revoke(cap);
    0
}

/// Derive a child token. Writes result into `out`. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn cap_derive(
    parent: *const CapToken,
    requested: SigmaU64,
    child_shard: SigmaU32,
    out: *mut CapToken,
) -> i32 {
    if parent.is_null() || out.is_null() { return -1; }
    *out = (*parent).derive(requested, child_shard);
    0
}
