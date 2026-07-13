# Distro Absorption: Fedora — Bleeding-Edge Innovation Distro

> **Status**: 📋 Planned | **Source Paradigm**: Fedora Linux | **Target Shard**: `SigmaOS Innovation Fast-Track`

---

## 1. Executive Summary

Fedora is Red Hat's upstream innovation distribution — the proving ground where new Linux technologies (Wayland, Btrfs default, systemd, Flatpak, Pipewire) debut before being stabilized in RHEL. Fedora's **Fedora Change Process** ensures that breaking changes are proposed, tracked, and communicated transparently.

SigmaOS adopts Fedora's **formal feature change process** and **beta-first technology adoption** model, creating a structured pipeline for evaluating and integrating new kernel features and userland innovations.

---

## 2. Key Features to Absorb

### 2.1 Sigma Change Process (SCP)

Modeled after Fedora's Change Process, every significant technology addition to SigmaOS must go through the Sigma Change Process:

```markdown
## SCP-001: Enable Huge Transparent Pages (THP) by Default
- Owner: @kernel-team
- Status: FinalBeta
- Self-Contained: Yes
- Summary: Enable THP for all anonymous mappings >1MB
- Benefit: 8-15% speedup in compilation and database workloads
- Contingency: Revert to madvise-only if OOM regression detected
- Completion: Included in SigmaOS 0.4
```

### 2.2 Sigma.next — Innovation Fast Track

`sigma.next` is SigmaOS's equivalent of Fedora Rawhide — a rolling-release branch that always tracks `main` plus the latest experimental features. Users can opt in for maximum freshness with minimal stability guarantees.

```bash
$ sigma-pkg channel set sigma.next
Σ [PKG] Channel changed to sigma.next (rolling, experimental)
  Warning: sigma.next receives daily updates; regressions possible.
  Backup snapshots recommended before update.

$ sigma update
Σ [UPDATE] sigma.next update:
  87 packages updated, 3 new features activated.
```

---

## 3. References & Standards

- Fedora Linux — `fedoraproject.org` (GPL-2.0+)
- Fedora Change Process — `docs.fedoraproject.org/en-US/program_management/changes_policy`
