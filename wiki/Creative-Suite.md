# SigmaOS: Creative Suite Integration Roadmap

This document maps the support, latency tuning, and delivery of advanced multimedia workstations on SigmaOS.

## Target Repositories for Absorption

1. **`olive-editor/olive` & `shotcut/shotcut`**
   - **Goal:** Non-linear video editing.
   - **SigmaOS Integration:** Adapt Olive's node-based compositing interface and Shotcut's timeline model to utilize custom Zenith rendering pipes for hardware-accelerated viewport preview.

2. **`synfig/synfig`**
   - **Goal:** Vector 2D animation.
   - **SigmaOS Integration:** Package Synfig tools using `sigpkg` sandboxing, ensuring direct vector graphic exports match desktop performance.

3. **`lmms/lmms` & `ardour/ardour`**
   - **Goal:** Audio synthesis and digital audio workstation capabilities.
   - **SigmaOS Integration:** Optimize latency through our custom low-overhead sound scheduler.

### Last Updated: July 2026
