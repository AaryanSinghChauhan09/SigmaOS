# Distro Absorption: Slackware

> **Status**: 📋 Planned | **Source Paradigm**: Slackware Linux | **Target Shard**: `SigmaOS Base Philosophy`

---

## 1. Executive Summary

Slackware is the oldest surviving Linux distribution. Its defining philosophy is extreme simplicity and purity: it provides software exactly as the original upstream authors intended, with zero distribution-specific patching, and it uses incredibly simple bash scripts for package management and init.

While SigmaOS is mathematically and architecturally advanced, it absorbs Slackware's **Upstream Purity** and **KISS (Keep It Simple, Stupid)** text-based configuration for its deepest recovery environments.

---

## 2. Key Features to Absorb

### 2.1 Upstream Purity

When SigmaOS builds a package (e.g., Python, Nginx), it explicitly rejects the Debian/Ubuntu philosophy of applying hundreds of custom patches to make the software behave in a "SigmaOS way".

If a piece of software is broken upstream, SigmaOS pushes the fix upstream. Packages in the `sigma-store` are bit-for-bit compiled from vanilla upstream tarballs. This drastically reduces the maintenance burden and prevents distribution-specific bugs.

### 2.2 Text-Based Configuration Fallback

When advanced declarative tools (like `sigma system apply`) fail, the system can always be configured via simple, plain-text files.

SigmaOS maintains a Slackware-style recovery mode where there is no registry, no binary blobs for configuration, and no complex XML. Everything from networking to boot parameters is a simple `key=value` text file that can be edited with `vi` from a rescue shell.

```bash
# Emergency text-mode configuration
$ vi /etc/sigma/sys.conf

# Just simple key-values
HOSTNAME=sigma-recovery
NET_IF=eth0
NET_IP=192.168.1.10
```

### 2.3 `pkgtool` Inspired Terminal UI

For terminal-based system installation and rescue, SigmaOS utilizes a curses-based UI (`sigma-setup`) heavily inspired by Slackware's `pkgtool` and installer. It is robust, works over serial consoles, and requires no graphics drivers.

```
┌───────────────── SIGMAOS SETUP ─────────────────┐
│                                                 │
│  [1] Disk Partitioning (cfdisk)                 │
│  [2] Install Base System                        │
│  [3] Configure Bootloader (sigma-boot)          │
│  [4] Network Configuration                      │
│                                                 │
│             < OK >      < Cancel >              │
└─────────────────────────────────────────────────┘
```

---

## 3. References & Standards

- Slackware Linux — `slackware.com` (GPL-2.0 / various)
