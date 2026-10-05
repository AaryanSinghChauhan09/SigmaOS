# Power Management — AI Agent Guidelines

**Component Status**: Critical — Currently Missing
**Inspiration**: Linux PM (ACPI, cpufreq, cpuidle), FreeBSD powerd, OpenBSD apmd

## Overview

Power management is essential for modern operating systems, especially on laptops and mobile devices. It controls CPU frequency scaling, idle states, device suspend/resume, thermal management, and battery monitoring.

---

## Linux & BSD Inspiration

### Linux Power Management
- **cpufreq**: CPU frequency scaling with governors (performance, powersave, ondemand, schedutil)
- **cpuidle**: CPU idle state management (C-states)
- **ACPI**: Advanced Configuration and Power Interface for hardware control
- **Runtime PM**: Per-device power management
- **Suspend states**: S3 (suspend-to-RAM), S4 (hibernate), S5 (power off)
- **TLP**: Advanced power management daemon
- **PowerTOP**: Power consumption analysis tool

### FreeBSD powerd
- **Adaptive frequency scaling**: CPU frequency based on load
- **ACPI integration**: Battery status, AC adapter detection
- **CPU governors**: Similar to Linux cpufreq
- **Device power states**: D0 (full power) to D3 (off)

### OpenBSD apmd
- **Automatic Performance Mode**: CPU frequency scaling
- **Battery monitoring**: Low battery warnings
- **Suspend/resume**: Sleep and wake management
- **Sensor framework**: Temperature and voltage monitoring

---

## Required Features for SigmaOS

### Phase 1: CPU Frequency Scaling
```rust
// src/power/cpufreq.rs

pub enum Governor {
    Performance,   // Always max frequency
    Powersave,     // Always min frequency  
    Ondemand,      // Scale based on load
    Conservative,  // Gradual scaling
    Schedutil,     // Scheduler-driven
}

pub struct CpuFreqDriver {
    pub min_freq: u64,      // kHz
    pub max_freq: u64,      // kHz
    pub current_freq: u64,  // kHz
    pub governor: Governor,
    pub available_freqs: Vec<u64>,
}

impl CpuFreqDriver {
    pub fn set_governor(&mut self, gov: Governor) { /* ... */ }
    pub fn set_frequency(&mut self, freq: u64) -> Result<(), PowerError> { /* ... */ }
    pub fn get_current_freq(&self) -> u64 { /* ... */ }
    pub fn scale_frequency(&mut self, load: f64) { /* ... */ }
}
```

### Phase 2: Idle State Management
```rust
// src/power/cpuidle.rs

pub struct IdleState {
    pub name: &'static str,
    pub latency_us: u32,      // Exit latency
    pub power_usage_mw: u32,  // Power consumption
    pub target_residency: u64, // Minimum time to be worthwhile
}

pub struct CpuIdleDriver {
    pub states: Vec<IdleState>,
    pub current_state: usize,
}

impl CpuIdleDriver {
    pub fn enter_idle(&mut self, expected_idle_time: u64) -> usize { /* ... */ }
    pub fn select_state(&self, idle_time: u64) -> usize { /* ... */ }
}
```

### Phase 3: Suspend/Resume
- S3 suspend-to-RAM implementation
- Device state save/restore
- Wake event handling
- Fast resume optimization

### Phase 4: Thermal Management
- Temperature monitoring
- Thermal throttling
- Fan control
- Critical temperature shutdown

---

## Implementation Priority

| Feature | Priority | Inspiration | Module |
|---------|----------|-------------|--------|
| CPU frequency scaling | P0 (Critical) | Linux cpufreq | `src/power/cpufreq.rs` |
| Idle state management | P1 (High) | Linux cpuidle | `src/power/cpuidle.rs` |
| Battery monitoring | P1 (High) | ACPI | `src/power/battery.rs` |
| Suspend/resume | P1 (High) | Linux PM | `src/power/suspend.rs` |
| Thermal management | P2 (Medium) | Linux thermal | `src/power/thermal.rs` |
| Device runtime PM | P2 (Medium) | Linux runtime PM | `src/power/device_pm.rs` |
| Power profiles | P2 (Medium) | power-profiles-daemon | `src/power/profiles.rs` |

---

## CPU Frequency Governors

### Performance
- Always run at maximum frequency
- Best for compute-intensive workloads
- Highest power consumption

### Powersave
- Always run at minimum frequency
- Maximize battery life
- Acceptable for idle or light workloads

### Ondemand (Default)
- Scale frequency based on CPU load
- Quick ramp-up to max frequency under load
- Gradual scale-down when idle

### Schedutil (Modern)
- Scheduler-driven frequency selection
- More accurate than ondemand
- Uses scheduler utilization metrics

---

## Development Guidelines

### Bolt ⚡ (Performance Agent)
- Minimize frequency switching overhead
- Use hardware P-states when available
- Optimize governor decision latency
- Fast wake from idle states

### Sentinel 🛡️ (Security Agent)
- Restrict power control to privileged processes
- Validate frequency bounds before hardware writes
- Protect against denial-of-service via rapid frequency changes
- Secure suspend/resume process

### Palette 🎨 (UX Agent)
- Clear battery status display
- Power profile selection UI
- Estimated battery time remaining
- Thermal warning notifications

---

## Power Profiles

```rust
pub enum PowerProfile {
    Performance,    // Max performance, no power saving
    Balanced,       // Balance performance and efficiency
    PowerSaver,     // Maximize battery life
}

impl PowerProfile {
    pub fn apply(&self, system: &mut PowerManager) {
        match self {
            Self::Performance => {
                system.cpu_freq.set_governor(Governor::Performance);
                system.disable_usb_autosuspend();
            }
            Self::Balanced => {
                system.cpu_freq.set_governor(Governor::Schedutil);
                system.enable_device_autosuspend();
            }
            Self::PowerSaver => {
                system.cpu_freq.set_governor(Governor::Powersave);
                system.aggressive_device_suspend();
            }
        }
    }
}
```

---

## ACPI Integration

### Required ACPI Tables
- **DSDT**: Differentiated System Description Table
- **FADT**: Fixed ACPI Description Table
- **MADT**: Multiple APIC Description Table  
- **SSDT**: Secondary System Description Table

### ACPI Events
- Power button press
- Sleep button press
- Lid close/open
- AC adapter plug/unplug
- Battery status change
- Thermal zone crossing

---

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_scaling() {
        let mut driver = CpuFreqDriver::new(800_000, 3_500_000);
        driver.set_governor(Governor::Ondemand);
        
        // High load should scale up
        driver.scale_frequency(0.9);
        assert!(driver.current_freq > 2_000_000);
        
        // Low load should scale down
        driver.scale_frequency(0.1);
        assert!(driver.current_freq < 1_500_000);
    }

    #[test]
    fn test_idle_state_selection() {
        let driver = CpuIdleDriver::new();
        
        // Short idle: shallow state
        let state = driver.select_state(100);
        assert_eq!(state, 0); // C1
        
        // Long idle: deep state
        let state = driver.select_state(10_000);
        assert!(state > 0); // C3 or deeper
    }

    #[test]
    fn test_power_profile() {
        let mut pm = PowerManager::new();
        
        PowerProfile::Performance.apply(&mut pm);
        assert_eq!(pm.cpu_freq.governor, Governor::Performance);
        
        PowerProfile::PowerSaver.apply(&mut pm);
        assert_eq!(pm.cpu_freq.governor, Governor::Powersave);
    }
}
```

---

## Integration Points

### With Kernel
- Timer interrupts for frequency decisions
- Scheduler load metrics for schedutil
- Interrupt handling for ACPI events
- Device driver suspend/resume hooks

### With Hardware
- MSR (Model-Specific Register) writes for frequency
- ACPI method invocation
- PCI power state transitions
- GPIO for device power control

### With Userspace
- `/sys/devices/system/cpu/cpu*/cpufreq/` interface
- Power profile daemon integration
- Battery status notifications
- Thermal event broadcasts

---

## File Locations

| Path | Purpose |
|------|---------|
| `src/power/mod.rs` | Main power management module |
| `src/power/cpufreq.rs` | CPU frequency scaling |
| `src/power/cpuidle.rs` | CPU idle state management |
| `src/power/battery.rs` | Battery monitoring |
| `src/power/suspend.rs` | System suspend/resume |
| `src/power/thermal.rs` | Thermal management |
| `src/power/acpi.rs` | ACPI interface |
| `/sys/power/` | Power management sysfs interface |

---

## References

- [Linux cpufreq Documentation](https://www.kernel.org/doc/html/latest/admin-guide/pm/cpufreq.html)
- [Linux cpuidle Documentation](https://www.kernel.org/doc/html/latest/admin-guide/pm/cpuidle.html)
- [ACPI Specification](https://uefi.org/specifications)
- [FreeBSD powerd](https://www.freebsd.org/cgi/man.cgi?powerd)
- [OpenBSD apmd](https://man.openbsd.org/apmd)

---

## Current Status

- ❌ No CPU frequency scaling
- ❌ No idle state management
- ❌ No battery monitoring
- ❌ No suspend/resume
- ❌ No thermal management
- ❌ No power profiles

**Next Steps**: Implement cpufreq driver with basic governor support (Phase 1)

---

*Last Updated: October 2026*
*Agent Guidelines for Power Management Development*
