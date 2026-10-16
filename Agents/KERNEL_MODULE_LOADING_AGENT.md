# Kernel Module Loading Agent

| Field | Value |
|---|---|
| **Portal** | Kernel Core |
| **Maintainer Agent** | KERNEL_MODULE_LOADING_AGENT |
| **Status** | Active |
| **Engine** | `src/kernel/sigma_kmod_loader_engine.rs` |
| **Language** | Rust (primary), Nim (controller) |

---

## Table of Contents
1. [Overview](#overview)
2. [Technical Specifications](#technical-specifications)
3. [Implementation](#implementation)
4. [Standard Library Authorization](#standard-library-authorization)
5. [Configuration](#configuration)
6. [Diagnostics](#diagnostics)
7. [See Also](#see-also)

---

## 1. Overview
Manages kernel module loading via `insmod`/`modprobe`, kmod library, symbol resolution from `System.map`, module alias database (`modules.alias`), soft/hard dependencies, and module signing verification.

## 2. Technical Specifications
- Load syscall: `init_module(2)`, `finit_module(2)`
- Module search path: `/lib/modules/$(uname -r)/`
- Dependency resolution: `modules.dep` (depmod-generated)
- Signing: PKCS#7 signature verification via kernel keyring
- Blacklisting: `/etc/modprobe.d/*.conf`

## 3. Implementation
Engine: [`sigma_kmod_loader_engine.rs`](../src/kernel/sigma_kmod_loader_engine.rs)

## 4. Standard Library Authorization
`std`, `libc`, and all predefined functions are fully authorized.

## 5. Configuration
```toml
[kmod]
module_path = "/lib/modules"
enforce_signing = true
blacklist = ["nouveau", "pcspkr"]
autoload = ["btrfs", "zram", "kvm_intel"]
```

## 6. Diagnostics
```bash
lsmod                            # list loaded modules
modinfo <module>                 # module metadata
dmesg | grep -i module           # load events
sigma-diag kmod --list
```

## 7. See Also
- [POSIX_SIGNAL_HANDLING_AGENT](./POSIX_SIGNAL_HANDLING_AGENT.md)
- [LIVE_PATCH_KPATCH_AGENT](./LIVE_PATCH_KPATCH_AGENT.md)
