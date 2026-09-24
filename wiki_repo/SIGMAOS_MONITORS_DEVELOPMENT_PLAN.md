# 🖥️ SigmaOS Display, Hardware, Process & System Monitors (`monitor`) Strategic Development Plan

## Executive Summary & Design Vision

System monitors in modern operating systems serve a dual role: managing **physical display monitors** (multi-monitor layout, EDID parsing, HDR metadata, refresh rate, DPMS power management) and providing real-time **hardware, process, and system performance monitoring** (CPU per-core load, GPU VRAM, NVMe disk I/O, network bandwidth, thermal sensors, and eBPF event tracing).

Drawing inspiration from Linux Wayland compositors (Hyprland, Sway, KWin), BSD hardware monitoring tools (`sysctl hw.sensors`, `bsd-sensors`), and modern TUI system monitors (`btop`, `htop`, `nvtop`), the **SigmaOS Monitor Infrastructure** provides a unified, zero-dependency, real-time monitoring and display configuration architecture in Safe Rust.

---

## 1. Multi-Distro & Multi-OS Monitor Inspirations

### 1.1 Linux Wayland DRM/KMS Display Monitors (Hyprland, Sway, KWin, KMS/DRM)
- **Inspirations**:
  - **Dynamic Multi-Monitor Topology**: Hotplug detection (`drmModeGetResources`), EDID resolution parsing, custom refresh rates (144Hz/240Hz), VRR/FreeSync adaptive sync, and Fractional Scaling (1.25x/1.5x/2.0x).
  - **DPMS & Power Management**: DPMS screen sleep (`On`, `Standby`, `Suspend`, `Off`) and ambient brightness sensors.
- **SigmaOS Integration**: `UniversalDrmKmsDisplayEngine` in `src/driver/` and `OmarchyHyprlandCompositorConfigEngine` in `src/distro/omarchy.rs`.

### 1.2 BSD & Linux Hardware Thermal & Power Rail Monitors (`sysctl`, `HWBusters`)
- **Inspirations**:
  - **FreeBSD `sysctl hw.sensors`**: Unified sensor tree reporting CPU core temperatures, fan RPM speeds, battery charge state, and power rail voltage.
  - **HW Busters & PCWorld Telemetry**: GPU VRM ripple current detection, ATX 3.1 power rail guard, and battery charging thresholds (80% battery saver mode).
- **SigmaOS Integration**: `HwbustersPowerTelemetryEngine` and `PcWorldBatteryHealthControllerEngine` in `src/tools/tech_media_extended_suite.rs`.

### 1.3 `btop`, `htop` & `nvtop` Real-Time Process & Resource Monitors
- **Inspirations**:
  - **TUI Process Graphs**: Real-time per-core CPU usage charts, memory allocation break-down (Active/Inactive/Cached/Buffer), disk read/write bandwidth, and process tree hierarchy.
  - **GPU Acceleration Monitoring**: VRAM usage, compute engine utilization, tensor core activity, and fan duty cycle monitoring.
- **SigmaOS Integration**: `HtopProcessMonitorEngine` in `src/tools/open_source_tools_engine.rs` and `TechPowerUpGpuProfilerEngine`.

### 1.4 eBPF & Kernel Performance Event Monitors (`bpftrace`, `sysinternals`)
- **Inspirations**:
  - **eBPF Ring Buffer Telemetry**: Lock-free kernel event streaming (`BPF_MAP_TYPE_RINGBUF`) capturing process spawn, disk I/O latency, network packet drops, and syscall latency histograms.
- **SigmaOS Integration**: `BpfRingBufferEngine` in `src/kernel/ebpf.rs` and `SysinternalsProcmon` in `src/tools/`.

---

## 2. Core Architectural Subsystems

```text
┌───────────────────────────────────────────────────────────────────────────┐
│                     Zenith Control Center & Desktop UI                    │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│               Display Monitor Controller (Wayland DRM/KMS)                │
│     - DRM Hotplug Detection & EDID Monitor Name/Resolution Parser         │
│     - Multi-Monitor Arrangement Matrix (Primary, Extend, Mirror)          │
│     - Refresh Rate (Hz), VRR/FreeSync Adaptive Sync, DPMS Sleep Modes     │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│             Hardware Sensor & Thermal Telemetry Engine                    │
│     - CPU Core Temps (°C), Fan RPM, Battery Charge %, Power Rail Volts    │
│     - GPU VRAM Utilization, Compute Core %, VRM Voltage & Power Guard     │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│             System & Process Monitor Engine (`btop` / `htop`)             │
│     - Per-Core CPU Load %, Memory Split (Used/Available/ZFS ARC)          │
│     - NVMe / SATA Disk Read/Write Throughput (MB/s)                       │
│     - Process Tree (PID, User, CPU%, Mem%, Priority, Signal Control)      │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                 eBPF & Kernel Event Monitor Ring Buffer                   │
│     - High-Frequency Kernel-to-Userland Event Telemetry                   │
└───────────────────────────────────────────────────────────────────────────┘
```

### 2.1 Display Monitor Hotplug & Arrangement
- Hotplug event resolution within **< 100 milliseconds**.
- Multi-monitor layout coordinates managed via Zenith compositor protocol (`/etc/sigma/monitors.conf`).

### 2.2 Telemetry Polling Rate & Overhead
- Default telemetry refresh interval: **1000ms** (configurable down to **100ms** for high-frequency profiling).
- Monitoring overhead constrained to **< 0.5% CPU utilization**.

---

## 3. Phased Development Roadmap

### Phase 1: DRM/KMS Display Monitor Engine (Q4 2026)
- Stabilize DRM/KMS hotplug detection and EDID parser in `src/driver/`.
- Implement multi-monitor relative positional layouts and DPMS power modes in Zenith Wayland compositor.
- Add `/etc/sigma/monitors.conf` persistence.

### Phase 2: Hardware Sensor & Telemetry Integration (Q1 2027)
- Unify Linux sysfs thermal/hwmon and FreeBSD `sysctl hw.sensors` data sources in `src/hardware/`.
- Deploy ATX 3.1 power rail guard and GPU VRAM/compute telemetry monitors.
- Implement battery health charging threshold rules (e.g., max 80% charge threshold).

### Phase 3: Real-Time TUI & GUI System Monitor (Q2 2027)
- Enhance `HtopProcessMonitorEngine` with tree process view, CPU affinity masks, and process kill signals.
- Render real-time ASCII/ANSI load graphs in `sigma-sh` and native Zenith GTK/Wayland GUI widgets.
- Add NVMe and SATA disk I/O throughput counters.

### Phase 4: eBPF Profiling & Benchmarks (Q3 2027+)
- Connect `BpfRingBufferEngine` with system monitor for live syscall latency histograms.
- Publish hardware and display monitor benchmark suite in `scripts/tech_media_benchmark_suite.sh`.
- Deploy automated thermal throttling alerting daemon.

---

## 4. Verification & Testing Standards

All monitor components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/driver/` (DRM/KMS display monitor hotplug & EDID parsing)
- `src/tools/tech_media_extended_suite.rs` (power rail telemetry & battery health monitors)
- `src/tools/open_source_tools_engine.rs` (`HtopProcessMonitorEngine`)
- `src/kernel/ebpf.rs` (`BpfRingBufferEngine` performance event tracing)
