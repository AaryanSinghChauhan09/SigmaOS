# SigmaOS AGENTS.md — AI Agent Operating Instructions & Process Management Protocols

Welcome to the **SigmaOS** repository! This document outlines guidelines and operational rules for AI coding agents (such as Jules, Copilot, Herdr, or custom subagents) interacting with the codebase, managing system processes, virtual machines, and optimizing power usage in SigmaOS.

---

## 🤖 Core Directives for AI Agents

1. **Zero-Trust Capability Sandboxing**
   - Every AI agent process spawned in SigmaOS must execute inside a capability-bounded sandbox (`PLEDGE_STDIO | PLEDGE_RPATH | PLEDGE_WPATH | PLEDGE_INET`).
   - Use `process.pledge()` and `process.unveil()` before executing arbitrary userland commands.

2. **Cgroup Resource Quotas & Rate Limits**
   - AI agent task execution threads must be attached to the `/sys/fs/cgroup/system.slice/sigma-agent.service` cgroup.
   - Enforce memory quotas (`memory.max = 2G`) and CPU limits (`cpu.max = 50000 100000`) to prevent runaway resource consumption.

3. **IPC & Subagent Communication Channels**
   - Inter-agent communication MUST utilize `ZeroCopyIpcChannel` or `AndroidBinderIpc` with cryptographic token verification (`security_token`).
   - Direct memory sharing between agent processes without capability-gated handles is strictly forbidden.

4. **Virtual Machine Guest Provisioning**
   - Agents executing untrusted or experimental code MUST spawn an isolated guest VM via `VirtualizationOrchestrator` using KVM/Bhyve backends.
   - Attach virtio-fs shared paths with strict OpenBSD `unveil()` read-only restrictions.

5. **Power & Thermal Awareness**
   - Agents must check system power profiles and CPU temperature via `PowerGovernor` before launching compute-intensive subtasks.
   - Restrict concurrency and defer heavy background AI model indexing on battery power (`powersave` / `conservative` governor modes).

6. **Zero-Dependency Core Systems**
   - Avoid adding third-party standard C++ or non-vetted external dependencies.
   - Core kernel, driver, and shell primitives must rely on `ZeroDependencyPrimitiveHub` and `klib`.

---

## 🛠️ Build & Verification Instructions

AI agents making code changes must run the following checks before submitting pull requests:

```bash
# 1. Run quality gate verification
./scripts/sigma_quality_check.sh

# 2. Run UI/UX & accessibility verification
./scripts/uiux_accessibility_test.sh

# 3. Synchronize documentation mirrors
./sync_wiki.sh
```

---

## 📌 Related Documentation
- Process Management Architecture: [`docs/process-management.md`](docs/process-management.md)
- AI Agent Process Management Guidelines: [`docs/ai-agent-process-management.md`](docs/ai-agent-process-management.md)
- AI Agent Virtual Machine Management: [`docs/ai-agent-vm-management.md`](docs/ai-agent-vm-management.md)
- AI Agent Power & Thermal Management: [`docs/ai-agent-power-management.md`](docs/ai-agent-power-management.md)
- Sovereign Developer Guide: [`DEVELOPER_RULES.md`](DEVELOPER_RULES.md)
