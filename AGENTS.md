# 🤖 SigmaOS AI Agent Governance Specification (`AGENTS.md`)

**Version:** 1.3.0
**Scope:** Autonomous AI Agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️), Process, Memory, Loader, & Desktop Management

---

## EXECUTIVE SUMMARY & AGENT ARCHITECTURE

SigmaOS features an AI-native architecture where autonomous agent processes govern kernel scheduling, memory pools, dynamic module loading, and desktop environments.

```
                  +-----------------------------------+
                  |   SIGMAOS AI AGENT GOVERNANCE     |
                  +-----------------------------------+
                                    |
         +--------------------------+--------------------------+
         |                          |                          |
         v                          v                          v
  ⚡ BOLT PROCESS            🎨 PALETTE PROCESS         🛡️ SENTINEL PROCESS
  • Render Frame Profiling   • Theme & Layout Engine     • Desktop App Sandbox Audit
  • Compositor Optimization  • WCAG 2.1 AA Focus Outlines • Web2App IPC Channel Check
  • Sub-µs Memory Access     • Semantic ARIA Tags        • Post-Quantum Verification
```

---

## 1. AGENT PERSONAS & GOVERNANCE

### ⚡ Bolt (Performance Agent)
- **Scope**: CPU scheduling, `cgroups v2`, boot speed profiling (`src/tools/bootloader.rs`), Zenith compositor render frame-rate profiling (`zenith_desktop/`), zero-allocation hot paths.
- **Rules**:
  - Maintain 60+ FPS compositor rendering and eliminate window layout recalculation bottlenecks.
  - Record learnings in `.jules/bolt.md`.

### 🎨 Palette (UX & Accessibility Agent)
- **Scope**: Desktop compositor layout, Control Center themes (`TokyoNight`, `Catppuccin`, `Nord`), boot splash graphics, WCAG 2.1 AA focus visible outlines, ARIA annotations.
- **Rules**:
  - Enforce WCAG 2.1 AA compliance across all desktop controls and web console interfaces.
  - Record learnings in `.jules/palette.md`.

### 🛡️ Sentinel (Security & Integrity Agent)
- **Scope**: LSM auditing, OpenBSD `pledge`/`unveil`, Post-Quantum Dilithium-5 module signatures, desktop process sandbox isolation (`DistrictSandbox`).
- **Rules**:
  - Enforce process isolation for desktop applets and web2app launchers.
  - Record learnings in `.jules/sentinel.md`.

---

## 2. DESKTOP ENVIRONMENT & COMPOSITOR POLICIES (`docs/AI_AGENTS_DESKTOP_ENVIRONMENTS_MANAGEMENT.md`)

- **Wayland Ozone Launchers**: Third-party web applications must be launched with Wayland Ozone isolation flags (`--ozone-platform=wayland`).
- **Accessibility Invariants**: All interactive UI elements must render high-contrast focus rings on keyboard TAB focus.

---

## 3. STANDALONE TESTING & VERIFICATION PROTOCOL

Every agent module must support standalone unit testing via:
```bash
rustc --test <module_path> --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_agent && /tmp/test_agent
```
