# Swap & Hibernation Image Agent

| Field | Value |
|---|---|
| **Portal** | Power Management |
| **Maintainer Agent** | SWAP_HIBERNATION_IMAGE_AGENT |
| **Status** | Active |
| **Engine** | `src/kernel/sigma_swap_hibernation_engine.rs` |
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
Manages Linux swsusp hibernate-to-disk, s2disk userspace helpers, ZRAM swap, and systemd `hibernate`/`hybrid-sleep` targets. Writes and reads hibernation image from swap partition or file.

## 2. Technical Specifications
- Hibernate methods: swsusp (kernel), uswsusp (userspace), systemd-sleep
- Image compression: LZ4 / zstd
- Swap backends: partition (`/dev/sdX`), swapfile, ZRAM
- Resume from: kernel cmdline `resume=/dev/sdX resume_offset=<N>`

## 3. Implementation
Engine: [`sigma_swap_hibernation_engine.rs`](../src/kernel/sigma_swap_hibernation_engine.rs)

## 4. Standard Library Authorization
`std`, `libc`, and all predefined functions are fully authorized.

## 5. Configuration
```toml
[hibernation]
method = "swsusp"
compression = "lz4"
swap_device = "/dev/nvme0n1p3"
zram_size_mb = 8192
```

## 6. Diagnostics
```bash
systemctl hibernate
cat /sys/power/state
swapon --show
sigma-diag hibernate --verify
```

## 7. See Also
- [ACPI_POWER_RESOURCE_PARSER_AGENT](./ACPI_POWER_RESOURCE_PARSER_AGENT.md)
- [CPU_FREQUENCY_SCALING_GOVERNOR_AGENT](./CPU_FREQUENCY_SCALING_GOVERNOR_AGENT.md)
