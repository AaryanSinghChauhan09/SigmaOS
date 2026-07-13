# Recovery & Forensics Specification (RescueZilla / CAINE Parity)

This specification outlines the partition backup, live memory capture, and sandboxed forensics analysis toolkits built directly into the SigmaOS kernel and userland tools.

---

## 💾 Partition Backup & Cloning (RescueZilla Parity)

The system recovery tool (`sigma-recover`) reads direct sector streams from block devices and writes compressed, encrypted backup files (`.simg`) to external or network drives.

```
                  ┌───────────────────────────────┐
                  │      sigma-recover Engine     │
                  └───────────────┬───────────────┘
                                  │
         ┌────────────────────────┼────────────────────────┐
         ▼                        ▼                        ▼
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
│  Sector Stream  │      │  Zstd Stream    │      │  Symmetric Key  │
│  (Direct Block  │ ───► │  Compression    │ ───► │  Encryption     │
│   Device I/O)   │      │  (Low Overhead) │      │  (AES-256-GCM)  │
└─────────────────┘      └─────────────────┘      └─────────────────┘
                                                           │
                                                           ▼
                                                [Encrypted .simg File]
```

---

## 🔍 Forensic Memory Dumping & Live Capture

SigmaOS provides low-level kernel hooks to generate secure memory dumps (`kdump` / Core Dumps) without exposing active system keys.

```rust
// kernel/src/diagnostics/kdump.rs
pub struct KernelMemoryDumper {
    target_buffer: *mut u8,
    buffer_size: usize,
}

impl KernelMemoryDumper {
    pub unsafe fn dump_physical_memory(&self) -> Result<usize, DumpError> {
        // Enumerate physical page frames (buddy allocator mapping)
        // Copy regions excluding kernel key rings and cryptographic slots
        // Write standard ELF core structure to target recovery partition
        Ok(self.buffer_size)
    }
}
```

---

## 🛡️ Sandboxed Forensic Containment (CAINE Parity)

The forensics utility `sigma-analyze` executes suspicious binaries inside a read-only namespace with restricted system capabilities.

1. **Mount Isolation**: Suspect drive images are mounted as **Read-Only** with loopback flags, protecting the original metadata.
2. **System Call Interception**: `sigma-trace` intercepts all file reads, writes, and network packets, outputting a structured JSON audit log for analysis.
3. **Execution Sandbox**: The suspect binary runs in a namespace where network access is disabled and filesystem interactions are redirected to a volatile in-memory overlay partition.
