# Zenith Dashboard & Telemetry UI

SigmaOS ships with the **Zenith UI Dashboard** right out of the box.

## Hardware-Accelerated Compositing

Zenith uses a Morphic Layer Composition (MLC) algorithm written natively into `SovereignZenithUI`. Because it operates in Ring-0 and uses direct framebuffer manipulation, Zenith generates smooth glassmorphism blurs and shadow depth effects without any external X11 or Wayland dependencies.

## System Telemetry

Our `SovereignTelemetryUI` exposes deep kernel insights at 120fps. It aggregates:

1. `SovereignNetStack` TCP/IP throughput.
2. `SovereignNUMA` thread and memory migration latency.
3. Multi-die workload balancing.

### Personalization

Thanks to `SovereignPersonalization` and `SovereignAccessibility`, Zenith effortlessly supports:

- Smart contrast auto-scaling
- Predictive app layouts based on user history.
