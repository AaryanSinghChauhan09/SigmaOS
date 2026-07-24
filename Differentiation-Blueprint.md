# SigmaOS Differentiation Blueprint

Execution plan for outpacing SteamOS, Clear Linux, NixOS, Fedora CoreOS, Flatcar, Solus, Rescuezilla, and RancherOS by combining sovereign low-level subsystems with polished user-facing experience.

---

## 1. Strategic Positioning

SigmaOS wins only if it is better in **both** axes:

- **Technical sovereignty**: own critical runtime layers (boot, net, container isolation, libc primitives, driver path)
- **UX and operations quality**: polished Zenith desktop, resilient recovery, automation, and transparent docs/release workflows

Neither axis alone is sufficient. A technically perfect OS with bad UX loses. A beautiful OS with unreliable kernel loses.

---

## 2. Current Gaps (Most Material)

| Gap | Impact | Phase to Fix |
|---|---|---|
| Hardware support breadth (GPU/WiFi/BT) | Blocks real hardware boot | Phase 1–2 |
| Container execution depth | sigma-pod CLI exists but kernel enforcement incomplete | Phase A |
| Recovery UX | Fallback shell exists but no guided recovery | Phase B |
| Desktop completeness | Zenith compositor exists but not production-hardened | Phase B |
| Build/package reproducibility | Direction exists, registry lifecycle not formalized | Phase C |

---

## 3. Phase Plan

### Phase A — Hardening Core Sovereignty (0–90 days)

**Networking**
- Complete RX/TX driver-to-stack loop for active NICs
- Enforce single socket ABI ownership
- Remove duplicate APIs between `kernel/net/` and `net/sockets/`

**Containers**
- Finish orchestrator handling for native namespace/cgroup spec
- `sigma-pod run-native` must apply limits in kernel path — not just CLI

**Boot Resilience**
- Promote safe-mode policy to default for repeated failed boot attempts
- Recovery menu accessible from boot stage without userspace

**Minimal libc Path**
- Prioritize `sigma_memcpy`, `sigma_strlen`, formatted output on hot codepaths

**Exit Criteria:**
- Boot success ≥ 99% in CI virtual profiles
- Packet TX/RX tests passing on ≥ 2 NIC targets
- Native pod launch applies namespace + cgroup limits in kernel path

---

### Phase B — Product-Grade UX + Recovery (90–180 days)

**Zenith Polish**
- Stable compositor loop
- Predictable window placement
- Accessibility hooks (AT-SPI2)
- Deterministic input handling

**Auto-Tiling WM**
- First-party tiling policy in Zenith
- Profile-aware defaults (developer = tiling, gaming = fullscreen, standard = floating)

**Personalization Engine**
- Declarative `~/.sigma_profile` + theme/layout presets
- Hot-reload without restart

**Recovery Surface**
- Recovery assistant for rollback/snapshot selection
- Diagnostics export (logs, kernel panic traces, hardware info)

**Exit Criteria:**
- UI smoke suite passes across standard profile matrix
- Recovery flow restores known-good boot without manual kernel edits
- Profile-driven desktop state reliably restored after reboot

---

### Phase C — Ecosystem + Transparency (180–365 days)

**Sovereign Registry Maturity**
- Deterministic build recipes
- Dilithium3-signed packages
- Provenance checks on every install

**Automation Hooks**
- Update/backup/recovery orchestration
- Scheduler integration for background maintenance

**GitHub-First Transparency**
- Changelog discipline: every subsystem change → docs update
- CI gating for kernel/driver/UI paths
- Wiki pages map directly to maintained subsystem owners

**Exit Criteria:**
- Every subsystem PR includes test evidence + docs update
- Release candidates include validated rollback + recovery evidence
- Wiki pages map directly to maintained subsystem owners

---

## 4. Competitor-Specific Surpass Strategy

| Competitor | Their Strength | SigmaOS Strategy |
|---|---|---|
| **SteamOS** | Gaming integration, Proton, polished UX | Sovereign graphics + predictable low-latency desktop + dev tooling |
| **Clear Linux** | Intel-tuned performance | Silicon-aware scheduling + cross-vendor PGO, not vendor-specific only |
| **NixOS** | Declarative reproducibility | Sovereign `.spkg` recipes + policy-backed registry + first-party automation |
| **Fedora CoreOS / Flatcar** | Immutable base, auto-updates | Immutable updates + automatic rollback + stronger local recovery |
| **RancherOS** | Container-first, Docker-native | Native container-first without Docker daemon dependency |
| **Solus** | Cohesive desktop UX | Zenith + first-class personalization + declarative profile flow |
| **Rescuezilla / SystemRescue** | Mature recovery GUI | Integrated recovery mode + rollback built into normal operation |
| **CAINE / Tails** | Forensic/anonymity specialization | Secure profile + WORM audit + zero-trace RAM scrubbing |
| **Ubuntu / Canonical** | Enterprise support, snaps | Profiles + automation + sovereign orchestration + India-native |

---

## 5. Operating Rules

No new subsystem is "done" unless it has:
1. Runtime tests
2. Recovery behavior documented
3. Docs update committed
4. Ownership declaration in `CODEOWNERS`

**Prefer** replacing dependency hot paths incrementally over risky all-at-once rewrites.

**Treat** reliability and polish as equal release gates — not afterthoughts.

---

## 6. Phase A/B/C Checklist Summary

### Phase A — Core Sovereignty
| Task | File | Status |
|---|---|---|
| NIC TX/RX driver-to-stack loop | `kernel/net/sigma_net.c` | `[~]` |
| TCP state machine | `kernel/net/sigma_net_tcp.cpp` | `[~]` |
| ARP resolution (replace stub) | `kernel/net/sigma_net_arp.cpp` | `[~]` |
| Single socket ABI authority | `kernel/net/sigma_net_socket.cpp` | `[~]` |
| sigma-pod kernel cgroup enforcement | `kernel/core/process/sigma_cgroup.c` | `[~]` |
| Boot resilience safe-mode default | `kernel/core/boot/sigma_boot_recovery_menu.c` | `[~]` |
| Microsecond SYSCALL asm entry | `arch/x86_64/syscall_entry.asm` | `[ ]` |

### Phase B — UX + Recovery
| Task | File | Status |
|---|---|---|
| Compositor input event loop | `zenith_desktop/compositor/` | `[~]` |
| Auto-tiling WM | `zenith_desktop/wm/sigma_tiling_wm.cpp` | `[~]` |
| Theme/widget engine | `zenith_desktop/theme/sigma_theme_engine.cpp` | `[~]` |
| Profile engine | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `[~]` |
| Recovery assistant GUI | `kernel/core/boot/sigma_boot_recovery_menu.c` | `[ ]` |

### Phase C — Ecosystem
| Task | File | Status |
|---|---|---|
| Sovereign .spkg registry | `sigma-pkg/cbuild.py` | `[~]` |
| Signed recipe provenance | `userland/pkg/sigma_sbom.h` | `[~]` |
| CI provenance gating | `.github/workflows/sigma_ci.yml` | `[~]` |
| Wiki-per-subsystem policy | `wiki_repo/` | `[x]` |

---

*See also: [Gap Analysis](Gap-Analysis) · [Development Roadmap](Development-Roadmap) · [Competitive Gap Matrix](Competitive-Gap-Matrix) · [Zenith System Improvement Plan](Zenith-System-Improvement-Plan)*
