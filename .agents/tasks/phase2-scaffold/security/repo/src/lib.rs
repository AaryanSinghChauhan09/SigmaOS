//! SigmaOS Security Subsystem
//!
//! Provides LSM hooks, privilege separation, capability-based access control,
//! and cryptographic context primitives.
//!
//! # Design Principles
//! - Least privilege: processes receive the minimum capabilities required.
//! - Secure defaults: denial is the default; access must be explicitly granted.
//! - Process isolation: privilege boundaries are enforced between processes.
//! - Fail-fast: policy violations surface as errors immediately.
//! - Memory safety: all interfaces are safe Rust; no raw pointer exposure.

// ── Inline stub for filesystem::vfs types ──────────────────────────────────
//
// The upstream SigmaOS lsm.rs depends on `crate::filesystem::vfs::{FsError, Inode}`.
// This standalone crate provides minimal stubs so the security subsystem compiles
// independently of SigmaOS-kernel.  When integrated via Cargo workspace the workspace
// can replace these with the real types.

/// Minimal stub for a virtual-filesystem inode.
///
/// Production code should use `sigmaos-kernel::filesystem::vfs::Inode`.
#[derive(Debug, Clone)]
pub struct Inode {
    /// Inode number.
    pub ino: u64,
    /// File mode bits (permission + type).
    pub mode: u32,
    /// Owner UID.
    pub uid: u32,
    /// Owner GID.
    pub gid: u32,
}

impl Inode {
    /// Construct a stub inode for testing.
    pub fn new(ino: u64, mode: u32, uid: u32, gid: u32) -> Self {
        Self { ino, mode, uid, gid }
    }
}

// ── Security error type ─────────────────────────────────────────────────────

/// Errors returned by security enforcement points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SecurityError {
    #[error("permission denied")]
    PermissionDenied,
    #[error("required capability missing")]
    CapabilityMissing,
    #[error("policy violation")]
    PolicyViolation,
    #[error("security label invalid")]
    LabelInvalid,
}

// ── Inode attribute struct (LSM helper) ────────────────────────────────────

/// Attributes that may be set on an inode (used by LSM setattr hook).
#[derive(Debug, Clone)]
pub struct InodeAttr {
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
}

// ── LSM Hook trait ─────────────────────────────────────────────────────────

/// Linux Security Module hook interface.
///
/// Implementors intercept kernel operations and enforce policy.
/// Every method returns `Ok(())` to allow or `Err(SecurityError)` to deny.
pub trait LsmHook: Send + Sync {
    fn security_inode_create(
        &self,
        dir: &Inode,
        name: &str,
        mode: u32,
    ) -> Result<(), SecurityError>;

    fn security_inode_link(
        &self,
        old_inode: &Inode,
        new_dir: &Inode,
        new_name: &str,
    ) -> Result<(), SecurityError>;

    fn security_inode_unlink(&self, dir: &Inode, name: &str) -> Result<(), SecurityError>;

    fn security_inode_rename(
        &self,
        old_dir: &Inode,
        old_name: &str,
        new_dir: &Inode,
        new_name: &str,
    ) -> Result<(), SecurityError>;

    fn security_inode_setattr(
        &self,
        inode: &Inode,
        attr: &InodeAttr,
    ) -> Result<(), SecurityError>;

    fn security_inode_getattr(&self, inode: &Inode) -> Result<(), SecurityError>;

    fn security_file_open(&self, inode: &Inode, flags: u32) -> Result<(), SecurityError>;

    fn security_file_permission(&self, inode: &Inode, mask: u32) -> Result<(), SecurityError>;

    fn security_socket_create(
        &self,
        family: u32,
        type_: u32,
        protocol: u32,
    ) -> Result<(), SecurityError>;

    fn security_socket_bind(&self, sock: usize, addr: &[u8]) -> Result<(), SecurityError>;

    fn security_socket_connect(&self, sock: usize, addr: &[u8]) -> Result<(), SecurityError>;

    fn security_socket_listen(&self, sock: usize, backlog: u32) -> Result<(), SecurityError>;

    fn security_socket_accept(&self, sock: usize) -> Result<(), SecurityError>;

    fn security_socket_sendmsg(&self, sock: usize) -> Result<(), SecurityError>;

    fn security_socket_recvmsg(&self, sock: usize) -> Result<(), SecurityError>;

    fn security_ptrace_access_check(
        &self,
        tracer: u64,
        tracee: u64,
    ) -> Result<(), SecurityError>;

    fn security_ptrace_traceme(&self, tracer: u64) -> Result<(), SecurityError>;

    fn security_capable(&self, cred: u64, cap: u64) -> Result<(), SecurityError>;

    fn security_bprm_set_uid(&self, bprm: usize) -> Result<(), SecurityError>;

    fn security_inode_mknod(
        &self,
        dir: &Inode,
        name: &str,
        mode: u32,
    ) -> Result<(), SecurityError>;

    fn security_inode_symlink(
        &self,
        dir: &Inode,
        name: &str,
        target: &str,
    ) -> Result<(), SecurityError>;
}

// ── Capability enum ────────────────────────────────────────────────────────

/// Coarse-grained capability taxonomy used by SigmaOS privilege model.
///
/// Maps to Linux POSIX capabilities but expressed at the policy level so that
/// higher-level components remain independent of kernel ABI constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Read files and directories.
    Read,
    /// Write files and directories.
    Write,
    /// Execute binaries.
    Execute,
    /// Open network connections.
    Network,
    /// Mount and unmount filesystems.
    Mount,
    /// Privileged / administrative operations.
    Admin,
}

impl Capability {
    /// Returns `true` if this capability grants elevated system access.
    pub fn is_privileged(self) -> bool {
        matches!(self, Self::Mount | Self::Admin)
    }
}

// ── CapabilitySet ──────────────────────────────────────────────────────────

/// Set of capabilities held by a subject (process, component, or sandbox).
///
/// Internally stores capabilities as a bitmask for O(1) lookup.
#[derive(Debug, Clone, Default)]
pub struct CapabilitySet {
    bits: u64,
}

impl CapabilitySet {
    /// Create an empty capability set (deny-all by default).
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    /// Create a set containing the given capabilities.
    pub fn from_caps(caps: &[Capability]) -> Self {
        let mut set = Self::new();
        for &cap in caps {
            set.grant(cap);
        }
        set
    }

    fn bit(cap: Capability) -> u64 {
        1u64 << (cap as u64)
    }

    /// Grant a capability.
    pub fn grant(&mut self, cap: Capability) {
        self.bits |= Self::bit(cap);
    }

    /// Revoke a capability.
    pub fn revoke(&mut self, cap: Capability) {
        self.bits &= !Self::bit(cap);
    }

    /// Returns `true` if the set contains the given capability.
    pub fn contains(&self, cap: Capability) -> bool {
        (self.bits & Self::bit(cap)) != 0
    }

    /// Returns `true` if the set is empty (no capabilities granted).
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    /// Returns the raw bitmask.
    pub fn bits(&self) -> u64 {
        self.bits
    }
}

// ── PrivilegeSeparator ─────────────────────────────────────────────────────

/// Enforces privilege boundaries between processes.
///
/// A `PrivilegeSeparator` holds the capabilities available to a named
/// process and provides checked access: callers must prove they hold a
/// capability before the separator permits the operation.
#[derive(Debug)]
pub struct PrivilegeSeparator {
    /// Name of the process/component this separator guards.
    pub process_name: String,
    /// Capabilities this process is allowed to exercise.
    capabilities: CapabilitySet,
    /// Whether `PR_SET_NO_NEW_PRIVS` has been applied (no escalation).
    no_new_privs: bool,
}

impl PrivilegeSeparator {
    /// Create a new separator with an empty (deny-all) capability set.
    pub fn new(process_name: impl Into<String>) -> Self {
        Self {
            process_name: process_name.into(),
            capabilities: CapabilitySet::new(),
            no_new_privs: false,
        }
    }

    /// Grant a capability to this process.
    pub fn grant(&mut self, cap: Capability) {
        self.capabilities.grant(cap);
    }

    /// Revoke a capability from this process.
    pub fn revoke(&mut self, cap: Capability) {
        self.capabilities.revoke(cap);
    }

    /// Check whether the process holds `cap`, returning an error on denial.
    pub fn check(&self, cap: Capability) -> Result<(), SecurityError> {
        if self.capabilities.contains(cap) {
            Ok(())
        } else {
            Err(SecurityError::CapabilityMissing)
        }
    }

    /// Apply `no_new_privs` — the process may not gain capabilities via exec.
    /// Once set this cannot be cleared (fail-fast, secure-default).
    pub fn apply_no_new_privs(&mut self) {
        self.no_new_privs = true;
    }

    /// Returns `true` if no-new-privs is active.
    pub fn has_no_new_privs(&self) -> bool {
        self.no_new_privs
    }

    /// Returns a reference to the underlying capability set.
    pub fn capability_set(&self) -> &CapabilitySet {
        &self.capabilities
    }
}

// ── CryptoContext ──────────────────────────────────────────────────────────

/// Errors returned by cryptographic operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum CryptoError {
    #[error("random generation failed")]
    RandomGenerationFailed,
    #[error("invalid key length")]
    InvalidKeyLength,
    #[error("invalid nonce length")]
    InvalidNonce,
    #[error("encryption failed")]
    EncryptionFailed,
    #[error("decryption failed: authentication tag mismatch")]
    DecryptionFailed,
}

/// Cryptographic context for a security domain.
///
/// Provides key generation, nonce generation, and stub AEAD
/// encrypt/decrypt operations.  In production these stubs should be
/// replaced with a vetted crate (e.g., `ring` or `aes-gcm`).
pub struct CryptoContext {
    /// PRNG state (LCG — for stubs/tests only; replace with CSPRNG in production).
    prng_state: u64,
}

impl CryptoContext {
    /// Create a new context with a time-derived seed on hosted targets,
    /// or a fixed seed on bare-metal targets.
    pub fn new() -> Self {
        #[cfg(not(target_os = "none"))]
        let seed: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x5a5a_5a5a_5a5a_5a5a);
        #[cfg(target_os = "none")]
        let seed: u64 = 0x5a5a_5a5a_5a5a_5a5a;
        Self { prng_state: seed }
    }

    /// Fill `buf` with pseudo-random bytes (LCG — stub only).
    pub fn fill_random(&mut self, buf: &mut [u8]) -> Result<(), CryptoError> {
        for byte in buf.iter_mut() {
            self.prng_state = self
                .prng_state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            *byte = (self.prng_state >> 32) as u8;
        }
        Ok(())
    }

    /// Generate a cryptographic key of `len` bytes.
    pub fn generate_key(&mut self, len: usize) -> Result<Vec<u8>, CryptoError> {
        if len == 0 {
            return Err(CryptoError::InvalidKeyLength);
        }
        let mut key = vec![0u8; len];
        self.fill_random(&mut key)?;
        Ok(key)
    }

    /// Generate a random nonce of `len` bytes (1–64 bytes).
    pub fn generate_nonce(&mut self, len: usize) -> Result<Vec<u8>, CryptoError> {
        if len == 0 || len > 64 {
            return Err(CryptoError::InvalidNonce);
        }
        self.generate_key(len)
    }

    /// Stub AEAD encryption: XORs plaintext with the key (cycled) and appends
    /// a trivial 16-byte "tag".  Replace with a real AEAD cipher in production.
    pub fn encrypt(
        &self,
        key: &[u8],
        _nonce: &[u8],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        if key.is_empty() {
            return Err(CryptoError::EncryptionFailed);
        }
        let mut ciphertext: Vec<u8> = plaintext
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ key[i % key.len()])
            .collect();
        // Append a trivial 16-byte authentication tag (stub).
        ciphertext.extend_from_slice(&[0xde, 0xad, 0xbe, 0xef,
                                       0xca, 0xfe, 0xba, 0xbe,
                                       0x01, 0x23, 0x45, 0x67,
                                       0x89, 0xab, 0xcd, 0xef]);
        Ok(ciphertext)
    }

    /// Stub AEAD decryption: strips the trailing 16-byte tag then XORs with key.
    pub fn decrypt(
        &self,
        key: &[u8],
        _nonce: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        if key.is_empty() {
            return Err(CryptoError::DecryptionFailed);
        }
        if ciphertext.len() < 16 {
            return Err(CryptoError::DecryptionFailed);
        }
        let payload = &ciphertext[..ciphertext.len() - 16];
        let plaintext: Vec<u8> = payload
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ key[i % key.len()])
            .collect();
        Ok(plaintext)
    }

    /// Constant-time byte-slice comparison (CWE-208).
    pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        let mut result = (a.len() != b.len()) as u8;
        for (&x, &y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        core::hint::black_box(result) == 0
    }
}

impl Default for CryptoContext {
    fn default() -> Self {
        Self::new()
    }
}

// ── Default allow-all LSM (for testing/composing) ─────────────────────────

/// A no-op LSM hook implementation that permits all operations.
///
/// Useful as a baseline or for testing hook composition.
pub struct AllowAllLsm;

impl LsmHook for AllowAllLsm {
    fn security_inode_create(&self, _: &Inode, _: &str, _: u32) -> Result<(), SecurityError> { Ok(()) }
    fn security_inode_link(&self, _: &Inode, _: &Inode, _: &str) -> Result<(), SecurityError> { Ok(()) }
    fn security_inode_unlink(&self, _: &Inode, _: &str) -> Result<(), SecurityError> { Ok(()) }
    fn security_inode_rename(&self, _: &Inode, _: &str, _: &Inode, _: &str) -> Result<(), SecurityError> { Ok(()) }
    fn security_inode_setattr(&self, _: &Inode, _: &InodeAttr) -> Result<(), SecurityError> { Ok(()) }
    fn security_inode_getattr(&self, _: &Inode) -> Result<(), SecurityError> { Ok(()) }
    fn security_file_open(&self, _: &Inode, _: u32) -> Result<(), SecurityError> { Ok(()) }
    fn security_file_permission(&self, _: &Inode, _: u32) -> Result<(), SecurityError> { Ok(()) }
    fn security_socket_create(&self, _: u32, _: u32, _: u32) -> Result<(), SecurityError> { Ok(()) }
    fn security_socket_bind(&self, _: usize, _: &[u8]) -> Result<(), SecurityError> { Ok(()) }
    fn security_socket_connect(&self, _: usize, _: &[u8]) -> Result<(), SecurityError> { Ok(()) }
    fn security_socket_listen(&self, _: usize, _: u32) -> Result<(), SecurityError> { Ok(()) }
    fn security_socket_accept(&self, _: usize) -> Result<(), SecurityError> { Ok(()) }
    fn security_socket_sendmsg(&self, _: usize) -> Result<(), SecurityError> { Ok(()) }
    fn security_socket_recvmsg(&self, _: usize) -> Result<(), SecurityError> { Ok(()) }
    fn security_ptrace_access_check(&self, _: u64, _: u64) -> Result<(), SecurityError> { Ok(()) }
    fn security_ptrace_traceme(&self, _: u64) -> Result<(), SecurityError> { Ok(()) }
    fn security_capable(&self, _: u64, _: u64) -> Result<(), SecurityError> { Ok(()) }
    fn security_bprm_set_uid(&self, _: usize) -> Result<(), SecurityError> { Ok(()) }
    fn security_inode_mknod(&self, _: &Inode, _: &str, _: u32) -> Result<(), SecurityError> { Ok(()) }
    fn security_inode_symlink(&self, _: &Inode, _: &str, _: &str) -> Result<(), SecurityError> { Ok(()) }
}

/// A deny-all LSM hook implementation that rejects every operation.
///
/// Useful as a policy that must be overridden by a concrete implementation.
pub struct DenyAllLsm;

impl LsmHook for DenyAllLsm {
    fn security_inode_create(&self, _: &Inode, _: &str, _: u32) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_inode_link(&self, _: &Inode, _: &Inode, _: &str) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_inode_unlink(&self, _: &Inode, _: &str) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_inode_rename(&self, _: &Inode, _: &str, _: &Inode, _: &str) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_inode_setattr(&self, _: &Inode, _: &InodeAttr) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_inode_getattr(&self, _: &Inode) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_file_open(&self, _: &Inode, _: u32) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_file_permission(&self, _: &Inode, _: u32) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_socket_create(&self, _: u32, _: u32, _: u32) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_socket_bind(&self, _: usize, _: &[u8]) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_socket_connect(&self, _: usize, _: &[u8]) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_socket_listen(&self, _: usize, _: u32) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_socket_accept(&self, _: usize) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_socket_sendmsg(&self, _: usize) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_socket_recvmsg(&self, _: usize) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_ptrace_access_check(&self, _: u64, _: u64) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_ptrace_traceme(&self, _: u64) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_capable(&self, _: u64, _: u64) -> Result<(), SecurityError> { Err(SecurityError::CapabilityMissing) }
    fn security_bprm_set_uid(&self, _: usize) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_inode_mknod(&self, _: &Inode, _: &str, _: u32) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
    fn security_inode_symlink(&self, _: &Inode, _: &str, _: &str) -> Result<(), SecurityError> { Err(SecurityError::PermissionDenied) }
}

// All public types defined above are accessible directly from the crate root.
// No re-export block needed for intra-module items.
