# Pull Request Proposal: Core Utilities and Userland Ecosystem Parity

## Executive Summary

This Pull Request proposal introduces a comprehensive, `#![no_std]` and `alloc`-compatible Core Utilities Parity framework and PR Gateway for SigmaOS, drawing architectural inspiration from flagship Linux distributions (GNU Coreutils, util-linux, BusyBox, Alpine Linux, Gentoo, Debian, Arch Linux) and BSD operating systems (FreeBSD userland, OpenBSD coreutils, NetBSD userland, DragonFly BSD, Illumos).

The goal is to achieve 100% native parity and execution supremacy across GNU Coreutils, BSD Userland applets, BusyBox multicall binaries, util-linux system tools, findutils/diffutils, text processing utilities (grep, sed, awk), archival & multi-format compression formats (tar, gzip, bzip2, xz, zstd, lz4, 7z), process management tools (procps, psmisc), documentation & manual pagers (less, man-db, mandoc, texinfo), and authentication/privilege escalation suites (sudo, doas, shadow with Argon2id, PAM, login, getty).

---

## Targeted Tools & Subsystems

| Subsystem / Utility Suite | Distro Inspirations | Strategic Parity Implementation in SigmaOS |
| :--- | :--- | :--- |
| **GNU Coreutils & BSD Userland** | Debian, Arch Linux, FreeBSD, OpenBSD | Universal `cat`, `ls`, `cp`, `mv`, `rm`, `stat`, `df`, `wc`, `head`, `tail` with color output, extended permissions, block sizes, and inode introspection. |
| **BusyBox Multicall Applets** | Alpine Linux, Buildroot, Embedded Linux | Multicall applet router, sandboxed applet execution with memory limits, and minimal footprint analysis. |
| **util-linux System Subsystem** | Fedora, Gentoo, Arch Linux | `lsblk`, `blkid`, `lscpu`, `fdisk` GPT/MBR disk inspection, `losetup` loopback devices, `mount`/`umount` status, and `chrt`/`taskset` CPU affinity tuning. |
| **findutils & diffutils** | Debian, NetBSD, FreeBSD | `find` predicate evaluation (`-name`, `-size`, `-type`), unified/context diff generation, and `patch` application. |
| **Text Processing (grep, sed, awk)** | GNU Coreutils, OpenBSD, Illumos | Context matching (`grep -A -B -C`), streaming in-place replacement (`sed -i`), and AWK script column extraction and aggregate summation. |
| **Archival & Multi-Format Compression** | Arch Linux, Clear Linux, FreeBSD | Pax/Ustar tar header parsing, multi-format magic byte auto-detection (`.zst`, `.lz4`, `.7z`, `.gz`, `.bz2`, `.xz`), and multi-threaded Zstd compression ratio estimation. |
| **procps & psmisc Process Suite** | Ubuntu, Fedora, Illumos | EEVDF/BORE score process listing (`ps`, `top`), `pgrep`/`pkill` signal dispatching, `pstree` process hierarchy rendering, and memory telemetry (`free`). |
| **Documentation, Pager & Manual Suite** | OpenBSD, Debian, FreeBSD | `less` viewport rendering, `man-db` lookups, `mandoc` roff macro translation, and Texinfo node navigation. |
| **Auth, Privilege & Session Suite** | OpenBSD, FreeBSD, Linux PAM | `/etc/sudoers` & `/etc/doas.conf` rule evaluation, PAM multi-stack module verification, Argon2id/SHA-512 shadow password verification, and getty session initialization. |

---

## Architectural Architecture & Design

### 1. GNU Coreutils & BSD Userland Engine (`GnuCoreutilsBsdUserlandEngine`)
- **Metadata Introspection**: Standardized `CoreUtilityFileInfo` and `FilesystemUsageSpec` supporting inode number, disk block count, device IDs, and 4-digit octal permissions.
- **File Utilities**: Functional implementations for `cp` (attribute preservation), `mv`, `rm` (recursive/force options), `stat` (POSIX stat formatting), and `df` (filesystem block calculations).

### 2. BusyBox Multicall Applet Engine (`BusyBoxMulticallEngine`)
- **Multicall Applet Router**: O(1) applet lookup across 16 default applets with category tagging and memory footprint tracking.
- **Sandboxed Execution**: Isolated execution runner enforcing memory limits (`run_applet_sandboxed`).

### 3. util-linux Subsystem Engine (`UtilLinuxSubsystemEngine`)
- **Storage & Hardware Introspection**: `lsblk` block device lists, `blkid` UUID/filesystem labeling, `lscpu` topology summaries, `fdisk` GPT partition table summaries, and `losetup` loopback specifications.
- **Process & Mounting Control**: `mount` status reporting and `chrt`/`taskset` CPU scheduling policy and mask assignments.

### 4. Findutils & Diffutils Engine (`FindutilsDiffutilsEngine`)
- **Predicate Evaluation**: `find_eval_predicates` supporting name wildcards, minimum size thresholds, and directory filters.
- **Unified Diff & Patching**: Unified diff generation (`diff_compare`) and patch diff application (`apply_patch`).

### 5. Text Processing Engine (`TextProcessingEngine`)
- **Regex & Context Matching**: `grep_match_context` providing `-A -B -C` leading/trailing line context.
- **Streaming & Scripting**: `sed_in_place_stream` replacement counters and `awk_process_script` field extraction and column aggregation.

### 6. Archival & Compression Engine (`ArchivalCompressionEngine`)
- **Multi-Format Magic Detection**: Auto-detection across Zstd (`0x28B52FFD`), LZ4 (`0x04224D18`), 7z (`0x377ABCAF271C`), Gzip, Bzip2, Xz, and Tar formats.
- **Parallel Compression Tuning**: Multithreaded Zstd ratio evaluation and ustar tar header parsing.

### 7. Procps & Psmisc Engine (`ProcpsPsmiscEngine`)
- **EEVDF Process Telemetry**: Process info tracking PID, PPID, CPU %, memory %, and EEVDF scheduler score.
- **Process Hierarchy & Signaling**: `pstree` process tree generation and `pkill` signal dispatching.

### 8. Documentation & Manual Suite (`DocumentationPagerEngine`)
- **Multi-Format Documentation**: `mandoc` roff macro transpilation (`.TH`, `.SH`, `.B`), Texinfo node parsing (`@node`), and `man-db` lookups.

### 9. Auth & Privilege Escalation Suite (`AuthPrivilegeSessionEngine`)
- **Privilege Rule Evaluation**: Dual rule evaluation for `sudoers` and `doas.conf`.
- **Argon2id & PAM**: Support for Argon2id shadow hashes (`$argon2id$`) and multi-stage PAM authentication stacks.

### 10. Core Utilities PR Gateway Engine (`CoreUtilsPullRequestGatewayEngine`)
- **Pre-Populated Proposals**: Pre-populated PR proposals (#3001..#3009) covering all 10 core utility categories with PQC Dilithium-5 digital signatures, automated CI gating, and merge pipeline.

---

## Verification & Status

Verified via standalone Rust unit tests (`rustc --test --edition=2021 src/userland/coreutils_pr_gateway.rs -o build/test_coreutils_pr_gateway && ./build/test_coreutils_pr_gateway`):
- `test_gnu_coreutils_bsd_userland_engine`
- `test_busybox_multicall_engine`
- `test_util_linux_subsystem`
- `test_findutils_diffutils`
- `test_text_processing_engine`
- `test_archival_compression_engine`
- `test_procps_psmisc_engine`
- `test_documentation_pager_engine`
- `test_auth_privilege_session_engine`
- `test_coreutils_pr_gateway_workflow`

Status: **Ready for Merge**.
