# Distro Absorption: Tails — Amnesic Privacy OS

> **Status**: 📋 Planned | **Source Paradigm**: Tails OS | **Target Shard**: `SigmaOS Privacy Mode`

---

## 1. Executive Summary

Tails (The Amnesic Incognito Live System) is a portable Debian-based OS that routes all network traffic through Tor, leaves no trace on the host machine, and provides built-in encryption tools. It is the gold standard for journalist and activist privacy.

SigmaOS absorbs Tails' **amnesic boot** (no persistent state by default), **forced Tor routing**, and **forensic resistance** patterns into a dedicated `PROFILE=privacy` deployment mode.

---

## 2. Key Features to Absorb

### 2.1 Amnesic Boot Mode

When SigmaOS boots in privacy mode, all filesystem writes go to a RAM-backed tmpfs. On shutdown, all data is zeroed. No disk writes occur unless the user explicitly enables an encrypted persistent volume.

```bash
$ sigma boot --profile privacy
Σ [BOOT] Privacy Mode activated:
  Filesystem:  RAM-only (tmpfs, 2GB)
  Persistence: DISABLED (enable with --persist /dev/sda2)
  Network:     Tor-only (all clearnet blocked)
  On shutdown: RAM wipe + secure erase
```

### 2.2 Forced Tor Network Routing

In privacy mode, the `sigma-net` shard enforces that **all** outbound traffic passes through a Tor circuit. Direct clearnet connections are blocked at the firewall level. DNS requests go through Tor's DNS resolver.

```bash
$ sigma net status --profile privacy
Σ [NET] Privacy network status:
  Tor circuit:  ESTABLISHED (3 hops)
  Exit node:    de-exit-42
  DNS:          Tor DNS resolver
  Clearnet:     BLOCKED (all ports)
  Leak test:    PASS ✓
```

### 2.3 Anti-Forensic Shutdown

On shutdown, SigmaOS privacy mode overwrites all RAM pages with cryptographic random data before powering off, preventing cold-boot attacks.

---

## 3. References & Standards

- Tails — `tails.net` (GPL-3.0)
- Tor Project — `torproject.org`
