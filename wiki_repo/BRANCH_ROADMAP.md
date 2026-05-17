# SigmaOS Branch-Wise Roadmap

This branch-wise roadmap ensures SigmaOS closes gaps with Linux distros while leveraging its USP: shard autonomy, post-quantum security, and AI-driven telemetry.

## 🌐 Main Branch (main)

### Issues

- Missing networking and storage subsystems.

- Limited driver support (GPU, Wi-Fi, peripherals).

### Fixes

- [ ] Implement a modular TCP/IP stack.

- [ ] Add sovereign file system support.

- [ ] Expand hardware driver coverage.

## ⚙️ Performance Branch

### Issues

- O(1) primitives not fully benchmarked under mixed workloads.

- Concurrency stress tests incomplete.

### Fixes

- [ ] Integrate automated latency/throughput benchmarks into CI.

- [ ] Add regression tests for memory fragmentation and starvation.

- [ ] Publish profiling reports before merging.

## 🔒 Security Branch

### Issues

- PQC modules (Dilithium-5, Kyber-1024) need side-channel resistance validation.

- Amnesic persistence not verified across SSD/NVMe.

### Fixes

- [ ] Add fuzzing for crypto modules.

- [ ] Expand S-ARMOR access control granularity (per-process, per-device).

- [ ] Audit logging for shard violations.

## 📚 Documentation Branch

### Issues

- Wiki not fully aligned with repo milestones.

- Missing subsystem diagrams and glossary.

### Fixes

- [ ] Migrate completed .md files into Wiki, then delete them.

- [ ] Add diagrams for shard autonomy, lattice flexibility, and CI/CD flow.

- [ ] Expand contributor guidelines and templates.

## 🧪 CI/CD Branch

### Issues

- No automated QEMU boot tests yet.

- Cross-architecture builds not validated.

### Fixes

- [ ] Add boot log validation in CI.

- [ ] Ensure reproducibility across x86, ARM, RISC-V.

- [ ] Integrate fuzzing and stress testing pipelines.

## 🚀 Editions Branch (S-Browser, S-App, S-Dual, S-Standalone)

### Issues

- Installer UX is basic (partitioning, language selection missing).

- No live USB boot option.

### Fixes

- [ ] Improve installer with enterprise-tier UX.

- [ ] Add live USB boot environments for testing.

- [ ] Harden dual-boot compatibility checks.

## 🧩 Universal OS Format Branch

### Issues

- Profiles (monolithic, hybrid, RTOS, cloud) not fully validated.

### Fixes

- [ ] Add automated compatibility tests for each profile.

- [ ] Improve embedded/RTOS documentation.

## 🤖 AI Telemetry Branch

### Issues

- AI profiling hooks not yet integrated.

### Fixes

- [ ] Add anomaly detection for shard performance.

- [ ] Implement predictive failure analysis.

- [ ] Prototype adaptive scheduling based on workload patterns.

## 🔑 Priority Fixes Across All Branches

- [ ] Networking + storage shards (critical for usability).

- [ ] Driver expansion (GPU, Wi-Fi, peripherals).

- [ ] Package manager + repo system (to match apt/dnf/pacman).

- [ ] Installer polish + live USB support.

- [ ] CI/CD automation (boot tests, cross-arch builds, fuzzing).

- [ ] Wiki alignment (migrate .md files, diagrams, glossary).
 