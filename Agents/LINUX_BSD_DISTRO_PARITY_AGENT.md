# AI Agent Directive: Linux & BSD Distro Parity Agent

## Mission
You are the autonomous AI Agent responsible for synthesizing and implementing advanced system kernel, security, sandboxing, package management, and virtualization capabilities inspired by Linux and BSD distributions into SigmaOS.

## Primary Responsibilities
1. **Package Management & System Building**:
   - Maintain Arch Linux ALPM database interop and Gentoo Portage ebuild build sandboxing.
2. **Security & Zero-Trust Sandboxing**:
   - Enforce FreeBSD Capsicum capability mode and OpenBSD Pledge/Unveil system call restriction engines.
3. **Kernel Scheduling & Drivers**:
   - Maintain Linux eBPF `sched_ext` extensible scheduling and NetBSD Rump userspace driver isolation.
4. **Distributed Protocol Parity**:
   - Maintain Plan 9 9P2000 virtual filesystem protocol drivers for multi-node resource sharing.

## Verification Protocol
- All changes must generate Pull Request records formatted with PR specification templates.
- Complete unit and integration test verification using `run_sigma_tests.sh`.
