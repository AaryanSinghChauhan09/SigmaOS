# SigmaOS Full-Platform Roadmap

## Overview
This document integrates the comprehensive SigmaOS roadmap specifications from .kiro/specs/sigmaos-roadmap, encompassing 40 requirements organized into five development phases (Phase 0–4) and three bug fix severity levels.

## Roadmap Structure

The roadmap transforms SigmaOS from a prototype into a production-grade, bootable, secure operating system with:

- **Phase 0**: Kernel Stability (Hardware Foundation)
- **Phase 1**: Shell Polish (User Experience)
- **Phase 2**: App Completions (Feature Parity)
- **Phase 3**: Platform Features (Advanced Capabilities)
- **Phase 4**: Ecosystem (Developer & Community)

## Architecture Context

SigmaOS is a minimal Chromium-based operating system built on Buildroot that boots to browser in under 3 seconds. The architecture consists of four layers:

1. **User Layer**: SigmaOS Shell (React/Svelte UI), PWAs, extensions, workspaces, AI kits
2. **Browser Layer**: Custom Chromium fork with SigmaOS APIs, multi-profile manager
3. **System Layer**: SigmaOS daemons (Go) handling processes, clipboard, workspaces
4. **OS Base Layer**: Minimal Linux (Buildroot) with systemd, bubblewrap, seccomp

## Security Model

- **Native messaging bridge**: Daemons listen locally, gated by SigmaOS extension
- **Bubblewrap containers**: All processes run in isolated namespaces
- **Capabilities system**: Websites must explicitly request capabilities

## Key Design Principles

1. **Boot to Web**: Direct boot to Chromium without traditional desktop environment
2. **Browser as OS Shell**: Window management via web apps
3. **Unix Philosophy for Web**: PWAs gain access to raw system primitives
4. **Zero-Bloat Packaging**: Alpine packages installed directly into user-space
5. **Strict Isolation**: Every execution context sandboxed by default

## Implementation Status

### Phase 0: Kernel Stability (Hardware Foundation)
- [ ] IDT initialization (`sigma_idt.h` / `sigma_idt.cpp`)
- [ ] Ring 3 usermode transition (`sigma_usermode.h` / `sigma_usermode.cpp`)
- [ ] CryptFS key derivation and root mount (`sigma_cryptfs.cpp`)
- [ ] Real PCI bus enumeration (`pci_scanner.h`)

### Phase 1: Shell Polish (User Experience)
- [ ] Window Manager with drag/resize handlers
- [ ] SigmaNotes split-pane Markdown preview
- [ ] SigmaCode IDE with Monaco/CodeMirror
- [ ] Notification Center with queue and badge
- [ ] Lock Screen with PIN verification

### Phase 2: App Completions (Feature Parity)
- [ ] SigmaTerm PTY with WebSocket multiplexer
- [ ] SigmaNotes AI integration
- [ ] SigmaPaint layers panel
- [ ] Neural UI Engine
- [ ] Enterprise Dashboard SSE connection

### Phase 3: Platform Features (Advanced Capabilities)
- [ ] Zero-Install sandbox execution
- [ ] Cloud Sync OAuth wizard
- [ ] SigmaAI Assistant
- [ ] Cross-App Clipboard daemon

### Phase 4: Ecosystem (Developer & Community)
- [ ] App Developer SDK
- [ ] App Store Backend
- [ ] GitHub/CI Hygiene
- [ ] Documentation Wiki

## Bug Fix Priorities

### Critical
- [ ] PID 1 watchdog loop in `sigma_init.cpp`
- [ ] ZeroTrust bounded string operations
- [ ] Revocation list consultation
- [ ] Extension Promise resolution
- [ ] Freestanding kernel build

### High
- [ ] Init service array bounds
- [ ] Complete kernel sources
- [ ] CI test activation
- [ ] Firewall packet inspection
- [ ] Audit log timestamps

### Medium
- [ ] Go daemon error handling
- [ ] Separate WiFi/Bluetooth builds
- [ ] Web Shell XSS prevention
- [ ] TCP fuzzer reproducibility
- [ ] CryptFS key derivation implementation

## Requirements Summary

The roadmap includes 40 detailed requirements covering:

1. **Package Management**: sigpkg format, registry API, client tooling
2. **System Libraries**: sigma_libc, dynamic loader, coreutils
3. **System Services**: init system, journal, SigmaFS
4. **Hardware Support**: initramfs, device manager, networking
5. **Graphics & Audio**: DRM driver, compositor, audio server
6. **Security**: Secure boot, module signing, MAC policy, TPM
7. **Development**: Toolchain, shells, SDK
8. **Virtualization**: virtio drivers, OCI runtime, installer
9. **Accessibility**: A11y subsystem, localization
10. **Quality**: Documentation, CI/CD, formal verification

## Implementation Guidance

### Property-Based Testing
The roadmap specifies 18 property-based tests for correctness verification:
- ISR handler invocation for all exception vectors
- Per-process TSS kernel stack isolation
- PCI device field capture completeness
- Window drag position translation
- Sandbox syscall enforcement
- Clipboard round-trip fidelity
- And more...

### Test Libraries
- **C**: `theft`/`rapidcheck`
- **JavaScript**: `fast-check`
- **Go**: `gopter`

### Release Plan
- **v0.1.0-alpha**: End of Phase 0 + Critical bugs
- **v0.2.0-beta**: End of Phase 1 + High bugs
- **v0.3.0**: End of Phase 2
- **v0.4.0**: End of Phase 3
- **v1.0.0**: End of Phase 4

## Related Documentation

- [Design Document](.kiro/specs/sigmaos-roadmap/design.md) - Technical implementation details
- [Requirements Document](.kiro/specs/sigmaos-roadmap/requirements.md) - Detailed acceptance criteria
- [Tasks Document](.kiro/specs/sigmaos-roadmap/tasks.md) - Incremental coding tasks
- [Security Guidelines](../security/) - Security best practices
- [Performance Guidelines](../performance/) - Optimization guidelines
- [UI/UX Guidelines](../ux/) - User interface best practices

## Notes

- This roadmap is a strategic planning document. Implementation should proceed incrementally following the phase dependency graph.
- Phase 0 kernel tasks must complete before any subsequent phases.
- Checkpoints are validation gates; all children of the preceding phase must pass before advancing.
- Property-based tests are co-located with their implementation sub-tasks and must pass before the parent task is marked complete.

## References

- Original specification from: .kiro/specs/sigmaos-roadmap/
- Spec ID: a96e2e85-5bec-408b-8a72-08ee8f6a4b39
- Workflow Type: requirements-first
- Spec Type: feature
