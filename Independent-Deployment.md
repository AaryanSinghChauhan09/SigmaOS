# Σ SIGMAOS: Independent Deployment (release/standalone)

The `release/standalone` branch is the pure, bare-metal expression of SigmaOS, optimized for independent hardware and resource-constrained environments.

## ⚙️ Performance Focus

- **Rapid Ignition**: Stripping unnecessary modules to achieve the fastest possible boot times.
- **Embedded Optimization**: Benchmarking real-time performance for IoT, edge, and aerospace devices.
- **Deterministic Scheduling**: Using the `SovereignAISched` in Hard-RT mode for absolute latency guarantees.

## 🧩 Shard Autonomy

- **Resource Constraints**: Validating shard performance on low-memory (MB range) hardware.
- **Hot-Swappable Shards**: Ensuring modules can be replaced under high load without system instability.

## 🧪 Testing & Validation

- **Bare-Metal Audits**: Automated testing on physical x86, ARM, and RISC-V hardware.
- **Simulation Framework**: Using the `simulation/` directory to stress-test algorithmic behavior at scale.

*"Standalone is the ultimate industrial fact."*
