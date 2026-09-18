# SigmaOS AI Agent Monitors Operation Management Guidelines

## 1. Overview
SigmaOS incorporates advanced hardware, performance, and multi-display monitor management frameworks supervised by AI agents (such as `DisplayMonitorAgent`, `HardwarePmcMonitor`, `BatteryPowerMonitor`, and `EbpfSystemTracer`). These guidelines define multi-monitor display configuration (`MultiMonitorManager`), DRM/KMS mode setting, EDID parsing, Wayland surface fractional scaling, hardware Performance Monitoring Counters (PMC), and system telemetry probes for AI agents in SigmaOS.

## 2. Core Monitors Operation Management Principles

### 2.1 DRM/KMS Mode Setting & Multi-Monitor Layouts
- **DRM/KMS Output Probe**: AI agents query Direct Rendering Manager (DRM) and Kernel Mode Setting (KMS) connectors (`/dev/dri/card0`) to detect active monitors, resolutions, and refresh rates (60Hz, 120Hz, 144Hz, 240Hz).
- **EDID Parsing**: Extended Display Identification Data (EDID) blobs are parsed to configure native display aspect ratios, color spaces (sRGB, DCI-P3, HDR10), and DPMS power saving modes.
- **Multi-Monitor Topology**: Agents manage screen arrangements (extended desktop, mirrored clone, vertical stack, or primary/secondary focus) via `MultiMonitorManager`.

### 2.2 Wayland Fractional Scaling & High-DPI Rendering
- **Mutter & Zenith Surface Scaling**: Agents interface with `Gnome46MutterEngine` and `Zenith` Wayland compositor to configure fractional scaling ratios (125%, 150%, 175%, 200%) per monitor output without UI blurriness.
- **XCursor Scale Sync**: Cursor themes (`CursorThemeEngine`) automatically rescale hotspot coordinates to match target display DPI settings.

### 2.3 Hardware PMC & System Telemetry Monitors
- **Performance Monitoring Counters (PMC)**: `HardwarePerfCounters` (`src/kernel/processor_management.rs`) monitors hardware CPU cycle counts, instructions retired, cache misses, and branch mispredictions.
- **Power Supply & Battery Monitoring**: `HwbustersPowerSupplyMonitor` tracks battery health (`/sys/class/power_supply/BAT0`), voltage rails, power consumption, and thermal throttling events.

### 2.4 eBPF System Tracing & Anomaly Telemetry
- **System Event Monitors**: `EbpfSystemTracer` tracks kernel I/O wait queues, page faults, process scheduling latencies, and network packet throughput in real time, streaming metrics to interactive shell commands (`btop`, `perf`, `aimon`, `systemd-cgtop`).

---
*Maintained by the SigmaOS Display, Hardware & Telemetry Steering Committee.*
