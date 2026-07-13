# OSS Absorption: Linux Security Modules — SELinux, AppArmor & Landlock

> **Status**: 🔄 Active | **Source Projects**: SELinux (NSA/Red Hat), AppArmor (Canonical), Landlock (Mickaël Salaün) | **Target Shard**: `SigmaOS Mandatory Access Control Layer`

---

## 1. Executive Summary

Linux Security Modules (LSM) provide Mandatory Access Control (MAC): policies defined by the system administrator that override discretionary permissions (file owner read/write bits). Even `root` cannot bypass MAC policies.

SigmaOS implements a **unified MAC layer** (`sigma-mac`) that draws from all three LSM implementations:

- **SELinux** philosophy: Type Enforcement and multi-category labels — absorbed for server/enterprise profiles
- **AppArmor** philosophy: Path-based profiles with human-readable syntax — absorbed for desktop application profiles
- **Landlock** philosophy: Unprivileged, stackable, userspace-composable filesystem restrictions — absorbed as the default mechanism for every sandboxed shard

---

## 2. sigma-mac Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                     SIGMA MAC FRAMEWORK                          │
│                                                                  │
│  Application or Shard                                            │
│         │                                                        │
│         ▼  syscall                                               │
│  ┌──────────────────────────────────────────────────────────┐    │
│  │             sigma-mac LSM hook                           │    │
│  │                                                          │    │
│  │  1. Check sigma-landlock rules (always applied)          │    │
│  │  2. Check sigma-apparmor profile (if desktop profile)    │    │
│  │  3. Check sigma-selinux labels (if server profile)       │    │
│  │                                                          │    │
│  │  All checks must pass — DENY wins at any layer           │    │
│  └────────────────────────┬─────────────────────────────────┘    │
│                           │ PERMIT → system call proceeds         │
└───────────────────────────┼──────────────────────────────────────┘
                            │ (or EPERM returned to caller)
```

---

## 3. Key Features

### 3.1 Landlock — Unprivileged Filesystem Sandboxing

Landlock is a Linux 5.13+ LSM that allows **any process** (even unprivileged ones) to restrict its own filesystem access. SigmaOS uses Landlock as the default sandboxing primitive for all shards:

```rust
// kernel/security/landlock.rs
// SPDX-License-Identifier: MIT

pub struct LandlockPolicy {
    pub allowed_reads:  Vec<PathBuf>,
    pub allowed_writes: Vec<PathBuf>,
    pub allowed_execs:  Vec<PathBuf>,
}

impl LandlockPolicy {
    /// Apply policy to the current process — after this, restrictions are permanent
    pub fn apply(self) -> Result<()> {
        let ruleset = LandlockRuleset::new()?
            .add_rule(LandlockRule::path_beneath(
                AccessFs::READ_FILE | AccessFs::READ_DIR,
                &self.allowed_reads,
            ))?
            .add_rule(LandlockRule::path_beneath(
                AccessFs::WRITE_FILE | AccessFs::MAKE_DIR,
                &self.allowed_writes,
            ))?
            .add_rule(LandlockRule::path_beneath(
                AccessFs::EXECUTE,
                &self.allowed_execs,
            ))?;

        ruleset.restrict_self()?;  // No going back after this point
        Ok(())
    }
}

// Usage in sigma-networking shard:
pub fn apply_net_shard_policy() {
    LandlockPolicy {
        allowed_reads:  vec!["/etc/sigma/network.d/", "/sigma/store/sigma-net/"],
        allowed_writes: vec!["/run/sigma/network/"],
        allowed_execs:  vec!["/sigma/store/sigma-net/bin/"],
    }.apply().expect("Failed to apply Landlock policy");
}
```

### 3.2 AppArmor-Style Profile Language (`sigma-profile`)

SigmaOS defines human-readable per-application MAC profiles in a syntax inspired by AppArmor:

```
# /etc/sigma/mac/profiles/firefox.sp
# Sigma security profile for Firefox

profile firefox /sigma/store/*-firefox-*/bin/firefox {
    # Filesystem
    /home/user/Downloads/   rw,
    /home/user/.mozilla/    rw,
    /run/user/*/wayland-0   rw,        # Wayland socket
    /run/user/*/pipewire-0  rw,        # Audio
    /proc/cpuinfo            r,
    deny /home/user/.ssh/   r,         # Deny SSH keys
    deny /etc/shadow         r,         # Deny password file

    # Capabilities
    deny capability net_admin,          # Cannot modify network config
    deny capability sys_admin,

    # Network
    network inet stream,    # Allow TCP connections
    network inet6 stream,
    deny network raw,       # No raw sockets

    # Signals
    signal receive peer=zenith,         # Can receive signals from compositor
}
```

```bash
# Load/unload a profile
$ sigma mac load-profile /etc/sigma/mac/profiles/firefox.sp
$ sigma mac status firefox
Σ [MAC] firefox — Profile Status:
  Status     : Enforce (denials logged and blocked)
  Profile    : /etc/sigma/mac/profiles/firefox.sp
  Denials (24h): 2 (attempted access to /home/user/.ssh/)
```

### 3.3 SELinux-Inspired Type Labels (Server Profile)

For server/enterprise deployments, SigmaOS adds SELinux-style type enforcement:

```bash
# File has type label
$ sigma mac get-label /etc/sigma/sigma.toml
sigma.toml: system_u:object_r:sigma_config_t:s0

# Process runs with domain label
$ sigma mac get-domain sigma-networking
sigma-networking[1234]: system_u:system_r:sigma_net_t:s0

# Policy allows: sigma_net_t → can read sigma_config_t files
# Policy denies: user_app_t → cannot read sigma_config_t files
```

### 3.4 Unified CLI

```bash
sigma mac status                # Show MAC status for all shards
sigma mac audit                 # Show recent policy denials
sigma mac profile list          # List installed profiles
sigma mac profile load <path>   # Load an AppArmor-style profile
sigma mac profile set-mode <app> enforce|complain|disabled
sigma mac label get <path>      # Show SELinux-style label
sigma mac label set <path> <label>
sigma mac landlock show <pid>   # Show Landlock restrictions on process
```

---

## 4. Default Policy Table

| Shard/App | Landlock | AppArmor Profile | SELinux Domain |
|:----------|:---------|:-----------------|:---------------|
| sigma-networking | /etc/sigma/net, /run/sigma/net | sigma_net | sigma_net_t |
| sigma-init | / (full — PID 1) | none | kernel_t |
| firefox | /home/$USER, /Downloads | firefox.sp | user_app_t |
| sshd | /etc/ssh/, /var/log/ | sshd.sp | sshd_t |
| User apps | /home/$USER only | default_app.sp | user_app_t |

---

## 5. References & Standards

- SELinux — NSA/Red Hat (GPL-2.0) `selinuxproject.org`
- AppArmor — Canonical (GPL-2.0) `apparmor.net`
- Landlock — Mickaël Salaün (GPL-2.0) `landlock.io`
- Linux Security Modules framework — Linux kernel docs
- CIS Benchmark Level 2 — Center for Internet Security
