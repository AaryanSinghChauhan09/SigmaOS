# SigmaOS Clock Algorithm, Timer & Timekeeping Guide for AI Agents

This guide provides technical specifications, clock Page Replacement hand-pointer algorithms, timer interrupt processing, and timekeeping invariants for AI agents managing clock subsystems in SigmaOS.

---

## 1. Zero-Dependency `#![no_std]` Clock & Timer Architecture

SigmaOS implements timekeeping, timer queue management, and clock replacement logic natively under `#![no_std]` Rust:

* **Clock Page Replacement Algorithm:**
  Scans physical memory page frames in a circular queue using a virtual hand pointer. If a page's reference bit is `1`, it is cleared to `0` and granted a second chance; if `0`, the page is selected for eviction.
* **Timer Interrupt & Monotonic Clock Processing:**
  Processes hardware clock interrupts, maintains high-resolution monotonic tick counters, and evaluates timer event expiration queues.

---

## 2. Timekeeping Invariants & Clock Rules

1. **Monotonicity:** Monotonic clock sources MUST NOT decrease or roll backwards.
2. **Clock Syscall Compliance:** System call handlers (`Gettimeofday`, `Nanosleep`, `Alarm`) MUST convert duration structures accurately without integer overflow.

---

## 3. Checklist for AI Agents Managing Clock Subsystems

1. **Verify #![no_std] Compatibility:** Ensure clock routines avoid external C libraries or `std::time`.
2. **Test Timekeeping Pipelines:**
   Run timer and timekeeping unit tests:
   ```bash
   ./run_sigma_tests.sh
   ```
