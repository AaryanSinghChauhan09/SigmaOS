# SigmaOS Sovereign Roadmap 🗺️

SigmaOS draws inspiration from the best aspects of various Linux distributions:

- **Debian‑style stability** → predictable releases
- **Fedora‑style innovation** → cutting‑edge drivers/security
- **Arch‑style flexibility** → modular FS and userland
- **Ubuntu‑style ecosystem** → strong community and package management

## Phase 1: Core System & Stability

- [ ] Unify branches into a stable main
- [ ] Kernel scheduler: finalize Round Robin/EDF into a robust, tested default
- [ ] Memory allocator: stress‑test and formally verify
- [ ] Syscall layer: expand non‑POSIX ABI for consistency
- [ ] Release cadence: adopt predictable stable releases

## Phase 2: Hardware Support

- [ ] Networking: expand NIC support beyond e1000
- [ ] Storage: add NVMe, SSD optimizations
- [ ] USB/HID: implement keyboard, mouse, and USB stack
- [ ] Graphics: move from VGA framebuffer to modern GPU drivers
- [ ] Audio: add basic sound subsystem

## Phase 3: File Systems & Storage

- [ ] Enhance FS support: journaling, encryption, sovereign FS
- [ ] Add modern FS equivalents: ext4‑like, btrfs‑like features
- [ ] Virtualization drivers: VirtIO for cloud/server use cases

## Phase 4: Package Management & Build System

- [ ] Develop sigpkg: sovereign package manager
- [ ] Deterministic builds: reproducible recipes, cryptographic verification
- [ ] Profiles: sigma-core, sigma-desktop, sigma-cloud

## Phase 5: Security & Sovereignty

- [ ] Sandboxing: sovereign equivalents
- [ ] Audit framework: syscall monitoring
- [ ] Secure boot: expand cryptographic verification, rollback protection
- [ ] Exploit mitigations: hardened allocators, memory safety

## Phase 6: Userland & Ecosystem

- [ ] Expand utilities: sovereign replacements for GNU tools
- [ ] Shell (sigma-sh): scripting, automation, developer ergonomics
- [ ] SDK/toolchain: sovereign SDK for driver/app development

## Phase 7: Community & Adoption

- [ ] Contribution workflow: PRs only into main, modular tasks
- [ ] Wiki expansion: roadmap, coding standards, migration guides
- [ ] Target domains: secure systems, research, silicon sovereignty
