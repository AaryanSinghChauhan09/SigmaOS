# Distro Absorption: EndeavourOS

> **Status**: 📋 Planned | **Source Paradigm**: EndeavourOS | **Target Shard**: `SigmaOS Welcome & Diagnostics Suite`

---

## 1. Executive Summary

EndeavourOS is an Arch-based rolling-release distribution that provides a minimal, terminal-centric installation with an extremely helpful welcoming interface and helper scripts to guide users.

SigmaOS absorbs the **Minimal Offline Installer** and **Diagnostic Helper Scripts** philosophy, ensuring users who prefer command-line installation are not left in the dark during hardware setup.

---

## 2. Key Features to Absorb

### 2.1 Terminal Diagnostics Wrapper (`sigma-welcome`)

Upon launching a new installation of SigmaOS, the user is greeted with a terminal dashboard providing quick actions for driver configuration, package updating, and log checks.

```bash
$ sigma welcome
Welcome to SigmaOS!
  [1] Update packages (sigma-pkg update)
  [2] Configure graphics cards (sigma-hw)
  [3] Read latest release notes
  [4] Audit system health logs (sigma-trace)
```

---

## 3. References & Standards

- EndeavourOS — `endeavouros.com`
