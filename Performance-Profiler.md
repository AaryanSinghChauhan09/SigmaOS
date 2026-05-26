# Performance Profiler (`sigma-prof`)

The `sigma-prof` utility acts as SigmaOS's dynamic workload auto-tuner. Inspired by Clear Linux's aggressive architectural optimizations, it shifts kernel and hardware parameters dynamically to ensure peak execution efficiency for specialized tasks.

## Design Philosophy
We decouple generic defaults from intensive tasks. When required, operators can strictly manually invoke `sigma-prof tune` to adjust state, preserving the overall purity of the Declarative Config Utility unless explicit performance augmentation is needed.

## Subsystems Addressed
* **Hardware Performance Monitoring (PMU):** Tracks AVX-512 saturation, L1/L2 cache locality, and NPU context switching.
* **Kernel Auto-Tuning:** Interacts with memory schedulers and power C-states.

## Available Profiles
* `hpc`: High Performance Computing (Max frequency, NUMA locality).
* `ai`: AI/Tensor operations (AVX-512 prioritization, increased memory bandwidth).
* `embedded`: IoT/SBC usage (Aggressive C-state sleeping, strict thermal caps).

## Usage
```bash
# Analyze bottlenecks
sigma-prof analyze

# Apply AI profile
sigma-prof tune ai

# Reset to declarative baseline
sigma-prof reset
```
