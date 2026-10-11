# AI Agent Future Development Roadmap: Observability & Developer Experience

This roadmap details developer platform tooling, kernel observability, and testing harness specifications for SigmaOS.

---

## 1. Observability & Tracing Architecture

SigmaOS delivers kernel and userland tracing parity inspired by Illumos DTrace, Linux eBPF/BPFtrace, and FreeBSD KTrace.

### 1.1 Tracing Primitives
* **eBPF VM & Verification**: JIT-compiled bytecode execution for tracepoints, kprobes, uprobes, and socket filters.
* **DTrace Provider Parity**: `syscall`, `fbt` (Function Boundary Tracing), `sched`, and `proc` provider probes with zero kernel panic risk.
* **Kernel Audit Subsystem**: Audit log record generation for file watches, syscall filters, and capability checks (`auditd` parity).

---

## 2. Developer Platform & Testing Harness

### 2.1 Kyua & kselftest Parity
* **Kyua Automated Test Runner**: ATF (Automated Testing Framework) test suite execution for kernel and userland regressions.
* **Linux kselftest Integration**: Isolated test execution for memory management, cgroups, network namespaces, and seccomp.
* **AI Safety & Model Management**: Curated AI model marketplace sandboxing with safety violation auditing (`AiSafetyGuardrails`).

---

## 3. Milestone Execution Schedule

| Milestone | Focus Domain | Key Technologies | Objectives |
| :--- | :--- | :--- | :--- |
| **Phase 1** | Tracing & eBPF | eBPF JIT, DTrace provider probes | Real-time low-overhead system observability |
| **Phase 2** | Audit & Security | Kernel Audit, Seccomp-BPF | System call filtering and security event logging |
| **Phase 3** | Test Harnesses | Kyua runner, kselftest | Continuous regression testing for kernel/userland |
| **Phase 4** | Developer Studio | AI Safety, Model Marketplace | Isolated workspace for AI-driven software development |
