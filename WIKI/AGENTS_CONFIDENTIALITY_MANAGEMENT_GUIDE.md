# SigmaOS AI Agent Confidentiality Operation Management Guide

This guide defines security protocols, hardware enclave memory protection standards, and zeroization mechanisms for AI agents managing data confidentiality and confidential computing across SigmaOS.

---

## 1. Core Principles of Kernel & Userland Confidentiality

SigmaOS enforces hardware-assisted confidential computing and strict secret isolation in `#![no_std]` environment. AI agents MUST observe the following confidentiality guidelines:

1. **Mandatory Memory Zeroization:** All sensitive buffers (cryptographic keys, passwords, private tokens, TLS session states) MUST implement automatic memory zeroization upon drop or deallocation using `volatile` writes (`core::ptr::write_volatile`).
2. **Constant-Time Comparison Operations:** Secrets and digests MUST be compared using constant-time algorithms (`subtle_constant_time_eq`) to eliminate side-channel timing leakages.
3. **Hardware Enclave Memory Protection:** In confidential VM environments (AMD SEV-SNP, Intel TDX, ARM CCA), private guest memory pages MUST be registered with hardware memory encryption gates and encrypted at rest in DRAM.
4. **Kernel Stack & Register Secret Scrubbing:** Upon returning from a system call or context switch, kernel thread stack frames containing secret data MUST be zeroed before yielding execution to untrusted userland tasks.

---

## 2. Zeroization & Secret Memory Scrubbing Protocol

AI agents managing sensitive kernel objects MUST derive or implement explicit zeroization guards:

```rust
pub struct SecureSecretBuffer<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> SecureSecretBuffer<N> {
    pub fn new(source: &[u8; N]) -> Self {
        Self { data: *source }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
}

impl<const N: usize> Drop for SecureSecretBuffer<N> {
    fn drop(&mut self) {
        // Volatile memory clearing prevents compiler dead-store elimination
        for byte in self.data.iter_mut() {
            unsafe {
                core::ptr::write_volatile(byte, 0x00);
            }
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
```

---

## 3. Constant-Time Equality Verification

When validating cryptographic MACs, signatures, or password hashes:

```rust
pub fn subtle_constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}
```

---

## 4. Confidential Computing (AMD SEV-SNP / Intel TDX)

1. **Page Encryption Bit (C-Bit / Shared Bit):** Memory pages marked as private in Page Map Level 4 (PML4) MUST set the hardware physical address encryption bit.
2. **Attestation State Measurement:** The guest launch measurement (SHA-384 / SHA-512) MUST be validated against the hardware Security Processor (PSP / TDX Module) prior to unsealing secret disk encryption keys.
3. **No Unencrypted Core Dumps:** Crash dumps, core dumps, or crash logs MUST NOT capture memory frames from protected confidential computing domains.
