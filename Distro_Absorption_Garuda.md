# Distro Absorption: Garuda Linux

> **Status**: 📋 Planned | **Source Paradigm**: Garuda Linux | **Target Shard**: `SigmaOS Performance Profiles`

---

## 1. Executive Summary

Garuda Linux is an Arch-based distribution famous for extreme performance tuning out-of-the-box. It ships with the Zen kernel, ananicy-cpp for process priority management, and zram by default, enabling elite gaming and developer responsiveness on commodity hardware.

SigmaOS absorbs Garuda's **performance profiling daemon** and **automatic process niceness management**, integrating them into the AI-driven scheduler layer.

---

## 2. Key Features to Absorb

### 2.1 Ananicy-Inspired Process Niceness (`sigma-prio`)

Instead of all processes competing equally for CPU cycles, `sigma-prio` is a system daemon that reads a curated database of known applications and their expected priority categories.

```toml
# /etc/sigma/process_rules.toml
[[rule]]
process_name = "firefox"
nice_level = -5
ioclass = "BE"

[[rule]]
process_name = "cargo build"
nice_level = 10
ioclass = "IDLE"
```

When a user launches Firefox, it gets instantaneous priority over background build processes.

### 2.2 Automatic zram Configuration

Like Garuda, SigmaOS activates hardware-compressed RAM swap (ZRAM) automatically, providing up to 3× additional usable memory on low-RAM systems.

---

## 3. References & Standards

- Garuda Linux — `garudalinux.org`
- ananicy-cpp — `github.com/Nefelim4ag/ananicy-cpp` (GPL-2.0)
