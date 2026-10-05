# SigmaOS Development Plan: Arch, Linux Mint, and Omarchy

**Status:** proposed roadmap, not a release promise or a statement of current capability. Updated 2026-10-04.

SigmaOS is not ready for general installation. The current kernel target does not build for `x86_64-unknown-none`: the crate depends on `std` and has extensive `no_std` compile errors. There is no verified boot-to-shell path, installer, or recovery process. Complete each phase's acceptance checks before starting the next phase's release claim.

## Design lessons

- **Arch Linux:** keep package recipes and system changes inspectable, provide precise task-based documentation, and make the supported configuration explicit. Study the [ArchWiki style guide](https://wiki.archlinux.org/title/Help:Style) and [package guidelines](https://wiki.archlinux.org/title/Arch_package_guidelines).
- **Linux Mint:** make installation, onboarding, updates, and recovery understandable to people who do not want to maintain the operating system. Use Mint's [user and installation guides](https://linuxmint.com/documentation.php) as usability references.
- **Omarchy:** offer deliberate defaults, keyboard-first navigation, and discoverable commands without requiring users to configure every detail. See its [CLI manual](https://omarchy.org/manual/omarchy-cli/).
- **SigmaOS:** adapt these practices only when the system has a working implementation and test evidence. Keep prototype models visibly separate from runtime support.

## Phase 0: Establish a real boot target

**Work:** split the bootable kernel from the hosted `sigmaos` library; define the boot protocol, target, linker layout, memory map handoff, panic output, and serial console. Build a minimal kernel that does not link `std`. Add a reproducible image build that rejects missing inputs and never emits placeholder artifacts.

**Acceptance:** a clean checkout builds the same kernel/image with pinned tools; `grub-file` or an equivalent boot-protocol validator accepts the kernel; QEMU boots it and captures a kernel-ready serial marker. Record exact QEMU version, machine type, memory, image checksum, and log. Unsupported checks remain blocked, not passed.

## Phase 1: Reach a minimal usable text system

**Work:** initialize physical memory, interrupts, timer, serial/console input, one storage path, a minimal filesystem, process creation, syscall entry, init, and a small shell. Select one architecture and one virtual hardware profile first. Implement service startup with dependency failure handling; do not represent in-memory service models as spawned processes.

**Acceptance:** QEMU boots to a shell on the real kernel path; shell commands exercise filesystem and process syscalls; malformed requests fail safely; reboot and panic logs are reproducible. Add tests at module and QEMU integration levels.

## Phase 2: Add safe installation and recovery

**Work:** create a guided installer with explicit target-disk confirmation, dry-run plan, partition validation, interruption-safe writes, post-install checks, and recovery media. Borrow Mint's clear onboarding and recovery flow. Do not write to physical disks until the virtual-disk matrix is stable.

**Acceptance:** install to disposable QEMU disks from blank state; test cancellation and power-loss points; verify boot after install and documented recovery; preserve unrelated disks; publish checksums, image provenance, requirements, and known limitations.

## Phase 3: Define a supportable base system

**Work:** implement user/account provisioning with an audited password-hashing provider, privilege boundaries, signed package metadata, package transaction rollback, update rollback, logs, and an offline recovery path. Follow Arch's inspectable package recipe model while keeping package installation transactional and recoverable.

**Acceptance:** clean-install tests cover account creation, rejected credentials, permission boundaries, signature failures, interrupted package changes, update rollback, and recovery. No crypto, authentication, TPM, networking, or hardware feature is called supported based only on a mock or unit test.

## Phase 4: Hardware, networking, and desktop

**Work:** add devices one by one against a published test matrix. Establish storage, wired networking, input, display, and audio on the chosen virtual machine before expanding to physical hardware. Then build an accessible desktop with keyboard navigation, visible shortcuts, sensible defaults, and a discoverable command palette inspired by Omarchy and Mint.

**Acceptance:** every supported device has a named model, driver path, tested operations, logs, and known limitations. Desktop acceptance covers clean first login, keyboard-only use, display scaling, audio, network configuration, and recovery from a failed session.

## Phase 5: Release engineering and maintenance

**Work:** pin toolchains, build in CI, produce checksums and signed provenance, scan dependencies, publish a hardware support matrix, track regressions, and maintain a release/rollback policy. Offer long-term support only after the project can sustain security response and update commitments.

**Acceptance:** a tagged release is reproducible from a clean checkout, passes required unit/integration/QEMU/install/recovery suites, has a reviewed security and compatibility record, and can be upgraded and rolled back using documented steps.

## Development procedure

1. Identify the owning component page and current capability status.
2. State the behavior, failure modes, and acceptance checks before implementation.
3. Make a focused change; add regression tests for normal and failure paths.
4. Run formatting, targeted tests, relevant integration tests, and QEMU checks where available. Report unavailable checks explicitly.
5. Update the canonical component page, this phase's milestone, and capability inventory from measured evidence.
6. Merge only reviewed, passing pull requests. Do not merge every branch or closed pull request indiscriminately.

## Release gate

SigmaOS is ready for general use only after a person unfamiliar with the repository can obtain a verified release image, install it safely, create an account, boot to a usable session, use the documented supported devices, update, recover from a failed update, and get security fixes under a published support policy. Unit tests, mock models, a generated ISO, or feature-name parity do not meet this gate.
