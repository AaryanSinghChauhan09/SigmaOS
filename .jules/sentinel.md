# Sentinel 🛡️ Security & Hardening Journal

## 2025-05-20 - Kernel Hardening TSS & Memory Alignment Protection
**Vulnerability:** Unaligned memory access on packed x86_64 Task State Segment (TSS64) struct fields during Ring 3/Ring 0 context switching validations.
**Learning:** Rust 2021 edition strictly warns against taking references to fields of `#[repr(C, packed)]` structs due to potential undefined behavior on strict-alignment architectures.
**Prevention:** Always read primitive values out of packed structs into value variables rather than creating references (`&engine.tss.rsp0`).
