# Feature Implementation Roadmap

> **Status**: 🔄 Active | **Scope**: `Strategic Planning & Prototypes`

## 1. Executive Summary

To leap ahead of legacy Linux distros, SigmaOS strategically absorbs their best innovations across file systems, media frameworks, AI agents, and kernel runtimes. 

This roadmap tracks the transition of these features from **Prototype Branches** into the **Main Repository**.

---

## 2. Immediate Priorities (0-3 Months)

**Focus: Core Storage and Multimedia Foundations**

- [x] **Audit & Migrate Documentation**: Move all conceptual `.md` files to the GitHub Wiki.
- [x] **SigmaPkg Consolidation**: Unify multiple scripts into the `sigma-pkg` compatibility layer.
- [ ] **SigmaFS Prototype**: Integrate Btrfs/ZFS-style snapshotting and XFS scalability.
  - *Branch*: `feat/sigma-fs-core`
  - *Status*: Validating forensic checksum generation.
- [ ] **SigmaMedia Prototype**: Unify audio/video routing (PipeWire) with graph-based pipelines (GStreamer).
  - *Branch*: `feat/sigma-media-graph`
  - *Status*: Real-time kernel thread scheduling optimization.

---

## 3. Mid-Term Priorities (3-9 Months)

**Focus: Intelligence and Automation**

- [ ] **Embedded AI Orchestrator**: Local LLM backend for automation and predictive maintenance.
  - *Branch*: `feat/sigma-ai-daemon`
  - *Status*: Hardware NPU/GPU acceleration integration.
- [ ] **Adaptive UX Agents**: Dynamic desktop profiles (Developer, Gamer, Minimalist) based on active window telemetry.
  - *Branch*: `feat/sigma-adaptive-ux`
  - *Status*: Designing eBPF hooks for telemetry gathering.
- [ ] **Compliance Dashboards**: Real-time evaluation of GDPR, ISO, and SOC2 policies.

---

## 4. Long-Term Priorities (9+ Months)

**Focus: Kernel Security and Runtime Hardening**

- [ ] **Capability-Native Runtime**: Enforce fine-grained cryptographic tokens at the syscall level (seL4/QubesOS inspired).
  - *Branch*: `feat/kernel-capabilities`
  - *Status*: Researching formal verification models.
- [ ] **Self-Healing Kernel**: Automatic isolation and restart of faulty kernel modules.
  - *Branch*: `feat/kernel-recovery`
  - *Status*: Concept planning.
- [ ] **Zero-Trust Boot**: TPM-integrated cryptographic verification of the entire boot chain.
  - *Branch*: `feat/sigma-secure-boot`
  - *Status*: Integrating Post-Quantum cryptography.

---

## 5. Branch Workflow

1. **Prototype Branch**: Feature is developed in isolation (e.g., `feat/sigma-fs-core`).
2. **Validation**: Automated tests and hardware validation runs.
3. **Merge**: Once stable, the branch is merged into `main` via Pull Request.
4. **Cleanup**: The prototype branch is deleted.
5. **Documentation**: Design docs are migrated to the GitHub Wiki.
