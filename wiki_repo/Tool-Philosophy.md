# 🔧 Tool Philosophy — SigmaOS Sovereign Utilities

> **Zero-Dependency. Zero-Compromise. Silicon-Direct.**

Every tool in SigmaOS is a **Sovereign Shard** — an executable or linked module with:
- **No `#include <stdio.h>`**, no `#include <stdlib.h>`, no `#include <string.h>`
- **No glibc, musl, or any predefined library** — not even `printf`
- **Raw kernel I/O** via `sigma_vga_puts` / `sigma_vga_putchar` / `sigma_vga_printf` (provided by our own VGA shim)
- **Statically linked** into the SigmaOS kernel binary or `sigma-sh` monolith

This philosophy is absorbed from:
| Distro | Idea Absorbed |
|--------|--------------|
| BusyBox / Alpine | One-binary multi-call architecture |
| Plan 9 | Everything is a file; tiny, composable tools |
| GNU coreutils | Feature-complete CLI contracts |
| FreeBSD Base | Statically-linked system tools |
| Arch Linux | Minimal, transparent, source-available |
| Debian / Ubuntu | Broad hardware compat + policy-compliant packaging |

---

## 📦 Registered Sovereign Utilities (v1.1.0 — 31 Tools)

### Core Shell Builtins (in `sigma_sh.cpp`)
| Builtin | Description |
|---------|-------------|
| `echo`    | Print text to terminal |
| `cat`     | Print file from SigmaFAT32 |
| `ls`      | List root directory entries |
| `clear`   | Clear VGA screen |
| `history` | Command history (Arch-inspired ring) |
| `help`    | List all commands |
| `halt`    | CPU halt (cli; hlt) |

### Process & System (in `tools/utilities/`)
| Tool | Source | Description |
|------|--------|-------------|
| `pwd`     | `sigma_pwd.cpp`     | Print working directory |
| `uname`   | `sigma_uname.cpp`   | Display system/arch info |
| `ps`      | `sigma_ps.cpp`      | Show process table |
| `top`     | `sigma_top.cpp`     | Live process monitor |
| `kill`    | `sigma_kill.cpp`    | Send signal to process |
| `strace`  | `sigma_strace.cpp`  | Sovereign syscall ring-buffer tracer |

### File Operations
| Tool | Source | Description |
|------|--------|-------------|
| `cp`      | `sigma_cp.cpp`      | Copy file |
| `mv`      | `sigma_mv.cpp`      | Move/rename file |
| `rm`      | `sigma_rm.cpp`      | Remove file |
| `chmod`   | `sigma_chmod.cpp`   | Change file permissions |
| `wc`      | `sigma_wc.cpp`      | Word/line/byte count |
| `head`    | `sigma_head.cpp`    | First N lines of a file |
| `hexdump` | `sigma_hexdump.cpp` | Hex dump of a file |
| `tar`     | `sigma_tar.cpp`     | ustar archive list/extract |

### Text Processing
| Tool | Source | Description |
|------|--------|-------------|
| `grep`    | `sigma_grep.cpp`    | Sovereign regex line search |
| `sed`     | `sigma_sed.cpp`     | Stream editor (s/pat/rep/, /pat/d) |
| `awk`     | `sigma_awk.cpp`     | Pattern-action text processor |
| `sort`    | `sigma_sort.cpp`    | QuickSort line sorter (-r/-n/-u) |
| `uniq`    | `sigma_uniq.cpp`    | Duplicate-line filter (-c/-d/-u/-i) |

### Disk & Storage
| Tool | Source | Description |
|------|--------|-------------|
| `df`      | `sigma_df.cpp`      | Disk free space |
| `mount`   | `sigma_mount.cpp`   | Mount partition by LBA |
| `fdisk`   | `sigma_fdisk.cpp`   | MBR + GPT partition table display |
| `zfs`     | `sigma_zfs.cpp`     | Copy-on-Write pool & dataset manager |

### Network
| Tool | Source | Description |
|------|--------|-------------|
| `ifconfig` | `sigma_ifconfig.cpp` | Network interface display |
| `ping`     | `sigma_ping.cpp`     | ICMP echo (sovereign IP stack) |

### Hardware
| Tool | Source | Description |
|------|--------|-------------|
| `lspci`    | `sigma_lspci.cpp`    | PCI device enumerator |
| `dmesg`    | `sigma_dmesg.cpp`    | Kernel ring buffer |

### System Management
| Tool | Source | Description |
|------|--------|-------------|
| `overlayfs` | `sigma_overlayfs.cpp`  | Union directory merger CLI |
| `cgroup`    | `sigma_cgroup.cpp`     | Silicon resource weight governor |
| `systemctl` | `sigma_systemctl.cpp`  | Background service manager |
| `env`       | `sigma_env.cpp`        | Environment variable inspector/setter |
| `rollback`  | `tools/cli/SovereignConfigRollbackCLI.cpp` | Atomic NixOS-style configuration rollback controller |
| `schedbench`| `tools/profiler/SovereignSchedulerBench.cpp` | CFS / EDF scheduler fairness and latency testbed |
| `ecohealth` | `tools/SovereignEcosystemHealth.cpp` | Centralized CI, security, and repository telemetry dashboard |

---

## 🛠 Adding a New Tool

1. Create `tools/utilities/sigma_<name>.cpp`
2. Export `extern "C" int sigma_<name>_main(int argc, char** argv)`
3. Use **only** these kernel-provided I/O primitives:
   ```cpp
   extern "C" void sigma_vga_puts(const char* s);
   extern "C" void sigma_vga_putchar(char c);
   extern "C" void sigma_vga_printf(const char* fmt, ...);
   extern "C" u32  sigma_fat32_read(const char* name, u8* buf, u32 max);
   ```
4. Add `extern "C" int sigma_<name>_main(int argc, char** argv);` declaration in `sigma_sh.cpp`
5. Add dispatch entry: `else if (sh_streq(argv[0], "<name>")) sigma_<name>_main(argc, argv);`
6. Add help string in `builtin_help()`
7. Run `sync_branches.ps1` to push to all 19 branches

---

## ⚗️ Sovereignty Checklist

Before merging any utility, verify:
- [ ] Zero `#include` directives for standard headers
- [ ] No `printf`, `malloc`, `free`, `strlen`, `memcpy` from libc
- [ ] All string ops use sovereign `sv_*` / `sed_*` / own helpers
- [ ] Single `extern "C" int sigma_<name>_main()` export
- [ ] Registers in `sigma_sh.cpp` dispatcher
- [ ] Covered in this wiki page

---

*Last updated: Phase 4 — 31 Sovereign Utilities registered.*
