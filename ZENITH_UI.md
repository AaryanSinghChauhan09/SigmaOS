# Zenith Dashboard & Telemetry UI

SigmaOS ships with the **Zenith UI Dashboard**—a production-grade, hardware-accelerated orchestration interface for the Sovereign Lattice.

## Hardware-Accelerated Compositing

Zenith utilizes the **Morphic Layer Composition (MLC)** algorithm, natively implemented in the `SovereignZenithUI` shard. By operating directly at Ring-0 with framebuffer manipulation, Zenith delivers high-fidelity glassmorphism, depth-shadowing, and ultra-smooth transitions without the overhead of external windowing systems (X11/Wayland).

## Industrial CSS Architecture

As of **Phase 45 Stabilization**, Zenith has migrated to a **Strict Utility-Class Architecture**.

* **Zero Inline Styles**: 100% of the UI styling is now encapsulated within `zenith_desktop.css`.

* **Modular Shards**: Components like the `market-list`, `app-shard-list`, and `kernel-console-shard` utilize optimized CSS utility classes (`.hidden-window`, `.full-width`, `.accent-bg`) for maximum rendering efficiency and cross-browser consistency.

## System Telemetry

The `SovereignTelemetryUI` exposes deep kernel insights with sub-millisecond latency. It correlates data from:

1. **SovereignNetStack**: Real-time throughput and packet-level entropy.

2. **SovereignMonitor**: Multi-die workload balancing and shard migration telemetry.

3. **SovereignDiag**: Silicon-direct fault localization and machine-state forensics.

## Spatial Shard Snapping (DSS)

The `SovereignSnapEngine` (located in `kernel/core/misc_utils/`) provides industrial-grade multi-window spatial organization. By utilizing a **2D Spatial Lattice**, it allows shards to "snap" into optimized Golden Ratio layouts, maximizing information density for high-concurrency monitoring and development tasks.

## Personalization & Accessibility

Integrated via `SovereignPersonalization`, Zenith supports:

* **Adaptive Contrast**: Auto-scaling based on ambient telemetry.
* **Predictive Layouts**: NPWO-driven interface adjustment based on workload history.

