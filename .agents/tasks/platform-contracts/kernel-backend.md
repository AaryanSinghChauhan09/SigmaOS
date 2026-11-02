# Kernel Backend Adapter Contract

> **Shared SigmaOS platform contract.** Define how Linux and BSD backends plug into the shared SigmaOS architecture. Each shared subsystem defines an interface; backends provide concrete adapters.

---

## Interface

trait KernelBackend { fn boot_log() -> BootLog; fn services() -> ServiceManager; fn packages() -> PackageAdapter; fn network() -> NetworkAdapter; fn security() -> SecurityAdapter; fn hardware() -> HardwareAdapter; }

## Invariants

- Shared subsystems depend only on the trait, never on a specific backend.
- Backend selection is deterministic (config-driven) and logged at boot.
- A backend that fails to initialize falls back to a minimal mode (no-op adapters) rather than panicking.
- New backends can be added by implementing the trait — no shared code changes.

## Pattern

Trait-based adapter: shared subsystem defines a `trait Backend { ... }`; Linux implements `LinuxBackend`, BSD implements `BsdBackend`. Selected at boot via `SIGMA_KERNEL_BACKEND=linux|bsd`.

## Linux Adapter

Implements KernelBackend using systemd, NetworkManager, pacman/apt/dnf, SELinux/AppArmor, sysfs, udev, modprobe.

## BSD Adapter

Implements KernelBackend using rc.d, ifconfig/wpa_supplicant, pkg/pkg_add, pledge/unveil/Capsicum, devinfo, devd, kldload.

## Failure Modes and Recovery

Backend not available (fall back to minimal mode + log), backend trait method panics (catch + log + degrade), backend misconfiguration (validate at boot, fail fast).

## Open Questions

Should a third backend (e.g., a custom SigmaOS microkernel) be supported? Current: trait-based, so yes — implement KernelBackend. Not in scope for Milestone 1.
