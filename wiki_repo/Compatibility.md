# 🌐 Distribution Parity & Binary Compatibility

SigmaOS implements a multi-distro compatibility matrix enabling seamless execution of Linux and BSD binaries, scripts, and package manifests.

---

## 1. Supported Distribution Compatibility Layers

| Distribution Family | Compatibility Engine Location | Key Features & Innovations |
|---|---|---|
| **Linux Mint** | `src/compatibility/mint_linux.rs` | MintUpdate (Levels 1-5), Timeshift Btrfs/RSYNC snapshots, Cinnamon themes, mint4win dual-boot installer |
| **Fedora / RHEL** | `src/compatibility/fedora.rs` | Crypto Policies, rpm-ostree atomic trees, Flatpak sandbox manager, Cockpit web console, PipeWire routing |
| **Arch Linux** | `src/sigpkg/arch_compat.rs` | PKGBUILD compiler, AUR helper, ALPM transaction engine, ALPM hooks, pacman-contrib utilities |
| **openSUSE / Slackware** | `src/compatibility/opensuse_slackware.rs` | Zypper SAT solver, OBS Open Build Service RPM targets, YaST 1-Click Install (.ymp) parser |
| **FreeBSD / OpenBSD** | `src/compatibility/freebsd_jails.rs` & `src/security/unveil.rs` | FreeBSD Jails, Capsicum sandbox, OpenBSD pledge/unveil |
| **Debian / Ubuntu** | `src/package/debian_translator.rs` | Debian control header parsing, dpkg triggers, maintainer script translation |

---

## 2. ELF Loader & Dynamic Symbol Resolver (`src/loader/elf/elf_loader.rs`)

* **Auxiliary Vector Builder (`ElfAuxvBuilder`):** Populates ELF64 auxiliary vectors (`AT_PHDR`, `AT_PHENT`, `AT_PHNUM`, `AT_PAGESZ`, `AT_BASE`, `AT_ENTRY`, `AT_EXECFN`).
* **Glibc Symbol Resolver (`GlibcSymbolResolver`):** In-kernel symbol table resolver providing POSIX libc and Linux syscall translation without requiring external `ld.so`.
