# Consolidated Operating System Documentation & GitHub Wiki System

## Overview & Architecture

SigmaOS consolidates all traditional operating system documentation sources—including Unix/Linux/BSD **man pages (Sections 1..9)**, **GNU Info pages (Texinfo trees)**, **/usr/share/doc offline package records**, and **distro Handbooks (Arch Wiki, FreeBSD Handbook)**—into a unified, searchable, single-source GitHub Wiki repository.

Instead of fragmented man page directories (`/usr/share/man`), Texinfo trees (`/usr/share/info`), and package documentation paths (`/usr/share/doc/*`), all OS information is normalized into standard GitHub Wiki Markdown documents and backed by native Rust engines (`SovereignDocumentationConsolidationPrGateway`).

---

## Documentation Source Matrix

| Source Format | Distro Inspiration | GitHub Wiki Page Target | Features & Capabilities |
| :--- | :--- | :--- | :--- |
| **Man Pages (Sections 1..9)** | FreeBSD, OpenBSD, Arch Linux, Debian | `wiki/Man-Pages-Catalog.md` | Troff/mdoc rendering, SYNOPSIS, DESCRIPTION, OPTIONS, SEE ALSO cross-links |
| **GNU Info Pages** | GNU Texinfo, Debian Info | `wiki/Info-Pages-Tree.md` | Hierarchical node navigation, Top/Up/Next/Prev links, interactive menus |
| **/usr/share/doc** | Debian, Fedora, Gentoo | `wiki/Package-Docs-Registry.md` | Package READMEs, changelogs, copyright notices, and configuration examples |
| **Arch Wiki & FreeBSD Handbook** | Arch Linux Wiki, FreeBSD Handbook | `wiki/Consolidated-OS-Documentation-System.md` | System administration guides, troubleshooting workflows, and step-by-step handbooks |

---

## Man Page Section Breakdown (Sections 1..9)

1. **User Commands (`Section 1`)**: Interactive programs and shell utilities (e.g. `sigma-sh(1)`).
2. **System Calls (`Section 2`)**: Kernel syscall interface (e.g. `pledge(2)`, `unveil(2)`).
3. **Library Functions (`Section 3`)**: POSIX and C library functions (e.g. `cap_rights_limit(3)`).
4. **Special Files (`Section 4`)**: Device nodes and kernel interfaces (e.g. `bpf(4)`).
5. **File Formats (`Section 5`)**: Configuration specifications (e.g. `fstab(5)`).
6. **Games (`Section 6`)**: Interactive demos and terminal games.
7. **Miscellaneous (`Section 7`)**: Macro packages and overview documentation (e.g. `cgroups(7)`).
8. **System Administration (`Section 8`)**: Daemons and package managers (e.g. `sigma-pkg(8)`).
9. **Kernel Routines (`Section 9`)**: VFS and internal kernel routines (e.g. `vfs(9)`).

---

## Synchronization Workflow

To synchronize documentation between the primary repository (`wiki/`) and the public GitHub Wiki repository (`wiki_repo/`), execute:

```bash
./scripts/sync_wiki.sh
```

The sync utility processes all Markdown files, aligns `README.md` to `Home.md`, and synchronizes core docs (`SECURITY.md`, `INSTALL.md`, `ARCHITECTURE.md`, `docs/PACKAGE_MANAGER.md`, `docs/DISTRO_COMPAT.md`, `docs/KERNEL.md`) to ensure 100% parity across all platforms.
