# Distro Absorption: Rocky Linux & CentOS

> **Status**: 📋 Planned | **Source Paradigm**: Rocky Linux / CentOS (RHEL-based) | **Target Shard**: `SigmaOS Enterprise Compatibility & Build Tooling`

---

## 1. Executive Summary

Rocky Linux was created as a direct community response to Red Hat shifting CentOS from a stable RHEL-downstream to CentOS Stream. Its purpose is to provide bug-for-bug compatibility with RHEL.

SigmaOS absorbs Rocky Linux/CentOS's **binary packaging rebuild compliance** and **enterprise migration tooling**, ensuring legacy enterprise users can upgrade to SigmaOS with zero configuration loss.

---

## 2. Key Features to Absorb

### 2.1 Automated Migration Tool (`sigma-migrate`)

To lower the barrier to entry, SigmaOS implements a migration tool inspired by Rocky's `migrate2rocky` script. It scans an active Rocky/CentOS server, generates a declarative system configuration mapping, installs the SigmaOS base system on the inactive partition, and reboots cleanly.

```bash
$ sigma-migrate --source centos9
Σ [MIGRATION] Auditing system...
  Identified: 15 active network routes
  Identified: 3 custom storage volume mounts
  Generating /etc/sigma/system.toml...
  Writing boot configuration...
```

---

## 3. References & Standards

- Rocky Linux — `rockylinux.org`
- CentOS — `centos.org`
