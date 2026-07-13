# Declarative Reproducibility Specification (NixOS Parity)

This specification outlines the declarative system model, immutable file structures, and content-addressed package store (`sigpkg`) that ensure reproducible builds and configurations across all SigmaOS installations.

---

## ❄️ Declarative System Configuration (`sigma.toml`)

All system states, cgroups, network interfaces, and installed packages are declared in `/etc/sigma.toml`. Booting the system parses this configuration and builds the transient environment in memory.

```toml
# /etc/sigma.toml
[system]
hostname = "sovereign-node"
profile = "desktop"
kernel_channel = "stable"

[packages]
installed = [
    "zenith-desktop-core",
    "sigma-shell",
    "sigma-browser"
]

[network]
interfaces.eth0 = { dhcp = true }
firewall.ingress_policy = "drop"
```

---

## 📦 Content-Addressed Store (`sigpkg`)

`sigpkg` uses a content-addressed storage (CAS) model for all package versions, preventing dependency conflicts or state corruption. Packages are identified by the SHA-256 hash of their contents.

```
/sigma/store/
├── hash-package-name-version/
│   ├── bin/
│   ├── lib/
│   └── share/
```

When a package is installed, it is placed into a hash-specific folder in `/sigma/store`. Symbolic links are then created in `/usr/bin` or `/usr/lib` pointing to the exact content-addressed files.

---

## 🔄 Immutable A/B Upgrades & OSTree-Style Rollbacks

The boot partition is set as **Read-Only** during normal execution. System updates are written to a secondary partition, which is verified before swapping the boot pointer.

```
                    [Active System Partition (A) - Read-Only]
                                       │
                      Update triggered via sigpkg
                                       │
                                       ▼
                 [Write update to Standby Partition (B)]
                                       │
                                       ▼
                  [Validate PQC signature on boot image]
                                       │
                                       ▼
                  [Configure bootloader to boot from B]
                   /                               \
                  /                                 \
             (Success)                            (Failure)
                /                                     \
               ▼                                       ▼
[Commit B as new Active]                [Auto-rollback to Partition A]
```

### Self-Healing & Automatic Recovery

If the kernel fails to boot or a critical service crashes within 60 seconds of a system update, the bootloader automatically reverts the boot pointer to the previous working partition (Partition A), ensuring a 100% recovery rate from corrupted updates.
