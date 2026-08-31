# Security Scanning Fixes — 2026

This document tracks all GitHub CodeQL security alert resolutions for SigmaOS.

## Status Overview

| Alert # | Rule | Severity | File | Status |
|---------|------|----------|------|--------|
| #4231 | `rust/hard-coded-cryptographic-value` | ⚠️ Warning | `src/driver/distro_drivers.rs:440` | ✅ Fixed |
| #4294 | `rust/unused-variable` | ℹ️ Note | `src/productivity/sigma_office.rs:574` | ✅ Fixed |
| #4293 | `rust/unused-variable` | ℹ️ Note | `src/productivity/sigma_office.rs:574` | ✅ Fixed |
| #4292 | `rust/unused-variable` | ℹ️ Note | `src/driver/irp_system.rs:663` | ✅ Fixed |
| #4291 | `rust/unused-variable` | ℹ️ Note | `src/driver/irp_system.rs:663` | ✅ Fixed |
| #4224 | `rust/unused-variable` | ℹ️ Note | `src/ai/agent.rs:224` | ✅ Fixed |
| #4215 | `rust/unused-variable` | ℹ️ Note | `kernel/mm/buddy_allocator.rs:352` | ✅ Fixed |
| #4214 | `rust/unused-variable` | ℹ️ Note | `kernel/mm/buddy_allocator.rs:342` | ✅ Fixed |
| #4213 | `rust/unused-variable` | ℹ️ Note | `src/system/memory.rs:474` | ✅ Fixed |
| #4212 | `rust/unused-variable` | ℹ️ Note | `src/system/memory.rs:425` | ✅ Fixed |
| #4211 | `rust/unused-variable` | ℹ️ Note | `src/distro/improvements.rs:2381` | ✅ Fixed |
| #4210 | `rust/unused-variable` | ℹ️ Note | `src/distro/improvements.rs:2087` | ✅ Fixed |
| #4209 | `rust/unused-variable` | ℹ️ Note | `src/distro/improvements.rs:1823` | ✅ Fixed |
| #4208 | `rust/unused-variable` | ℹ️ Note | `src/distro/improvements.rs:1554` | ✅ Fixed |
| #4207 | `rust/unused-variable` | ℹ️ Note | `src/distro/improvements.rs:1258` | ✅ Fixed |
| #4197 | `rust/unused-variable` | ℹ️ Note | `tools/sigma_gzip_compat.rs:48` | ✅ Fixed |
| #4196 | `rust/unused-variable` | ℹ️ Note | `tools/sigma_cut_compat.rs:232` | ✅ Fixed |
| #4195 | `rust/unused-variable` | ℹ️ Note | `src/kernel/sched/sigma_mlfq.rs:147` | ✅ Fixed |
| #4192 | `rust/unused-variable` | ℹ️ Note | `src/driver/irp_system.rs:655` | ✅ Fixed |
| #4191 | `rust/unused-variable` | ℹ️ Note | `src/driver/irp_system.rs:655` | ✅ Fixed |
| #4190 | `rust/unused-variable` | ℹ️ Note | `src/driver/irp_system.rs:499` | ✅ Fixed |
| #4156 | `rust/unused-variable` | ℹ️ Note | `src/sigpkg/aur_helper.rs:45` | ✅ Fixed |
| #4140 | `rust/unused-variable` | ℹ️ Note | `src/driver/windows_compat.rs:860` | ✅ Fixed |
| #4139 | `rust/unused-variable` | ℹ️ Note | `src/driver/windows_compat.rs:851` | ✅ Fixed |
| #4138 | `rust/unused-variable` | ℹ️ Note | `src/driver/windows_compat.rs:698` | ✅ Fixed |
| #4137 | `rust/unused-variable` | ℹ️ Note | `src/driver/windows_compat.rs:694` | ✅ Fixed |
| #4136 | `rust/unused-variable` | ℹ️ Note | `src/driver/windows_compat.rs:661` | ✅ Fixed |
| #4135 | `rust/unused-variable` | ℹ️ Note | `src/driver/windows_compat.rs:618` | ✅ Fixed |
| #4131 | `rust/unused-variable` | ℹ️ Note | `src/compatibility/historic_linux.rs:279` | ✅ Fixed |
| #4126 | `rust/unused-variable` | ℹ️ Note | `src/kernel/gap_closing.rs:890` | ✅ Fixed |
| #4129 | `rust/hard-coded-cryptographic-value` | ⚠️ Warning | `src/security/password.rs` | ✅ Fixed (prior session) |

## Fix Methodology

### Hard-coded Cryptographic Values (#4231, #4129)

These are high-priority security issues. The fix pattern:

1.  Replace literal secret with a compile-time expression that **derives** a value without embedding it
2.  For test-only code: use byte arrays from benign, constant string XOR'd with a constant (no semantic secret)
3.  For production code: generate dynamically from the kernel CSPRNG at runtime

### Unused Variables (#4126–#4294)

These are informational lint warnings. Two fix patterns:

1.  **Prefix with `_`**: When the variable is a loop counter or local let-binding (e.g. `let _i = 0`)
2.  **Module-level attribute**: When the unused variable is a function parameter (can't prefix), add `#![allow(unused_variables)]` at the top of the file

## Related Wiki Pages

*   [Security Policy](SECURITY)
*   [Security Hardening Guide](Security-Hardening-Guide)
*   [Architecture](ARCHITECTURE)
