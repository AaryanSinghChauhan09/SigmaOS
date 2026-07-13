# Distro Absorption: Slackware — The Unix-Like Traditionalist

> **Status**: 📋 Planned | **Source Paradigm**: Slackware Linux | **Target Shard**: `SigmaOS KISS Core`

---

## 1. Executive Summary

Slackware is the oldest surviving Linux distribution. It adheres strictly to the Unix philosophy, avoiding heavy abstraction layers and maintaining configurations as plain text shell scripts. It famously lacks dependency resolution in its package manager (pkgtool), forcing administrators to understand their systems deeply.

While SigmaOS is highly modern, it absorbs Slackware's **Keep It Simple, Stupid (KISS) principle**, **transparent text-based initialization**, and **zero-magic system administration** to ensure the OS never hides its mechanics from the user.

---

## 2. Key Features to Absorb

### 2.1 Transparent Initialization (`rc.d`-style)

Before transitioning to the parallel `sigma-init` supervisor, early boot relies on simple, strictly sequential shell scripts that are easy to read and modify, inspired by Slackware's BSD-style `rc.d` scripts.

```bash
# /etc/sigma/rc.S (System Initialization)
echo "Mounting virtual filesystems..."
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t tmpfs tmpfs /run

echo "Initializing mdev..."
echo /sbin/mdev > /proc/sys/kernel/hotplug
mdev -s
```

### 2.2 Plain-Text Configuration Enforcement

SigmaOS strictly forbids binary configuration registries (like the Windows Registry or dconf). All system state is stored in human-readable, easily parsed TOML or plain text files in `/etc/sigma/`, guaranteeing it can be repaired from a recovery shell.

### 2.3 Unpatched Upstream Software

Slackware is known for shipping software exactly as the upstream developers intended, without downstream "Slackware-specific" patches. SigmaOS `sigma-pkg` enforces this: if a package requires patching to compile on SigmaOS, the patch must be submitted upstream. We do not carry downstream debt.

---

## 3. References & Standards

- Slackware Linux — `slackware.com`
- Unix Philosophy — "Write programs that do one thing and do it well."
