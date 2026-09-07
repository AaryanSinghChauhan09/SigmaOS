# AI Agent Development Instructions for Automation Subsystem (`src/automation/`)

This directory contains system-level automation routines, workflow engine orchestration, AI task optimization, macro recording/playback, hotkey trigger mapping, and scheduled task execution for SigmaOS.

## Subsystem Architecture & Directives

1. **System Automation Orchestration (`orchestrator.rs` & `engine.rs`)**
   - Execute workflows combining system maintenance, resource cleanup, backup triggers, and custom event handlers.
   - Enforce rate-limiting and maximum iteration depth on automated loops to prevent runaway background execution.

2. **AI Task Optimizer (`ai_optimizer.rs`)**
   - Evaluate system metrics (CPU load, memory working set, power profile) to automatically tune routine priorities.
   - Fall back to deterministic fallback rules when AI optimization confidence levels are below threshold (`< 0.85`).

3. **Macro Recording & Hotkey Handling (`macro.rs` & `hotkey.rs`)**
   - Ensure macro event streams (keyboard/mouse sequences) undergo permission checks before replay.
   - Replay input sequences through sandboxed input channels to prevent privileged command execution from unprivileged macro files.

4. **Script Execution & Task Scheduling (`script.rs` & `scheduler.rs`)**
   - Support cron-like schedule specifications and event-driven triggers (`OnBoot`, `OnUserLogin`, `OnLowBattery`, `OnNetworkConnect`).
   - Isolate script execution within capability-constrained sandboxes.

5. **Code Hygiene & Verification**
   - Maintain clean modular exports in `mod.rs`.
   - Verify changes with `cargo check --lib` before committing.
