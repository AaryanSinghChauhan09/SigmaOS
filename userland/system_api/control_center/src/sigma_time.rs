//! SigmaOS Native Time Module
//! Replaces chrono dependency with simple timestamp functions

#![no_std]

use core::sync::atomic::{AtomicU64, Ordering};

/// Simple timestamp structure
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SigmaTimestamp {
    pub seconds: u64,
    pub nanos: u32,
}

/// Global monotonic counter for timestamps
static MONOTONIC_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Get current timestamp using RDTSC
#[inline]
pub fn get_timestamp() -> SigmaTimestamp {
    unsafe {
        let mut low: u32;
        let mut high: u32;
        core::arch::asm!(
            "rdtsc",
            out("eax") low,
            out("edx") high,
            options(nomem, nostack)
        );
        let tsc = ((high as u64) << 32) | (low as u64);
        
        // Convert to approximate seconds/nanos (assuming 3GHz CPU)
        // This is a rough approximation - for production use proper time source
        let seconds = tsc / 3_000_000_000;
        let remainder = tsc % 3_000_000_000;
        let nanos = (remainder * 1_000_000_000 / 3_000_000_000) as u32;
        
        SigmaTimestamp { seconds, nanos }
    }
}

/// Get monotonic counter value
#[inline]
pub fn get_monotonic() -> u64 {
    MONOTONIC_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Simple delay function (busy-wait)
pub fn delay_nanos(nanos: u32) {
    // Very rough approximation using CPU cycles
    // For production, use proper timer
    let cycles = (nanos as u64) * 3; // Assuming 3 cycles per nanosecond
    let start = get_monotonic();
    while get_monotonic() - start < cycles {
        core::hint::spin_loop();
    }
}

/// Simple delay in milliseconds
pub fn delay_ms(ms: u32) {
    delay_nanos(ms * 1_000_000);
}

/// Simple delay in seconds
pub fn delay_secs(secs: u32) {
    for _ in 0..secs {
        delay_ms(1000);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let ts1 = get_timestamp();
        let ts2 = get_timestamp();
        assert!(ts2.seconds >= ts1.seconds);
    }

    #[test]
    fn test_monotonic() {
        let m1 = get_monotonic();
        let m2 = get_monotonic();
        assert!(m2 > m1);
    }
}
