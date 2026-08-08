//! # SigmaOS pledge() — OpenBSD-style syscall restriction
//!
//! Implements an OpenBSD `pledge(2)`-inspired mechanism for restricting which
//! syscalls a process may invoke after initialization.
//!
//! A process calls `pledge()` to declare the set of privileges it needs going
//! forward. Any attempt to use a privilege outside this set triggers an
//! immediate process termination (no signal, no recovery).
//!
//! ## References
//! - OpenBSD pledge(2): https://man.openbsd.org/pledge.2
//! - OpenBSD source: sys/kern/kern_pledge.c
//! - Fuchsia job policy: fuchsia.dev/fuchsia-src/concepts/process/jobs

#![no_std]

use core::sync::atomic::{AtomicU64, Ordering};

/// Pledge promise bits — each bit enables a category of syscalls.
/// Inspired directly by OpenBSD's pledge promise strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum PledgePromise {
    /// Basic stdio operations (read/write on already-open fds)
    Stdio        = 1 << 0,
    /// File-system read access
    Rpath        = 1 << 1,
    /// File-system write access
    Wpath        = 1 << 2,
    /// Create/delete files
    Cpath        = 1 << 3,
    /// DNS resolution
    Dns          = 1 << 4,
    /// Network I/O (TCP/UDP)
    Inet         = 1 << 5,
    /// Unix domain sockets
    Unix         = 1 << 6,
    /// Process management (fork, exec, wait)
    Proc         = 1 << 7,
    /// exec() a new binary
    Exec         = 1 << 8,
    /// Dynamic library loading
    Lib          = 1 << 9,
    /// sendfd/recvfd
    Sendfd       = 1 << 10,
    /// Memory-mapped files
    Mmap         = 1 << 11,
    /// Tape/audio devices
    Audio        = 1 << 12,
    /// Video devices
    Video        = 1 << 13,
    /// Virtual memory management
    Vminfo       = 1 << 14,
    /// Hardware random number generation
    Rndnum       = 1 << 15,
    /// Cryptographic operations
    Crypto       = 1 << 16,
    /// Error output to stderr (always enabled with Stdio)
    Errors       = 1 << 17,
    /// IPC capabilities (SigmaOS extension)
    Ipc          = 1 << 18,
    /// Capability token operations (SigmaOS extension)
    Capabilities = 1 << 19,
    /// Timer and clock access
    Clocks       = 1 << 20,
    /// Block device I/O
    Bdev         = 1 << 21,
    /// All promises (before first pledge call — unrestricted)
    All          = u64::MAX,
}

/// A set of pledge promises encoded as a bitmask
#[derive(Debug, Clone, Copy, Default)]
pub struct PledgeSet(pub u64);

impl PledgeSet {
    /// Empty promise set (no syscalls allowed except exit)
    pub const NONE: Self = Self(0);
    /// Full promise set (unrestricted — initial state before any pledge call)
    pub const ALL: Self = Self(u64::MAX);

    /// Create a promise set from a slice of promises
    pub const fn from_promises(promises: &[PledgePromise]) -> Self {
        let mut bits: u64 = 0;
        let mut i = 0;
        while i < promises.len() {
            bits |= promises[i] as u64;
            i += 1;
        }
        Self(bits)
    }

    /// Check if a specific promise is included
    #[inline]
    pub const fn has(&self, promise: PledgePromise) -> bool {
        self.0 & (promise as u64) != 0
    }

    /// Intersect two promise sets (can only reduce, never expand)
    #[inline]
    pub const fn intersect(&self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Returns true if the set is empty
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

/// Per-process pledge state
pub struct PledgeState {
    /// Current active promises (bitmask)
    promises: AtomicU64,
    /// Whether pledge has been called at least once (can only shrink after first call)
    pledged: AtomicU64, // 0 = not yet pledged, 1 = pledged
}

impl PledgeState {
    /// Initial state: no pledge applied (all promises open)
    pub const fn new() -> Self {
        Self {
            promises: AtomicU64::new(u64::MAX),
            pledged: AtomicU64::new(0),
        }
    }

    /// Apply a new pledge set.
    ///
    /// Rules (matching OpenBSD semantics):
    /// - The new set can only be a subset of the current set
    /// - Attempting to expand promises returns `Err(PledgeError::Expansion)`
    /// - Once pledged, you can only further restrict
    ///
    /// Returns `Ok(())` on success, `Err` if the promise set would expand.
    pub fn pledge(&self, new_promises: PledgeSet) -> Result<(), PledgeError> {
        let current = self.promises.load(Ordering::Acquire);
        // New promises must be a subset of current promises
        if new_promises.0 & !current != 0 {
            return Err(PledgeError::Expansion {
                current: PledgeSet(current),
                requested: new_promises,
            });
        }
        self.promises.store(new_promises.0, Ordering::Release);
        self.pledged.store(1, Ordering::Release);
        Ok(())
    }

    /// Check if a promise is currently active.
    /// This is called on every syscall entry — must be extremely fast.
    #[inline]
    pub fn check(&self, promise: PledgePromise) -> Result<(), PledgeError> {
        let bits = self.promises.load(Ordering::Relaxed);
        if bits & (promise as u64) != 0 {
            Ok(())
        } else {
            Err(PledgeError::Violation { violated: promise })
        }
    }

    /// Returns the current promise set
    pub fn current(&self) -> PledgeSet {
        PledgeSet(self.promises.load(Ordering::Relaxed))
    }

    /// Returns whether pledge() has been called at least once
    pub fn is_pledged(&self) -> bool {
        self.pledged.load(Ordering::Relaxed) != 0
    }
}

impl Default for PledgeState {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors returned by pledge operations
#[derive(Debug, Clone, Copy)]
pub enum PledgeError {
    /// Attempted to expand promises after pledging (not allowed)
    Expansion {
        current: PledgeSet,
        requested: PledgeSet,
    },
    /// Syscall attempted outside current promise set — process must be killed
    Violation {
        violated: PledgePromise,
    },
}

/// Parse a pledge promise string (e.g., "stdio rpath inet") into a PledgeSet.
/// Returns `None` if any unknown promise token is encountered.
pub fn parse_pledge_string(s: &str) -> Option<PledgeSet> {
    let mut bits: u64 = 0;
    for token in s.split_ascii_whitespace() {
        let p = match token {
            "stdio"        => PledgePromise::Stdio,
            "rpath"        => PledgePromise::Rpath,
            "wpath"        => PledgePromise::Wpath,
            "cpath"        => PledgePromise::Cpath,
            "dns"          => PledgePromise::Dns,
            "inet"         => PledgePromise::Inet,
            "unix"         => PledgePromise::Unix,
            "proc"         => PledgePromise::Proc,
            "exec"         => PledgePromise::Exec,
            "lib"          => PledgePromise::Lib,
            "sendfd"       => PledgePromise::Sendfd,
            "mmap"         => PledgePromise::Mmap,
            "audio"        => PledgePromise::Audio,
            "video"        => PledgePromise::Video,
            "vminfo"       => PledgePromise::Vminfo,
            "rndnum"       => PledgePromise::Rndnum,
            "crypto"       => PledgePromise::Crypto,
            "errors"       => PledgePromise::Errors,
            "ipc"          => PledgePromise::Ipc,
            "capabilities" => PledgePromise::Capabilities,
            "clocks"       => PledgePromise::Clocks,
            "bdev"         => PledgePromise::Bdev,
            _ => return None,
        };
        bits |= p as u64;
    }
    Some(PledgeSet(bits))
}
