# Userland Capabilities Upgrade

This document outlines the architectural enhancements made to SigmaOS's Userland and Zenith Desktop environments to support advanced Artificial Intelligence, Object-Oriented UIs, Cyber Security, and Data Science capabilities without relying on a standard library (`no_std`) or heap allocations.

## 1. Artificial Intelligence & Automation
SigmaOS now features a highly optimized, localized AI routing engine.

- **`sigma_llm_backend.rs`**: Implements a zero-allocation `AiTaskQueue` that schedules inference requests based on priority (`Background`, `Normal`, `Interactive`, `Critical`). It routes prompts to the hardware backend safely.
- **`local_llm.rs`**: Manages LLM session contexts. It retains conversation and command history in fixed-size buffers, enabling continuity for complex automated tasks.

## 2. Object-Oriented User Interfaces (Zenith Desktop)
To prove the viability of advanced UI patterns in a kernel-like environment, we implemented strict OOP paradigms using Rust Trait objects.

- **BSP Tiling Window Manager (`sigma_tiling_wm.rs`)**: A robust tiling algorithm utilizing a Binary Space Partitioning (BSP) tree. Windows implement the `WindowNode` and `Drawable` traits, supporting dynamic resizing and layout recalculations without dynamic memory allocation.
- **Declarative Profile Engine (`sigma_profile_engine.rs`)**: A configuration parser for `~/.sigma_profile` supporting Glassmorphism. It uses an OOP Enum Dispatch pattern (`ConfigToken`) to map themes, colors, blur radii, and high-contrast modes directly to the compositor.

## 3. Cyber Security & Isolation
Hardening the system beyond basic kernel permissions.

- **Security Center Daemon (`security_center.rs`)**: A background service that monitors the immutable kernel audit logs. It applies temporal decay heuristics to identify threats (e.g., rapid IPC auth failures or sandbox escape attempts) and can autonomously terminate malicious shards before they escalate.
- **Sovereign Sandbox (`sigma_sandbox.rs`)**: Provides the CLI API to wrap untrusted executables in a heavily restricted `SandboxConfig` (limiting memory, CPU time, network, and FS access) prior to execution.

## 4. Data Science & Algorithms
SigmaOS embeds fundamental computer science algorithms directly into the base OS tools, enabling autonomous telemetry analysis.

- **Machine Learning (`sigma_data.rs`)**: Implements an embedded, zero-allocation K-Means Clustering algorithm. It can classify system telemetry or user data streams autonomously.
- **Signal Processing (`sigma_data.rs`)**: Includes a Discrete Fourier Transform (DFT) implementation for localized audio/signal processing.
- **Benchmarking Suite (`sigma_bench.rs`)**: Provides rigorous performance testing tools that exercise the CPU, the physical memory allocator, and the new Data Science algorithms via the hardware Time Stamp Counter (TSC).

*Last Updated: July 2026*
