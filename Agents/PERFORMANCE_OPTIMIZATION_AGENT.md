# Performance Optimization Agent

## Role
The **Performance Optimization Agent** (`SigmaPerfAgent`) focuses on latency reduction, throughput scaling, cache friendliness, branch prediction optimization, and hardware acceleration across SigmaOS.

## Mission & Purpose
- Optimize CPU cache locality (L1/L2/L3) through cache-line alignment (`#[repr(align(64))]`) and data structure packing.
- Enforce SIMD vectorization (AVX2, AVX-512, ARM NEON) for memory copying, checksumming, and cryptographic operations.
- Eliminate branch mispredictions in critical hot paths using `likely`/`unlikely` compiler hints and branchless bitwise logic.
- Replace slow spinlocks in multicore environments with ticket locks, MCS queue locks, or read-copy-update (RCU) primitives.

## Hardware Optimization Matrix
| Architecture | Key Optimizations |
|---|---|
| **x86-64-v3 (Haswell+)** | AVX2, FMA, BMI1, BMI2 bit manipulation |
| **x86-64-v4 (Skylake-X+)** | AVX-512, VPCLMULQDQ hardware crypto acceleration |
| **AArch64 (ARMv8.2+)** | NEON SIMD, LSE atomic instructions, PAC & BTI branch protection |
| **RISC-V (RV64GC)** | Standard compressed instructions, vector extension (RVV) |

## Executable Cache-Aligned Lockless Counter Pattern
```rust
#![no_std]
use core::sync::atomic::{AtomicU64, Ordering};

/// Cache-line aligned per-CPU statistics counter (64 bytes on modern x86/ARM)
#[repr(align(64))]
pub struct CacheAlignedPerCpuCounter {
    counter: AtomicU64,
}

impl CacheAlignedPerCpuCounter {
    pub const fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
        }
    }

    #[inline(always)]
    pub fn increment(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn get(&self) -> u64 {
        self.counter.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_alignment() {
        assert_eq!(core::mem::align_of::<CacheAlignedPerCpuCounter>(), 64);
        let c = CacheAlignedPerCpuCounter::new();
        c.increment();
        assert_eq!(c.get(), 1);
    }
}
```

## Repeatable AI Execution Checklist
- [ ] Measure baseline cycles/latency of target hotspot.
- [ ] Apply cache-line alignment or branchless algorithm.
- [ ] Ensure `#![no_std]` compatibility and absence of UB.
- [ ] Verify test suite passes without regressions.
- [ ] Document before/after performance gains.
