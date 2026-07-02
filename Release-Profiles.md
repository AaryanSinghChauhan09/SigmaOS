# Release Profiles

SigmaOS ships **two kinds of profiles**: deployment profiles (which branch / build target to use for a given hardware or use-case) and **profession profiles** (pre-configured shard bundles that tune the OS for a specific role). This page covers both.

---

## Deployment Profiles

Each deployment profile maps to a `release/*` branch and a CMake toolchain file. They all share the same freestanding Ring-0 microkernel but enable/disable subsystems via USE flags.

### Quick Selection

```
Laptop / workstation   →  release/standalone   (profiles/workstation.cmake)
Thin client / browser  →  release/browser      (default standalone build)
Bare server / research →  release/microkernel  (SIGMA_USE_ZENITH_DE=0)
ARM tablet / Pi        →  release/mobile       (profiles/iot-minimal.cmake)
Industrial / robotics  →  release/rtos         (SIGMA_SCHED_REALTIME=1)
Windows/Linux coexist  →  release/dual-boot    (GRUB chain-load)
Cloud VM (single node) →  release/cloud        (SIGMA_PROFILE=cloud-x86)
Multi-node cluster     →  release/distributed  (SIGMA_USE_CLUSTER=ON)
```

### USE Flag Matrix

| Flag | standalone | browser | microkernel | mobile | rtos | cloud | distributed |
|---|---|---|---|---|---|---|---|
| `SIGMA_USE_HYPERVISOR` | ON | ON | OFF | OFF | OFF | ON | ON |
| `SIGMA_USE_AI_ENGINE` | ON | ON | OFF | OFF | OFF | ON | ON |
| `SIGMA_USE_ZENITH_DE` | ON | ON | OFF | OFF | OFF | OFF | OFF |
| `SIGMA_USE_CLUSTER` | OFF | OFF | OFF | OFF | OFF | OFF | ON |
| `SIGMA_USE_BLUETOOTH` | ON | ON | OFF | ON | OFF | OFF | OFF |
| `SIGMA_USE_WIFI` | ON | ON | OFF | ON | OFF | OFF | OFF |
| `SIGMA_USE_CRYPTFS` | ON | ON | ON | ON | ON | ON | ON |
| `SIGMA_USE_PQ_NET` | OFF | OFF | OFF | ON | OFF | ON | ON |
| `SIGMA_USE_WASM` | ON | ON | OFF | OFF | OFF | OFF | OFF |

Set flags on the command line:
```bash
make SIGMA_USE_ZENITH_DE=0 SIGMA_USE_AI_ENGINE=0   # headless server
cmake -DCMAKE_TOOLCHAIN_FILE=profiles/iot-minimal.cmake -B build  # IoT
```

---

## Profession Profiles

Introduced in `release/microkernel` v15.0, profession profiles are cryptographically attested shard bundles. Each profile pre-configures the kernel, userland, and toolchain for a specific role.

Activate with:
```bash
sigma-pkg install --profile ai-researcher
# or at build time:
cmake -DSIGMA_PROFILE_BUNDLE=ai-researcher -B build
```

---

### AI Researcher

Tuned for ML training and inference workloads.

- **Kernel shards**: `S-CUDA`, `S-ROCm`, `S-NNFS`
- **Userland tools**: JupyterLab, PyTorch, TensorFlow, TensorBoard, NumPy, scikit-learn
- **Scheduler**: GPU-shard orchestration, high-priority ML threads
- **Workflow**: Automated model attestation via Dilithium3 signatures; GPU-accelerated compute with direct silicon access

---

### Cybersecurity Analyst

Zero-trust networking and silicon-level forensic auditing.

- **Kernel shards**: `S-PLOIT`, `S-MAP`, `S-AUDIT`
- **Userland tools**: Metasploit, Wireshark, Nmap, GPG, Volatility
- **Security**: All processes pledged to minimal promise sets; full audit ring buffer; IDS alerts wired to Zenith notification center
- **Workflow**: Live forensic mode, zero-trust policy enforcement dashboard, packet capture piped to native analysis tools

---

### Data Scientist

High-performance telemetry sharding and scientific compute.

- **Kernel shards**: `S-PANDAS`, `S-JULIA`, `S-R`
- **Userland tools**: NumPy, matplotlib, scikit-learn, Apache Spark, Jupyter
- **Scheduler**: NUMA-aware thread pinning to minimize cross-socket latency on multi-socket machines
- **Workflow**: Automated dataset attestation; sigma-pkg zero-install for scientific libraries

---

### Software Engineer

OCI container orchestration and rapid lattice compilation.

- **Kernel shards**: `S-CLOUD`, `S-GIT`, `S-WASM`
- **Userland tools**: VS Code (via Zenith iframe), GCC/Clang, Go, Rust, Node.js, Docker-equivalent (`sigma-pod`)
- **Features**: SigmaCode IDE with Monaco editor, SigmaTerm PTY, sigma-pkg zero-install binaries
- **Workflow**: `sigma-pod run-native` for namespaced containers, `sigma-pkg update --delta` for incremental updates

---

### CS Educator

Sandboxed student environments with sub-millisecond execution feedback.

- **Kernel shards**: `S-PLAY`, `S-DSA`
- **Userland tools**: Interactive REPL, DSA libraries, algorithm tutorials, NCERT virtual lab simulators (Class 1–12)
- **Security**: Strict unveil/pledge — student processes see only their own home directory
- **Workflow**: Isolated per-student sandbox; instructor can revoke capabilities in real time via zero-trust policy

---

### Gaming

High-performance gaming with dynamic GPU scheduling.

- **Kernel shards**: `S-GPU`, `S-AUDIO`, `S-INPUT`
- **Userland tools**: Sovereign compositor with auto-tiling WM, controller manager, low-latency audio pipeline
- **Scheduler**: Dynamic GPU scheduler (`SovereignGPUSched`), background tasks pinned to efficiency cores
- **Features**: Direct framebuffer access, Vulkan compositing, zero-X11/Wayland overhead

---

### Industrial / RTOS

Deterministic execution for safety-critical systems.

- **Kernel shards**: `S-SCHED_SOVEREIGN`, `S-PLC`, `S-CAN`
- **Features**:
  - Hard real-time class: `SCHED_SOVEREIGN` (priority > 80)
  - Lock-free SPSC ring buffers for sub-microsecond IPC
  - Priority inheritance via `SovereignMutex` (no unbounded priority inversion)
  - seccomp filter: only whitelisted syscalls permitted
- **Target hardware**: ARM Cortex-M/R, x86 PLC controllers, robotics platforms

---

### Enterprise / Cloud

Compliance-first, immutable deployments.

- **Kernel shards**: `S-IMMUTABLE`, `S-CGROUPS`, `S-AUDIT`
- **Features**:
  - Immutable root filesystem (`SovereignImmutableHostEngine`)
  - A/B partition swap with attestation before commit
  - `sigmad-fleet` enterprise telemetry (SSE streaming, 30-second dashboard refresh)
  - Karma-gated staged rollout: canary (1%) → testing (10%) → stable
- **Compliance**: Audit log with real monotonic timestamps (ISO 8601, millisecond precision)

---

## Adding a Custom Profile

1. Create `profiles/my-profile.cmake`:
```cmake
set(SIGMA_PROFILE "my-profile" CACHE STRING "" FORCE)
set(SIGMA_USE_ZENITH_DE    ON  CACHE BOOL "" FORCE)
set(SIGMA_USE_AI_ENGINE    OFF CACHE BOOL "" FORCE)
set(SIGMA_USE_HYPERVISOR   OFF CACHE BOOL "" FORCE)
set(SIGMA_USE_PQ_NET       ON  CACHE BOOL "" FORCE)
```

2. Create a shard bundle recipe at `sigma_pkg_registry/recipes/my-profile.sigma.recipe`

3. Build:
```bash
cmake -B build -DCMAKE_TOOLCHAIN_FILE=profiles/my-profile.cmake
make -C build -j$(nproc)
```

---

*See also: [Branch Guide](Branch-Guide) · [Building from Source](Building-from-Source) · [App Manifest Format](App-Manifest)*
