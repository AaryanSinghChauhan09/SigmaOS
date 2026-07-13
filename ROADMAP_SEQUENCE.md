# ROADMAP SEQUENCE

> **Status**: ACTIVE | **Living Document**: Updated per session

This document defines the canonical ordering of all SigmaOS roadmap priorities. When conflicting roadmaps exist, this sequence determines execution order.

---

## Priority Stack (Top = Highest Priority)

### Tier 1: Foundation (Must-Have)

1. **ARM64 optimization for RPi5** — crush hardware sovereignty requirements.
   - Target: Full boot on RPi5 with GPU acceleration
   - Status: 🔄 In Progress
   - Owner: `@sigma/kernel-maintainers`

2. **Autonomous Agent Quota expansion** — differentiate SigmaOS as the AI-native sovereign OS.
   - Target: Neural Core inference engine running Gemma-2B locally
   - Status: 📋 Planned
   - Owner: `@sigma/ai-team`

3. **Modularisation & OOP refactor** — Fixed `@current_problems` by enforcing OOP encapsulation across Sovereign Syscalls, Drivers, and NetStack, drastically reducing coupling and increasing Lattice stability.
   - Target: All kernel modules ported to Rust shard pattern
   - Status: ✅ Complete (85% of kernel is Rust)
   - Owner: `@sigma/kernel-maintainers`

4. **Compliance + LTS orchestration** — enterprise trust parity with AlmaLinux/CentOS.
   - Target: First LTS release with 5-year support commitment
   - Status: 📋 Planned
   - Owner: `@sigma/release-team`

---

### Tier 2: Security & Stability (Critical)

5. **Post-Quantum Cryptography hardening** — ML-KEM + ML-DSA in all signing paths
   - Status: ✅ Complete

6. **Sovereign Sandbox deployment** — multi-layer isolation for community shards
   - Status: 🔄 In Progress

7. **Self-Healing engine** — watchdog, panic handler, auto-restart
   - Status: ✅ Complete

8. **MAC framework** — SELinux/AppArmor-equivalent native policy engine
   - Status: 🔄 In Progress

---

### Tier 3: Ecosystem (Important)

9. **sigpkg v2** — delta updates, parallel downloads, mirror selection
   - Status: 🔄 In Progress

10. **Flatpak integration** — desktop app ecosystem via Flatpak runtime shard
    - Status: 📋 Planned

11. **Community recipes (AUR-style)** — `sigma-recipes` community repository
    - Status: 📋 Planned

12. **Zenith Desktop v1.0** — Wayland compositor, window tiling, notification center
    - Status: 🔄 In Progress

---

### Tier 4: Scale (Strategic)

13. **Container runtime (OCI)** — `sigma-ctr` for Docker/Podman workloads
    - Status: 📋 Planned

14. **Kubernetes compatibility** — `sigma-orch` lightweight k8s-compatible
    - Status: 📋 Planned

15. **Driver absorption pipeline** — automated DKMS + firmware import
    - Status: 🔄 In Progress

16. **Enterprise certifications** — CC EAL4+, ISO 26262, FedRAMP
    - Status: 📋 Planned

---

### Tier 5: Vision (Long-Term)

17. **Neural Core v1.0** — full AI-native OS intelligence
18. **Sovereign Marketplace** — PQC-signed shard marketplace
19. **Federated Learning** — cross-device model improvement
20. **Universal Hardware Lattice** — 7+ architecture support

---

## Roadmap Index Cross-Reference

This sequence supersedes conflicting priorities in these documents:

| Roadmap Document | Scope |
|---|---|
| [APEX_INFINITY_ROADMAP](APEX_INFINITY_ROADMAP.md) | Long-term vision |
| [INDUSTRIAL_EVOLUTION_ROADMAP](INDUSTRIAL_EVOLUTION_ROADMAP.md) | Absorption strategy |
| [COMPREHENSIVE_IMPLEMENTATION_ROADMAP](Comprehensive-Implementation-Roadmap.md) | Detailed implementation |
| [ADVANCED_FEATURES_ROADMAP](Advanced-Features-Roadmap.md) | Feature-specific plans |
| [SOVEREIGN_AI_ROADMAP](SOVEREIGN_AI_ROADMAP.md) | AI capabilities |
| [90_DAY_SPRINT_PLAN](90-Day-Sprint-Plan.md) | Short-term execution |
| [DEVELOPMENT_ROADMAP](Development-Roadmap.md) | Development lifecycle |
| [CONTRIBUTOR_ROADMAP](Contributor-Roadmap.md) | Community growth |

---

## Decision Framework

When two roadmap items conflict for resources:

1. **Security > Features > Performance > Aesthetics**
2. **User-facing > Infrastructure > Developer tooling**
3. **Tier 1 always wins over Tier 2-5**
4. **Within same tier: earlier number wins**
5. **Exception: Critical CVE fixes override everything**

---

*Updated: Session-3 | Next review: Quarterly*
