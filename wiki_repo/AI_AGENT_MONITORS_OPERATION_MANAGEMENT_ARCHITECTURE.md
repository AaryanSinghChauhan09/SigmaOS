# AI Agent Monitors Operation Management Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                       AI Monitors & Telemetry Manager                           |
|  (DisplayMonitorAgent, HardwarePmcMonitor, HwbustersPowerSupplyMonitor)         |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                     DRM / KMS & Telemetry Dispatch Router                       |
|       (EDID Parser, Wayland Fractional Scaling, Hardware PMC, eBPF Tracer)      |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
| Wayland DRM/KMS Outputs|  | Hardware PMC Counters |   | Power & Thermal Probe |
| (MultiMonitorManager) |   | (HardwarePerfCounters)|   | (Battery & Voltage)   |
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                   Kernel Hardware Drivers & Display Connectors                  |
|          (KMS Framebuffer, PCIe PMC Registers, ACPI Power Controllers)          |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **Multi-Display DRM/KMS Subsystem**:
   - `MultiMonitorManager` probes DRM/KMS connectors (`HDMI-A-1`, `DP-1`, `eDP-1`).
   - Parses EDID timings and handles hotplug events (`BsdDevdHardwareEventDispatcher`).
   - Configures Wayland fractional scaling surfaces and syncs cursor themes (`CursorThemeEngine`).

2. **Hardware PMC & eBPF Telemetry Pipeline**:
   - `HardwarePerfCounters` reads hardware CPU performance counters via x86_64 MSRs (`IA32_PERF_FIXED_CTR`) or ARM PMU registers.
   - `EbpfSystemTracer` hooks kernel tracepoints to collect real-time I/O, paging, and scheduling metrics for CLI monitors (`btop`, `perf`, `systemd-cgtop`).

3. **Power & Thermal Management**:
   - `HwbustersPowerSupplyMonitor` tracks battery capacity, AC adapter status, and thermal zones (`/sys/class/thermal/thermal_zone*`).

4. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
