# Distro Absorption: NixOS — Declarative, Reproducible OS

> **Status**: 📋 Planned | **Source Paradigm**: NixOS | **Target Shard**: `SigmaOS Declarative Configuration Layer`

---

## 1. Executive Summary

NixOS is a Linux distribution built entirely around the Nix package manager's functional, declarative configuration model. The entire operating system — packages, services, users, files — is described in a single Nix expression. Any change generates a new system generation that can be rolled back atomically.

SigmaOS absorbs NixOS's **whole-system declarative configuration** and **generation-based rollback** as `sigma-declare`, giving administrators full system reproducibility without NixOS's learning curve.

---

## 2. Key Features to Absorb

### 2.1 Whole-System Declarative Configuration

A single `system.toml` file describes the desired state of the entire system: installed packages, enabled services, user accounts, firewall rules, and kernel parameters.

```toml
# /etc/sigma/system.toml — Complete system declaration
[system]
hostname = "sigma-workstation"
timezone = "Asia/Kolkata"

[packages]
installed = [
  "helix", "rust", "python3", "nodejs",
  "sigma-devtools", "sigma-forensic"
]

[services]
enabled = ["sshd", "sigma-agent", "sigma-gateway"]

[users.developer]
groups = ["sudo", "network", "audio"]
shell = "sigma-sh"

[firewall]
default_policy = "drop"
allow_in = ["tcp:22", "tcp:443"]
```

```bash
$ sigma-declare apply
Σ [DECLARE] Computing delta from current state...
  + Install: helix, sigma-forensic (2 new packages)
  ~ Change: firewall.allow_in (added tcp:443)
  Applying... done. Generation 14 created.
```

### 2.2 Generation-Based Atomic Rollback

Every `sigma-declare apply` creates a new immutable system generation. Rollback to any previous generation takes under 1 second.

```bash
$ sigma-declare generations
  GEN  DATE                  DESCRIPTION
  14   2026-07-13 16:30      Added helix + forensic tools
  13   2026-07-12 09:15      SSH hardening
  12   2026-07-10 14:00      Initial install

$ sigma-declare rollback 12
Σ [DECLARE] Rolled back to generation 12 in 0.4s. Reboot to apply kernel changes.
```

### 2.3 Flake-Inspired Pinned Dependencies

All package inputs are locked to exact content-addressed hashes, ensuring identical environments across time and machines.

---

## 3. References & Standards

- NixOS — `nixos.org` (MIT)
- Nix Package Manager — `nixos.org/manual/nix`
