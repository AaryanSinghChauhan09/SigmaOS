# AI Agent System Configurability & Declarative Management in SigmaOS

## Overview

SigmaOS system configurability architecture (`src/kernel/linux_bsd_innovations.rs`, `src/customization/`, `src/distro/clear_linux.rs`, `src/sysctl.rs`) provides declarative NixOS-style configuration management, Gentoo USE-flag compile-time feature selection, Intel Clear Linux stateless config resolution, and dynamic `sysctl` kernel parameter tuning.

AI agents (such as Jules, Herdr agentic subagents, system configurators, and desktop customization daemons) must adhere to these configurability guidelines.

---

## Configurability Architecture & Paradigms

```
AI Agent Config Request → Intel Clear Linux Stateless Config Engine
                                    │
        ┌───────────────────────────┼───────────────────────────┐
        ▼                           ▼                           ▼
Declarative NixOS Config    Gentoo USE Flags Engine     Dynamic Sysctl Parameters
(`NixOsDeclarativeManager`)   (`GentooUseFlags`)          (`LinuxBsdSysctlEngine`)
```

---

## 1. Declarative NixOS-Style Configuration (`NixOsDeclarativeManager`)

AI agents managing system-wide package and service configurations utilize declarative generations:

```rust
use sigmaos::kernel::NixOsDeclarativeManager;

let mut decl_mgr = NixOsDeclarativeManager::new();

// Apply declarative system configuration profile
decl_mgr.apply_configuration(&[
    "services.nginx.enable = true;",
    "security.sudo.enable = false;",
    "networking.firewall.allowedTCPPorts = [ 80, 443 ];"
])?;

// Rollback to previous configuration generation atomically if needed
decl_mgr.rollback()?;
```

---

## 2. Gentoo-Inspired USE Flags (`GentooUseFlags`)

AI agents configuring custom package compilation or kernel feature toggles evaluate USE flags and companion dependencies:

```rust
use sigmaos::kernel::GentooUseFlags;

let mut gentoo_flags = GentooUseFlags::new();

// Toggle compile-time feature flags
gentoo_flags.set_flag("wayland", true);
gentoo_flags.add_dependency("wayland", "egl"); // wayland requires egl companion flag

// Validate dependency consistency
if gentoo_flags.check_dependencies() {
    println!("USE flags dependencies verified cleanly.");
}
```

---

## 3. Intel Clear Linux Stateless Resolution (`IntelClearLinuxStatelessEngine`)

AI agents modifying `/etc` must follow stateless configuration separation:

```rust
use sigmaos::kernel::IntelClearLinuxStatelessEngine;

let mut stateless = IntelClearLinuxStatelessEngine::new();

// Factory defaults remain pristine in /usr/share/defaults
stateless.register_default_config("/etc/sshd/sshd_config", "Port 22\nPermitRootLogin no");

// Agents write custom user overrides to /etc
stateless.set_user_override("/etc/sshd/sshd_config", "Port 2222\nPermitRootLogin no");

// Query active configuration (returns user override if present)
let resolved = stateless.resolve_config("/etc/sshd/sshd_config").unwrap();
```

---

## 4. Dynamic Sysctl Kernel Tuning (`LinuxBsdSysctlEngine`)

AI agents tuning kernel runtime parameters manage `sysctl` keys:

```rust
use sigmaos::distro::missing_distro_innovations::LinuxBsdSysctlEngine;

let mut sysctl = LinuxBsdSysctlEngine::new();

// Query kernel configuration parameter
let max_proc = sysctl.get_sysctl_value("kern.maxproc")?;

// Dynamically tune network buffer space
sysctl.set_sysctl_value("net.inet.tcp.sendspace", "65536")?;
```

---

## Directives for AI Agents

1. **Maintain Stateless Hygiene**: Never modify `/usr/share/defaults/` binaries or factory templates; place overrides under `/etc/`.
2. **Atomic Configuration Rollbacks**: Always preserve previous generation state when applying declarative system profile changes.
3. **Validate USE-Flag Dependencies**: Check `check_dependencies()` before initiating custom package builds to prevent build-time missing symbol errors.
