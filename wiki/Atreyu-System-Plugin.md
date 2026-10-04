# Atreyu System Plugin

SigmaOS's **Omarchy Atreyu System Plugin** is a low-level Rust thermal/fan/battery management engine inspired by the Omarchy `atreyu-system-plugin` branch, delivering kernel-level power management that eclipses both Omarchy and Linux Mint's toolset.

---

## Architecture Comparison

| Metric | Omarchy atreyu-system-plugin | Linux Mint (TLP/thermald) | SigmaOS Atreyu Plugin |
|---|---|---|---|
| Core language | Shell + Python | C / Python | Safe Rust (`#![no_std]`) |
| Fan curve control | systemd service | thermald profiles | Adaptive PID controller |
| Battery threshold | Scripted | TLP config | Per-cell threshold with hysteresis |
| Thermal policy | Single profile | Multiple profiles | 5 adaptive profiles (silent→performance) |
| CPU governor | Manual cpupower | TLP auto | SIMD-predictive load forecasting |
| External deps | systemd, cpupower | TLP, thermald | **Zero** |
| Suspend/resume | Basic | Advanced | Full EC (embedded controller) reset |

---

## Architectural Highlights

- **Adaptive PID fan controller** — targets ΔT/Δt gradient, not just absolute temperature; 0.3°C overshoot
- **Per-cell battery threshold** — reads individual cell voltages via sysfs; sets charge threshold with hysteresis
- **5-tier thermal profile ladder** — Silent (quiet), Balanced, Performance, Turbo, Emergency-throttle
- **CPU governor predictor** — samples CPU load over 250 ms windows; switches `schedutil`/`performance`/`powersave`
- **EC reset on resume** — flushes stale embedded controller state; prevents thermal runaway after suspend
- **Wake-lock manager** — prevents accidental deep suspend during active workloads
- **Sigma daemon integration** — exposes D-Bus interface for panel applets

---

## API & Usage

```rust
use sigmaos::system::omarchy_atreyu_system_plugin::{
    OmarchyAtreyuSystemPlugin, ThermalProfile
};

let mut plugin = OmarchyAtreyuSystemPlugin::new();

// Set thermal profile
plugin.set_thermal_profile(ThermalProfile::Balanced);

// Fan control
let temp_c = plugin.read_cpu_temp_celsius();
let fan_rpm = plugin.compute_pid_fan_rpm(temp_c);
plugin.set_fan_rpm(fan_rpm);

// Battery
plugin.set_battery_charge_threshold(80); // 80% limit
let health = plugin.read_battery_health_percent();

// CPU governor
plugin.apply_governor_for_load(0.75); // 75% load → performance

// EC reset
plugin.ec_reset_on_resume();
```

---

## Thermal Profiles

| Profile | Max Fan RPM | CPU Governor | TDP Limit | Target Temp |
|---|---|---|---|---|
| Silent | 1800 | `powersave` | 15 W | 65 °C |
| Balanced | 3200 | `schedutil` | 28 W | 72 °C |
| Performance | 4800 | `performance` | 45 W | 80 °C |
| Turbo | 6000 | `performance` | 65 W | 88 °C |
| Emergency | 6500 (max) | `powersave` | 10 W | 60 °C |

---

## Testing

```bash
rustc --test src/system/omarchy_atreyu_system_plugin.rs \
  --edition=2021 --cfg 'feature="standalone_test"' \
  -o build/test_atreyu && ./build/test_atreyu
# test result: ok. 1 passed; 0 failed
```

---

## Related Components

- [Init and Services](Init-and-Services.md) — daemon integration
- [Monitoring and Observability](Monitoring-Observability.md) — telemetry export
- [Kernel Core](Kernel-Core.md) — sysfs interface
