# SigmaOS CPU Power Management & Dynamic Frequency Scaling Specification

## 1. Overview

The CPU Power Management Subsystem dynamically adjusts CPU clock speeds, operating voltages (P-states), and idle power states (C-states) across asymmetric multi-core processors (e.g. Intel Alder/Raptor Lake P/E cores, ARM big.LITTLE / DynamIQ, Apple Silicon). It maximizes energy efficiency for portable devices while delivering instantaneous compute performance under heavy workloads.

## 2. Power States Architecture

```
+-----------------------------------------------------------------+
|                    CPU Operating Power States                   |
+-----------------------------------------------------------------+
| P-States (Performance / Voltage Scaling)                        |
|  - P0: Max Frequency & Voltage (e.g., 5.2 GHz @ 1.35V)          |
|  - P1..Pn: Intermediate Frequencies (e.g., 2.0 GHz @ 0.85V)     |
+-----------------------------------------------------------------+
| C-States (Core Power Reduction / Idle States)                   |
|  - C0: Active Execution                                         |
|  - C1 (Halt): Core clock gated, low exit latency (~1 μs)        |
|  - C6 (Deep Power Down): Core voltage dropped to 0V (~100 μs)   |
|  - C10 (Package Deep Idle): Entire CPU package power minimized  |
+-----------------------------------------------------------------+
```

## 3. Dynamic Scaling Governors

The CPU frequency scheduler (`schedutil` equivalent) evaluates per-core workload pressure in real time:

1. **`schedutil` (Default Governor)**: Integrated directly with the kernel EEVDF task scheduler. Uses actual task compute demand to immediately scale CPU frequency up or down without timer polling latency.
2. **`powersave`**: Pins CPU frequencies to energy-efficient baseline states for maximum battery longevity.
3. **`performance`**: Locks CPU cores at maximum turbo frequency for latency-critical audio processing and high-framerate gaming.
4. **`ondemand` / `conservative`**: Step-based frequency scaling governors for legacy hardware lacking direct scheduler integration.

## 4. Hardware Power Interfaces (Intel HWP / AMD CPPC)

- **Intel HWP (Hardware P-states / Speed Shift)**: Autonomously adjusts core voltage and clock frequencies directly within hardware based on performance hint registers (`IA32_HWP_REQUEST`).
- **AMD CPPC (Collaborative Processor Performance Control)**: Ranks cores by silicon efficiency and directs scheduler thread placement onto highest-performing cores.

## 5. Sysfs Control & Programmatic API

CPU power management interfaces are accessible via sysfs:

```bash
# View active governor
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor

# Set active governor
echo "schedutil" > /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor

# View current CPU frequency
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq
```
