# SigmaOS Core System Design

## Overview
The Core System is the foundation of SigmaOS, providing kernel-level functionality, hardware abstraction, and system initialization. Taking inspiration from Arch Linux and Fedora, SigmaOS combines a Rolling/Stable hybrid release model with Fedora's strong hardware driver integration and Arch's minimal base philosophy.

```
          [SigmaOS Release Strategy]
                      │
            ┌─────────┴─────────┐
            ▼                   ▼
    [Arch Rolling Stream]  [Fedora Stable Stream]
    (Developers/Cutting)    (Enterprise/Stable)
            │                   │
            └─────────┬─────────┘
                      ▼
            [Unified Base System]
```

## System Properties & Models
1. **Rolling & Stable Hybrid Model**: Developer profiles run on a rolling-release tree (`sigma-rolling`), receiving package upgrades immediately. Production and enterprise instances run on a stable-cadence tree (`sigma-stable`), receiving frozen, hardened packages validated on 6-month cycles.
2. **Minimal Base System**: Follows Arch's minimal core footprint philosophy. The default base system includes only the microkernel, the `sigmad` init system, `sigpkg`, and basic terminal utils. Everything else is structured as standalone packages.
3. **Hardware Compatibility Matrix**: Upstreams drivers directly and publishes an Hardware Compatibility List (HCL) generated automatically from user telemetry.
4. **Fedora Infrastructure Driver Integration**: Absorbs driver support structures and build pipeline orchestration patterns inspired by `https://github.com/fedora-infra`. This includes implementing custom analogues of the Koji build system for automated package compilation, Bodhi for gating updates based on hardware testing feedback loops, and MirrorManager for high-speed local driver package delivery.

## System Configuration Specification
System release streams are configured in `/etc/sigma/core.conf`:
```toml
[system]
stream = "stable" # stable or rolling
version = "2026.07"
minimal_base = true

[hcl]
telemetry_enabled = true
publish_hcl_status = true
```

## Technical Implementation
The system bootstrap loads the environment and dynamically maps the release repositories based on stream configuration.

```rust
// kernel/init/sigma_init.rs
pub fn determine_system_profile() -> SystemProfile {
    let config = load_system_config("/etc/sigma/core.conf");
    match config.get("system", "stream") {
        Some("rolling") => SystemProfile::Rolling,
        _ => SystemProfile::Stable,
    }
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Base bootloader optimization and rolling release repository hosting.
- **Phase 2 (Months 3-6)**: Hardware detection suite and publication of the automated HCL website.
- **Phase 3 (Months 6-9)**: Containerized testing farm to validate stable release package trees.
- **Phase 4 (Months 9-12)**: Upstream driver submission tool and automated compatibility warning systems.
