# SigmaOS Timer Subsystem

## Overview

The SigmaOS timer subsystem provides high-resolution kernel timekeeping with nanosecond precision. It is fully sovereign — no dependency on libc, POSIX, or platform-specific timer APIs.

**Location:** `src/kernel/sigma_timer.rs`

---

## Architecture

### Clock Sources

| Source | Resolution | Notes |
|--------|-----------|-------|
| `Tsc` | ~1ns | CPU Time Stamp Counter (x86 RDTSC/RDTSCP) |
| `Hpet` | ~100ns | High Precision Event Timer (chipset) |
| `Pit` | ~838ns | Legacy 8253/8254 Programmable Interval Timer |
| `Rtc` | 1s | Real-Time Clock — wall time only |
| `Monotonic` | tick-based | Software fallback |

### Hierarchical Timing Wheel

SigmaOS uses a two-level timing wheel, inspired by Linux `kernel/timer.c` and BSD `kern/subr_callout.c`:

```
Level 0: 256 slots × 1ms  = 256ms range  (fine-grained)
Level 1:  64 slots × 256ms = 16.4s range  (coarse-grained)
```

**Complexity:**
- `add_timer()` — O(1)
- `cancel_timer()` — O(1)
- `advance_clock()` — O(expired timers)

---

## API Reference

### `SigmaTimerWheel`

```rust
let mut wheel = SigmaTimerWheel::new(SigmaClockSource::Tsc, 1_000_000); // 1ms ticks

// One-shot timer at 50ms
let id = wheel.add_timer(50_000_000, my_callback, user_data, "my-timer");

// Periodic timer: every 10ms
let id = wheel.add_periodic_timer(10_000_000, 10_000_000, tick_cb, 0, "tick");

// Advance clock (call from interrupt handler)
let fired = wheel.advance_clock(1_000_000); // advance 1ms

// Cancel
wheel.cancel_timer(id);

// Read time
let now = wheel.get_monotonic_ns();
```

### `SigmaClockManager`

System-wide clock manager wrapping the timer wheel:

```rust
let mut mgr = SigmaClockManager::new();
mgr.set_wall_clock(unix_timestamp);
mgr.apply_ntp_offset(offset_ns);
mgr.tick(); // advances by tick_ns
```

---

## Comparison: Linux vs BSD vs SigmaOS

| Feature | Linux hrtimer | BSD callout | SigmaOS |
|---------|-------------|------------|---------|
| Resolution | nanosecond | tick-based | nanosecond |
| Wheel levels | 5 (LVL0..4) | 1 | 2 |
| One-shot | Yes | Yes | Yes |
| Periodic | Via restart | No (manual) | Yes |
| Cancellation | O(1) | O(1) | O(1) |
| no_std | No | No | **Yes** |
| NTP integration | Yes (CLOCK_ADJTIME) | adjtime() | Yes |

---

## NTP Clock Adjustment

```rust
// Adjust wall clock forward by 500 nanoseconds (NTP slew)
mgr.apply_ntp_offset(500);
```

---

## Implementation Notes

- All times are in absolute nanoseconds on the monotonic clock
- Periodic timers automatically re-arm without system call overhead
- Timer IDs are monotonically increasing 64-bit integers
- Statistics track: added, fired, cancelled, ticks
