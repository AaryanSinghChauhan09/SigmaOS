# SigmaOS Phase F: Native Performance Architecture

## Overview

SigmaOS competes directly against browser-based operating systems and cloud-native desktop platforms. A fundamental advantage of SigmaOS is **native execution speed** — something browser OS architectures cannot replicate due to their JavaScript/V8 rendering pipeline overhead.

The **Performance Governor** (`kernel/power/sigma_perf_governor.cpp`) manages CPU frequency scaling, SIMD capability detection, high-resolution timing, and thermal protection.

---

## CPU Feature Detection (CPUID)

On boot, the HAL uses `CPUID` to detect hardware capabilities:

```
CPUID Leaf 0x07 (Extended Features)
  EBX bit 16 → AVX-512 Foundation (avx512f)
  EBX bit 30 → AVX-512 Byte/Word  (avx512bw)
  EBX bit  5 → AVX2

CPUID Leaf 0x16 (Frequency Info)
  EAX[15:0] → Base clock MHz
  EBX[15:0] → Boost clock MHz

CPUID Leaf 0x15 (TSC/Crystal ratio)
  → Used to calibrate RDTSC frequency in Hz
```

Feature flags are published to the kernel and made available to:
- Zenith Neural UI compositor (AVX-512 accelerated compositing)
- Crypto subsystem (AES-NI, SHA-NI acceleration)
- Network stack (SIMD-optimised checksum)

---

## Governor Profiles

| Profile | Description | Use Case |
|---------|-------------|----------|
| `POWERSAVE` | Minimum P-state, max C-states, BIAS=0xF | Battery-critical / passive cooling |
| `BALANCED` | HWP-managed, BIAS=0x8 | Default desktop workload |
| `PERFORMANCE` | Max sustained frequency, BIAS=0x0 | Compilation, rendering |
| `BURST` | Turbo + C0-only (no halt) | Ultra-low latency, HFT-class workloads |

Governor selection API:
```c
sigma_perf_set_governor(SIGMA_PERF_BURST);      // lock to turbo
sigma_perf_set_governor(SIGMA_PERF_BALANCED);   // auto-scale
```

---

## RDTSC High-Resolution Timer

SigmaOS uses the **Time Stamp Counter (TSC)** for nanosecond-precision timing. Unlike `clock_gettime()` in Linux (which has syscall overhead), SigmaOS's `sigma_perf_rdtsc_ns()` is a userland-callable vDSO equivalent:

```
ns = (RDTSC × 1,000,000,000) / tsc_freq_hz
```

TSC frequency is calibrated once at boot using CPUID leaf 0x15 (core crystal clock ratio). This eliminates the need for PIT/HPET for high-frequency timing.

---

## Thermal Protection

The performance governor integrates with the ACPI thermal zone:

```
CPU Temperature → ACPI → sigma_perf_thermal_event(temp_celsius)
    ≥ 95°C → Emergency: POWERSAVE mode
    ≥ 85°C → Warning:   BALANCED mode
    < 85°C → Normal:    Restore user-selected profile
```

---

## Competitive Advantage vs Browser OS

| Metric | SigmaOS | ChromeOS | WebOS |
|--------|---------|----------|-------|
| AVX-512 access | ✅ Direct | ❌ V8 SIMD only | ❌ None |
| Frequency control | ✅ MSR-level | ❌ OS-managed only | ❌ None |
| Timer precision | ✅ RDTSC (nanoseconds) | ⚠️ performance.now() (~1 ms) | ❌ |
| Boot-to-usable | ✅ < 2 s target | ⚠️ 8-15 s | ⚠️ 10-20 s |
| Background process | ✅ Native scheduling | ⚠️ Service workers only | ❌ |
| Thermal control | ✅ Kernel-direct | ⚠️ Chrome daemon | ❌ |

---

## IA32 MSR Reference

| MSR | Address | Purpose |
|-----|---------|---------|
| `IA32_PERF_CTL` | `0x199` | Target P-state ratio [15:8] |
| `IA32_ENERGY_PERF_BIAS` | `0x1B0` | Power/perf preference (0=perf, 15=power) |
| `MSR_PLATFORM_INFO` | `0xCE` | Max/min ratio discovery |

---

## Related Modules

- `kernel/power/sigma_perf_governor.cpp` — Governor implementation
- `kernel/power/sigma_power_manager.cpp` — ACPI S0-S5 and C-state management
- `hal/SovereignHAL.cpp` — Boot-time integration
- `zenith_desktop/neural/sigma_neural_ui.cpp` — AVX-512 compositor
