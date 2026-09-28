# 🛡️ Sentinel's Security Journal

## 2026-03-31 - Packed Struct Memory Alignment under x86_64
**Learning:** Bypassing strict alignment checks in packed C-FFI structs when interfacing with kernel protection ring TSS structures can lead to general protection faults on strict x86_64 checks.
**Action:** Enforce `#[repr(C, packed)]` with explicit padding fields in security ring structures to guarantee strict byte alignment across kernel userland boundaries.
