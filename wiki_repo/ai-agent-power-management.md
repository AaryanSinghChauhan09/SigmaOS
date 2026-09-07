# AI Agent Power Management & Thermal Governance in SigmaOS

## Overview

SigmaOS power management (`src/power/`, `src/power/governor.rs`, `src/power/sovereign_power.rs`) dynamically regulates power consumption, thermal limits, and CPU frequency governor states.

AI agents (such as Herdr agentic tasks, LLM inferences, and background automation loops) must adhere to power-aware scheduling policies to prevent thermal throttling and maximize battery longevity on mobile and edge devices.

---

## Power Profiles & Governor States

SigmaOS supports 4 power profiles that automatically govern AI agent execution:

| Power Profile | Governor Mode | AI Agent Policy | Max Frequency Cap |
|---------------|---------------|-----------------|-------------------|
| **Performance** | `performance` | Full parallel multi-threading & GPU acceleration | 100% Boost |
| **Balanced** | `schedutil` | Adaptive frequency scaling based on task deadline | 85% Base |
| **Power Saver** | `powersave` | Restrict background agents to single-thread execution | 50% Cap |
| **Extreme Saver** | `conservative` | Defer non-critical background AI tasks until AC power connected | 30% Cap |

---

## Adaptive Thermal Throttling Protocol

AI agents monitoring thermal sensors via `NetBsdSysmonPowerPwmEngine` must adjust workload intensity based on CPU/GPU junction temperatures:

```rust
use sigmaos::power::{PowerGovernor, ThermalPolicy};

let mut governor = PowerGovernor::new();

// Check CPU temperature before spawning compute-heavy agent task
if governor.cpu_temperature_celsius() > 80.0 {
    // Throttle subagent thread pool count to 1
    agent_task.set_max_concurrency(1);
    agent_task.set_priority(ThreadPriority::Low);
}
```

---

## Battery Level Event Callbacks

AI agents register power state change callbacks via `AutomationRoutineController`:

```rust
// Auto-suspend background LLM indexer when battery drops below 15%
routine_controller.add_routine(
    "suspend_ai_indexer",
    RoutineTrigger::PowerStateChange,
    "battery_low_15",
    "pause_agent_indexer"
);
```

---

## Directives for AI Agents on Battery Power

1. **Avoid Idle Spin Loops**: Agents must use `futex_wait()` or async channel receivers rather than busy-polling loops.
2. **Batch Model Inferences**: Group multiple user requests into batched SIMD/NPU execution runs to reduce SoC wakeups.
3. **Respect System Sleep Inhibitors**: Agents requesting system stay-awake locks must release `InhibitorLock` immediately upon task completion.
