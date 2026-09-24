# 🏷️ SigmaOS Command, Package, Network & Driver Aliases (`alias`) Strategic Development Plan

## Executive Summary & Design Vision

Aliases in operating systems provide short, ergonomic, and context-aware shortcuts across shell environments, package management CLI utilities, network interfaces, and kernel driver hotplug subsystems. Modern shell environments and system services go far beyond simple Bash string substitutions (`alias ll='ls -la'`). They support **suffix aliases** (`alias -s txt=nvim`), **global aliases** (`alias -g G='| grep'`), **automatic typo autocorrect aliases** (`sl -> ls`), **multi-distro package manager command aliasing** (`sigpkg apt update`), **network interface aliases** (`ifconfig eth0:1`), and **kernel driver modaliases** (`pci:v00008086d000015B8* -> e1000e`).

This strategic plan establishes the architectural design, security guards, multi-distro inspirations, core subsystems, phased roadmap, and verification standards for **SigmaOS Comprehensive Alias Infrastructure**.

---

## 1. Multi-Distro & Multi-OS Alias Inspirations

### 1.1 Zsh, Fish & Bash Advanced Shell Aliases
- **Inspirations**:
  - **Zsh Suffix Aliases**: Executing a filename directly (`notes.txt`) automatically invokes the registered suffix handler (`nvim notes.txt`).
  - **Zsh Global Aliases**: Aliases expanded anywhere in a command line (`cat log.txt G err` -> `cat log.txt | grep err`).
  - **Fish Abbreviations (`abbr`)**: Expands shortcuts in-place on pressing Space or Enter so the shell history records full, explicit commands rather than obfuscated shorthands.
  - **Typo & Auto-Correction Aliases**: Automatic recognition of common terminal typos (`sl -> ls`, `dc -> cd`, `gti -> git`).
- **SigmaOS Integration**: `AliasManager`, `AliasType::Suffix`, `AliasType::Global`, and `AliasType::AutoCorrect` in `src/shell/alias_system.rs` and `src/shell/repl.rs`.

### 1.2 Multi-Distro Universal Package Manager Command Aliases (`sigpkg`)
- **Inspirations**: Universal package management ergonomics across distributions:
  - `sigpkg apt update` / `sigpkg dnf check-update` / `sigpkg pacman -Syu` / `sigpkg apk update` / `sigpkg xbps-install -S` / `sigpkg emerge --sync`.
  - Distro name aliasing (`sigpkg debian ...`, `sigpkg arch ...`, `sigpkg fedora ...`, `sigpkg alpine ...`, `sigpkg freebsd ...`).
- **SigmaOS Integration**: `ForeignPackageAliasRouter` in `src/bin/sigpkg.rs` and `src/sigpkg/universal_adapter.rs`.

### 1.3 Linux Kernel Driver Hotplug & PCI/USB Modaliases (`modprobe`)
- **Inspirations**: Linux `/lib/modules/$(uname -r)/modules.alias` and `udev` rule matching. Hardware insertion generates a bus modalias string (e.g., `pci:v000010DEd00002204...`), matching kernel driver alias patterns to auto-load the exact driver.
- **SigmaOS Integration**: `SovereignModaliasEngine` and `BsdDevdHardwareEventDispatcher` driver hotplug alias matching in `src/driver/` and `src/distro/bsd_linux_innovations.rs`.

### 1.4 BSD Network Interface Aliases (`ifconfig` / `ip`)
- **Inspirations**: FreeBSD / OpenBSD network alias configuration (`ifconfig em0 inet 192.168.1.50/24 alias`), assigning multiple IPv4/IPv6 addresses to a single physical network interface.
- **SigmaOS Integration**: `NetworkFabricEngine` interface IP alias assignment in `src/net/`.

### 1.5 Git & DevOps Command Aliases (`~/.gitconfig`)
- **Inspirations**: Shortened version control and container workflow operations (`git co -> checkout`, `git st -> status`, `git br -> branch`, `k -> kubectl`, `d -> docker`).
- **SigmaOS Integration**: `OmarchyLazyGitConfigurationEngine` and user profile alias configurations in `src/distro/omarchy.rs`.

---

## 2. Core Architectural Subsystems

```text
┌───────────────────────────────────────────────────────────────────────────┐
│                     Userland System & Shell / CLI                         │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                     Zsh / Fish / Bash Alias Subsystem                     │
│      - Standard Aliases (`alias ll='ls -la'`)                            │
│      - Global Aliases (`alias -g G='| grep'`)                            │
│      - Suffix Aliases (`alias -s txt='nvim'`)                             │
│      - Typo Autocorrect (`sl -> ls`, `dc -> cd`, `gti -> git`)            │
│      - Circular Dependency Protection (Max Depth 16 Expansion)            │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│              Multi-Distro Package Command Router (`sigpkg`)               │
│      - `sigpkg apt` -> Debian APT package adapter                        │
│      - `sigpkg dnf` -> Fedora DNF package adapter                        │
│      - `sigpkg pacman` -> Arch ALPM package adapter                      │
│      - `sigpkg apk` -> Alpine APK package adapter                          │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│             Kernel Hardware Driver Modalias & Hotplug Matching            │
│      - PCI / USB / ACPI / DeviceTree Modalias Pattern Matching            │
│      - Dynamic Driver Auto-loading (`modprobe` alias rules)               │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                 BSD Network Interface Multi-IP Aliasing                   │
│      - Multiple IPv4 / IPv6 addresses assigned to single NIC (`eth0:1`)   │
└───────────────────────────────────────────────────────────────────────────┘
```

### 2.1 Circular Dependency Guard & Max Expansion Depth
- Max recursive expansion depth: **16 levels**.
- If `a -> b` and `b -> a`, expansion terminates cleanly without stack overflow, returning the last valid expanded command token.

### 2.2 Suffix & Global Alias Resolution Rules
- When a user inputs a file path directly (e.g., `document.pdf`), the shell inspects registered suffix aliases (`pdf -> zathura`).
- Global aliases are expanded during lexing/tokenization, allowing pipes (`|`), redirections (`>`), and filter shorthands anywhere in the command string.

---

## 3. Phased Development Roadmap

### Phase 1: Shell Alias Engine & Expansion Guards (Q4 2026)
- Extend `src/shell/alias_system.rs` with Fish-style abbreviations (`abbr`) and Zsh global/suffix aliases.
- Implement circular reference prevention and 16-level expansion limits.
- Integrate alias expansion into `src/shell/repl.rs` and `src/shell/terminal_emulator.rs`.

### Phase 2: Package Manager & DevTool Ergonomic Aliasing (Q1 2027)
- Unify foreign package manager CLI command aliases (`sigpkg apt`, `sigpkg dnf`, `sigpkg pacman`, `sigpkg apk`) in `src/bin/sigpkg.rs`.
- Integrate Git and LazyGit alias configurations into Zenith desktop profiles.

### Phase 3: Kernel Driver Modaliases & Hardware Hotplug (Q2 2027)
- Deploy PCI, USB, ACPI, and DeviceTree modalias string generator in `src/driver/`.
- Connect modalias matcher with hardware hotplug event queue (`BsdDevdHardwareEventDispatcher`) for automatic driver loading.

### Phase 4: Network Interface Multi-IP Aliasing & Shell Profiling (Q3 2027+)
- Implement POSIX/BSD network interface alias configuration (`eth0:1`, `em0 alias`).
- Add shell alias performance profiling and startup benchmarking in `scripts/tech_media_benchmark_suite.sh`.

---

## 4. Verification & Testing Standards

All alias components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/shell/alias_system.rs` (user-named, suffix, global, and typo autocorrect aliases)
- `src/shell/repl.rs` & `src/shell/terminal_emulator.rs` (shell REPL alias substitution)
- `src/bin/sigpkg.rs` (multi-distro package manager CLI aliases)
- `src/driver/` & `src/distro/bsd_linux_innovations.rs` (driver modalias hotplug matching)
