# SigmaOS Roadmap

## Phase 0 (0-3 months)
- Finalize architecture RFC (microkernel vs hybrid, async syscalls)
- Set kernel coding standards (Rust, unsafe usage rules)
- Implement minimal reproducible build pipeline & cross-toolchain
- Build boot-to-userspace demo:
  - Boot kernel
  - Spawn userspace process
  - Demonstrate simple async syscall & IPC
- Create driver SDK prototypes (WASM + Rust host ABI)
- Configure CI with cross-compilation, unit tests, nightly benchmarks
- Begin outreach to 3 hardware vendors for driver partnerships

## Phase 1 (3-9 months)
- Kernel v0: memory management, basic process model, IPC, async syscall interface
- Basic userspace: shell, minimal filesystem, package manager skeleton
- Linux-compat prototype (container runtime or syscall shim)

## Phase 2 (9-18 months)
- Stable driver model (WASM host)
- NIC and block drivers (bare minimum)
- Scheduler tuning
- Basic security features (secure boot)
- Mature developer tooling

## Phase 3 (18-36 months)
- Full userspace stack
- Production-grade filesystems
- GPU/graphics stack
- NVMe performance optimizations
- Enterprise-grade CI
- Fuzzing everywhere
- Formal verification for critical modules

## Phase 4 (36+ months)
- Certifications
- Vendor partnerships
- Mainstream device driver coverage
- Migration tools
- Scale community and support offerings
