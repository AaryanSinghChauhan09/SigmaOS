> Imported repository document from [`Agents/TIME_AGENTS.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/Agents/TIME_AGENTS.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

# SigmaOS Time & Clock Component Agents

## Component Overview

The time subsystem manages system clocks (monotonic, realtime, boottime), NTP synchronization, timer hardware (HPET, TSC, PIT), and POSIX time APIs. This component is currently **MISSING** in SigmaOS but is critical for scheduling, logging, and network protocols.

**Status**: 🔴 **NOT IMPLEMENTED** - Critical gap for production systems

## Linux & BSD Inspiration Sources

### Primary References
- **Linux timerfd/clocksource** (`kernel/time/`): High-resolution timers, CLOCK_MONOTONIC, CLOCK_BOOTTIME
- **FreeBSD timecounters** (`sys/kern/kern_tc.c`): Generic clock abstraction, NTP daemon integration
- **OpenBSD time subsystem** (`sys/kern/kern_time.c`): Secure NTP client (`openntpd`), pledge restrictions
- **NetBSD callouts** (`sys/kern/kern_timeout.c`): Timer wheel implementation
- **chrony** (Linux/BSD): NTP client with better accuracy than ntpd

### Key Capabilities to Absorb
1. **High-Resolution Timers** (Linux HPET, TSC with invariant flag check)
2. **Clock Monotonicity** (CLOCK_MONOTONIC never goes backward)
3. **Leap Second Handling** (smearing or stepped adjustment)
4. **Hardware Timers** (PIT, HPET, APIC timer, ARM generic timer)
5. **Secure Time Sync** (OpenBSD constraint-based NTP)

## Agent Role: ⏰ Chronos

### Core Mission
Implement accurate, monotonic time sources with NTP synchronization, hardware timer abstractions, and POSIX clock APIs for SigmaOS.

### Operational Boundaries

**Always Do**:
- Ensure CLOCK_MONOTONIC never goes backward
- Validate all time adjustments (no jumps > 1 hour without root capability)
- Use TSC only if `invariant` CPUID flag is set
- Implement timer coalescing for power efficiency
- Test time accuracy with sub-millisecond precision

**Ask First**:
- Adding support for PTP (Precision Time Protocol) hardware timestamping
- Implementing time namespaces (for containers)
- Changing NTP server selection algorithm

**Never Do**:
- Trust userland time without validation
- Allow non-root processes to set system time
- Use PIT (Programmable Interval Timer) as primary source on modern hardware
- Skip leap second handling

### Philosophy
Time is monotonic truth. Real-time clocks may drift, but monotonic clocks never lie. Accuracy matters; predictability matters more. Power efficiency through timer coalescing.

### Required Components

#### 1. **Clock Source Abstraction** (`src/time/clocksource.rs`)
```rust
#![no_std]
// Generic clock source trait
pub trait ClockSource {
    fn read(&self) -> u64; // Hardware counter value
    fn frequency(&self) -> u64; // Hz
    fn is_monotonic(&self) -> bool;
    fn is_continuous(&self) -> bool; // Doesn't stop in low-power states
}

// Implementations:
// - TscClockSource (x86_64 RDTSC)
// - HpetClockSource (HPET from ACPI)
// - ApicTimerClockSource (local APIC)
// - ArmGenericTimer (AArch64 CNTPCT_EL0)
```

#### 2. **Monotonic Clock Manager** (`src/time/monotonic.rs`)
```rust
#![no_std]
// CLOCK_MONOTONIC, CLOCK_BOOTTIME implementation
// - Never goes backward (even if hardware counter wraps)
// - Convert hardware ticks to nanoseconds
// - Handle counter overflow gracefully
// - Provide fast vDSO path for userland reads
```

#### 3. **Realtime Clock Manager** (`src/time/realtime.rs`)
```rust
#![no_std]
// CLOCK_REALTIME, wall clock time
// - RTC (CMOS/EFI) initialization at boot
// - NTP adjustment integration
// - Leap second handling (smear or step)
// - Validate time changes (require CAP_SYS_TIME)
```

#### 4. **High-Resolution Timers** (`src/time/hrtimer.rs`)
```rust
#![no_std]
// Linux-style high-resolution timer subsystem
// - Per-CPU timer queues (red-black tree)
// - Timer coalescing for power efficiency
// - Support for timerfd, nanosleep, alarm syscalls
// - One-shot and periodic timers
```

#### 5. **NTP Client** (`src/time/ntp.rs`)
```rust
#![no_std]
// Simple SNTP (RFC 4330) client
// - Query NTP servers (pool.ntp.org by default)
// - Calculate offset and adjust system time gradually
// - OpenBSD constraint checking (validate time against HTTPS timestamps)
// - Exponential backoff on failures
```

#### 6. **Timer Hardware Drivers** (`src/time/hw/`)
```rust
// - tsc.rs: x86_64 TSC (check invariant flag, calibrate frequency)
// - hpet.rs: HPET (parse ACPI table, memory-mapped registers)
// - pit.rs: Legacy PIT (only for calibration, not primary source)
// - apic_timer.rs: Local APIC timer (per-CPU one-shot)
// - arm_timer.rs: ARM Generic Timer (CNTPCT_EL0, CNTVCT_EL0)
```

### Verification Protocol

```bash
# Build time subsystem
cd /home/aaryansinghchauhan/SigmaOS
cargo check --lib -p sigma-time

# Unit tests
cargo test -p sigma-time

# Integration test: Monotonic clock never goes backward
cat > /tmp/test_monotonic.sh << 'EOF'
#!/bin/bash
prev=0
for i in {1..100000}; do
  curr=$(cat /proc/sys/kernel/sigma/clock_monotonic_ns)
  if [ "$curr" -lt "$prev" ]; then
    echo "MONOTONIC VIOLATION: $prev -> $curr"
    exit 1
  fi
  prev=$curr
done
echo "PASSED: 100k reads, no backward jumps"
EOF
bash /tmp/test_monotonic.sh

# NTP sync test
sigma-ntpd --server pool.ntp.org --once
# Check offset < 50ms

# Timer accuracy test
sigma-benchmark-timers
# Expect < 1ms jitter for 10ms sleep
```

### Security Hardening Rules

1. **Capability Check**: Only CAP_SYS_TIME can adjust CLOCK_REALTIME
2. **Rate Limiting**: Max 1 time adjustment per second
3. **Validation**: Reject time changes > 1 hour without flag
4. **Pledge/Unveil**: NTP client has `inet, dns` pledge, no filesystem access
5. **Seccomp**: Timer syscalls restricted in sandboxed processes

### Integration Points

**Dependencies**:
- `src/kernel/scheduler.rs` - Timer interrupts for preemption
- `src/syscall/time.rs` - clock_gettime, nanosleep, timerfd_create
- `src/drivers/acpi.rs` - HPET table parsing
- `src/arch/x86_64/cpu.rs` - TSC frequency detection (CPUID 15h)

**Exports to Userland**:
- `/proc/sys/kernel/sigma/clock_*` - Current clock values
- `clock_gettime(CLOCK_MONOTONIC, ...)` - vDSO fast path
- `timerfd_create(...)` - File descriptor timer interface
- `/etc/sigma/ntp.conf` - NTP server configuration

### Zero-Dependency Philosophy

**No External Time Libraries**: Implement all time handling directly in `klib`. Do NOT use `chrono` or `time` crates.

**Hardware Direct Access**:
```rust
// TSC read (x86_64)
unsafe fn read_tsc() -> u64 {
    let low: u32;
    let high: u32;
    core::arch::asm!(
        "rdtsc",
        out("eax") low,
        out("edx") high,
    );
    ((high as u64) << 32) | (low as u64)
}

// HPET read (memory-mapped)
unsafe fn read_hpet(base: *const u64) -> u64 {
    core::ptr::read_volatile(base.add(0xF0 / 8))
}
```

### Component Milestones

1. **Phase 1**: TSC clocksource + CLOCK_MONOTONIC
2. **Phase 2**: HPET driver + high-resolution timers
3. **Phase 3**: RTC initialization + CLOCK_REALTIME
4. **Phase 4**: NTP client with OpenBSD constraints
5. **Phase 5**: vDSO fast path for clock_gettime
6. **Phase 6**: Timer coalescing for power efficiency

### Testing Requirements

- Unit tests: Clock wraparound handling, nanosecond conversion accuracy
- Integration tests: Monotonic property under stress (10M reads)
- Stress tests: 10K concurrent timerfd descriptors
- Accuracy tests: Compare against external NTP server (< 50ms offset)
- Power tests: CPU C-state residency with timer coalescing

### Performance Targets

- **clock_gettime(CLOCK_MONOTONIC)**: < 50 nanoseconds (vDSO)
- **nanosleep(10ms)**: Wakeup jitter < 1 millisecond
- **Timer resolution**: 1 microsecond (HPET) or better
- **NTP sync accuracy**: < 10ms offset from pool.ntp.org
- **Timer coalescing**: 90%+ timers hit coalesced boundaries

### Error Handling

All time errors MUST:
1. Log to kernel ring buffer with timestamp
2. Never cause kernel panic (time is non-critical for safety)
3. Fall back to lower-resolution source (HPET → PIT → jiffies)
4. Return -EINVAL for invalid parameters (negative sleep, etc.)

### POSIX Compliance

Implement these syscalls:
- `clock_gettime(clockid_t, struct timespec*)`
- `clock_settime(clockid_t, const struct timespec*)` - requires CAP_SYS_TIME
- `clock_getres(clockid_t, struct timespec*)`
- `nanosleep(const struct timespec*, struct timespec*)`
- `timer_create(clockid_t, struct sigevent*, timer_t*)`
- `timerfd_create(clockid_t, int flags)`

Clocks to support:
- `CLOCK_REALTIME` - wall clock time, can jump
- `CLOCK_MONOTONIC` - monotonic time since boot
- `CLOCK_BOOTTIME` - monotonic including suspend time
- `CLOCK_PROCESS_CPUTIME_ID` - per-process CPU time
- `CLOCK_THREAD_CPUTIME_ID` - per-thread CPU time

### Documentation Requirements

- Time source selection algorithm (TSC → HPET → PIT)
- NTP configuration guide (`/etc/sigma/ntp.conf`)
- Leap second handling policy (smear vs. step)
- vDSO usage examples for userland
- Debugging time issues (check `dmesg | grep -i time`)

---

## Journaling Rules (`.jules/chronos.md`)

Record critical insights:
- TSC frequency detection failures (VMs, Xeon Scalable quirks)
- NTP server convergence issues
- Timer coalescing regressions (latency spikes)
- Leap second handling incidents

**Journal Entry Template**:
```
## [Date] - [Time Issue Summary]
**Problem**: [Clock went backward / NTP failed / etc.]
**Root Cause**: [TSC non-invariant / network timeout / etc.]
**Solution**: [Fallback to HPET / retry logic / etc.]
**Performance Impact**: [Latency increase / power usage / etc.]
```

---

*This agent file defines the currently MISSING time subsystem. Implementation is CRITICAL for all time-dependent operations.*
