# Core Utilities Parity & Pull Request Gateway Specification

This document details the Pull Request (PR) proposals, architectures, and sub-engines implemented in `src/userland/coreutils_pr_gateway.rs` to bring SigmaOS userland into full parity with GNU Coreutils, BSD Userland, BusyBox multicall applet suites, util-linux, findutils, diffutils, grep, sed, awk, tar, gzip, bzip2, xz, zstd, procps, psmisc, less, man-db, mandoc, texinfo, sudo, doas, shadow, PAM, login, and getty.

---

## 1. Subsystem Architecture Overview

The Core Utilities Parity Engine is structured into 10 modular sub-engines coordinated by a unified Post-Quantum Cryptography (PQC) Dilithium-5 signed Pull Request Gateway:

```
+-------------------------------------------------------------------------------+
|                 Core Utilities Pull Request Gateway Engine                    |
|          (PQC Dilithium-5 Signed, Automated CI Gating, SAT Constraints)       |
+-----------------------+---------------------------------------+---------------+
                        |                                       |
    +-------------------+-------------------+   +---------------+-------------------+
    | Core utilities Sub-Engines            |   | Authentication & Privilege        |
    | - GNU Coreutils & BSD Userland        |   | - sudo & doas rule evaluators     |
    | - BusyBox Multicall Applets           |   | - Shadow Argon2id password hash   |
    | - util-linux Storage & System         |   | - PAM multi-stack module chains   |
    | - findutils & diffutils Engine        |   | - Getty dynamic MOTD & TTY init   |
    | - Text Processing (grep, sed, awk)    |   | - Login lockout rate limiting     |
    | - Archival (tar, gzip, xz, zstd)      |   +-----------------------------------+
    | - Procps & Psmisc Process Hierarchy   |
    | - Documentation & Pagers (less, man)  |
    +---------------------------------------+
```

---

## 2. Core Sub-Engine Capabilities

### 2.1 GNU Coreutils & BSD Userland Engine (`GnuCoreutilsBsdUserlandEngine`)
- **File Management & Inspection**: Implements `cat` (with line numbering), `ls` (long/all formats), `cp` (preserving attributes), `mv`, `rm` (recursive/force), `stat` (standard and custom `%n %s %i %U %G` format string replacements), `df` (1K-blocks & human-readable `format_human_size`), `chmod` (octal & `u+x`/`g-w`/`o=r` symbolic mode conversion), `chown`, `touch`, `mkdir` (parent creation & octal permissions), `du` (standard & summary).
- **Text Slicing & Manipulation**: `sort` (numeric, reverse, unique deduplication), `tr` (character set translation & delete mode), `head`/`tail` (lines & byte offset slicing), `wc` (line, word, and byte counting), `comm`, `paste` (custom delimiters), `cut` (field-based slicing), `uniq` (repeated filter & count), `xargs` (chunked invocation generator), `base64_encode`, `sha256sum`, `timeout`, `seq`, `mktemp`, `realpath`, `expr_eval` (arithmetic & comparisons).

### 2.2 BusyBox Multicall Applet Engine (`BusyBoxMulticallEngine`)
- **Multicall Applet Dispatch**: Single-binary execution dispatching via `argv[0]` or executable symlinks (`/bin/cat -> busybox`).
- **Resource Sandboxing & Limits**: Memory limit execution controls (MB) and CPU quota enforcement (percentage).
- **Embedded Shell Evaluation**: `sh_applet_eval` evaluating multi-command scripts inside lightweight userland environments.
- **Applet Registry**: Dynamic registration of applets with category classification (`coreutils`, `findutils`, `editors`, `shells`, `init`, `util-linux`, `procps`, `archival`) and footprint byte calculation.

### 2.3 util-linux System Utilities Subsystem (`UtilLinuxSubsystemEngine`)
- **Block & Loop Devices**: MBR vs GPT partition scheme detection, `lsblk` device inventory & hierarchical JSON formatting, `blkid` UUID & file system type identification, `losetup` loop device assignment & free loop finder (`/dev/loop0`).
- **Disk & Mount Operations**: Mount options flag parser (`ro`, `noexec`, `nosuid`, `nodev`), `fdisk` geometry listing, `swapon` status breakdown, `fallocate` byte preallocation, `flock` lock acquire (exclusive & non-blocking).
- **System & Real-Time Controls**: `chrt`/`taskset` CPU affinity and real-time scheduling policy assignment, `uptime` load average summaries (1m, 5m, 15m), `lsmod` kernel module dependency inspection, `column` table alignment, `hwclock` RTC time synchronization.

### 2.4 findutils & diffutils Engine (`FindutilsDiffutilsEngine`)
- **File Discovery & Predicates**: `find_walk` path search, `find_eval_predicates` (min size, directory filter, name patterns), `find_time_predicates` (`mtime` max days & `newer_than_ts`), `find_exec_action_formatter` (`{}` pattern substitution).
- **Difference Analysis & Patching**: Unified diff generation (`diff_compare`), side-by-side formatted diffs (`diff_side_by_side`), whitespace-insensitive diffs (`diff_unified_ignore_whitespace`), context format diffs (`diff_context_format`), unified patch application (`apply_patch`), and patch dry-run validation (`validate_patch_dry_run`).

### 2.5 Text Processing Engine (`TextProcessingEngine`)
- **grep Engine**: Case-insensitive matching, extended option processing (line numbering, invert match, count-only), context lines display (`-A`, `-B`, `-C`), byte offset location calculation (`grep_byte_offsets`).
- **sed Engine**: Global string replacement, in-place stream processing, script evaluation (`s/target/replacement/`), hold space operations (`h`, `g`, `x` pattern/hold space swaps).
- **awk Engine**: Column extraction, numerical field processing & column summing, key-value field aggregation maps, `printf` field formatting (`awk_printf_field_formatting`).

### 2.6 Archival & Compression Engine (`ArchivalCompressionEngine`)
- **Format Auto-Detection**: Magic byte analysis recognizing `Zstd` (`0x28B52FFD`), `Lz4` (`0x04224D18`), `7z` (`0x377ABCAF271C`), `Gzip` (`0x1F8B`), `Bzip2` (`b"BZh"`), `Xz` (`0xFD7zXZ\0`), and `Tar`.
- **Archive Inspection & Configuration**: PAX header line parsing, ustar header extraction, tar header index building (`build_tar_header_index`), multithreaded Zstd compression ratio telemetry, Zstd dictionary configuration, archive integrity verification.

### 2.7 Procps & Psmisc Process Subsystem (`ProcpsPsmiscEngine`)
- **Process Inspection & Sorting**: `ps_list_processes` with EEVDF interactivity scores, `top_sorted` sorting by CPU %, Memory %, EEVDF score, or PID.
- **Process Signals & Search**: `pgrep_find` pattern matching, `pkill_signal` and `killall_signal` signal dispatching, `fuser_lookup_file` open file descriptor process finder.
- **System Diagnostics**: `pstree_hierarchy` tree rendering, `free_memory_summary` RAM status, `watch_command_simulation` output change tracking, `slabtop_kernel_cache_summary` slab cache inspector, `/proc/PID/statm` & `/proc/PID/io` telemetry.

### 2.8 Documentation, Pager & Manual Suite (`DocumentationPagerEngine`)
- **Less Pager**: Windowed page rendering, forward and backward text searching with offset tracking.
- **Manual Databases (`man-db` & `mandoc`)**: Section-specific man page lookups, `apropos` keyword searching, roff macro rendering (`.TH`, `.SH`, `.B`), mandoc linting warnings for missing macros, mandoc HTML rendering.
- **Texinfo Engine**: Texinfo `@node` parsing, `@menu` item navigation, `@xref` cross-reference index searching.

### 2.9 Authentication, Privilege & Session Suite (`AuthPrivilegeSessionEngine`)
- **Privilege Escalation**: `sudoers` rule evaluation with user alias resolution (`ADMINS`), `doas` rule evaluation with `doas.conf` `nopass` handling, sudo timestamp ticket validation.
- **Shadow Authentication**: SHA-512 and Argon2id (`$argon2id$`) shadow password hash verification, multi-algorithm hash checks, password expiration calculation (`verify_shadow_expiry`), password quality checks (min length, alpha/digit requirements).
- **PAM & Sessions**: PAM environment map management, service authentication (`pam_authenticate`), module verification chains (`pam_verify_module_chain`), account lockout checks (`login_account_lockout_check`), getty TTY initialization & dynamic MOTD rendering (`%h`, `%r`, `%l` specifier substitution).

---

## 3. Pull Request Proposals (#3001..#3022)

The `CoreUtilsPullRequestGatewayEngine` pre-populates 22 proposals covering all userland core utility categories:

| PR ID | Title | Category | Improved Components |
|-------|-------|----------|---------------------|
| 3001  | GNU Coreutils & BSD Userland Parity Engine | `GnuCoreutilsBsdUserland` | `cat`, `ls`, `cp`, `mv`, `rm`, `stat`, `df`, `wc` |
| 3002  | BusyBox Multicall Applet Engine & Sandboxing | `BusyBoxMulticall` | `busybox`, `sh`, `init` |
| 3003  | util-linux Storage & Hardware Introspection Engine | `UtilLinuxSubsystem` | `lsblk`, `blkid`, `lscpu`, `fdisk`, `losetup`, `mount` |
| 3004  | findutils & diffutils Unified Patching Engine | `FindutilsDiffutils` | `find`, `diff`, `patch` |
| 3005  | Text Processing Engine (grep context, sed -i, awk scripts) | `TextProcessingGrepSedAwk` | `grep`, `sed`, `awk` |
| 3006  | Archival & Multi-Format Compression Engine | `ArchivalCompressionFormats` | `tar`, `gzip`, `bzip2`, `xz`, `zstd`, `lz4`, `7z` |
| 3007  | Procps & Psmisc EEVDF Process Hierarchy Engine | `ProcpsPsmiscManagement` | `ps`, `top`, `pgrep`, `pkill`, `pstree`, `free` |
| 3008  | Documentation, Pager & Manual Suite | `DocumentationLessManDb` | `less`, `man-db`, `mandoc`, `texinfo` |
| 3009  | Auth, Privilege Escalation & Session Suite | `AuthPrivilegeSudoDoasPam` | `sudo`, `doas`, `shadow`, `PAM`, `login`, `getty` |
| 3010  | GNU Coreutils Extended File Attributes & Directory Manipulation | `GnuCoreutilsBsdUserland` | `chmod`, `chown`, `touch`, `mkdir`, `du`, `env` |
| 3011  | util-linux Swap Management, Uptime & Driver Introspection | `UtilLinuxSubsystem` | `swapon`, `swapoff`, `uptime`, `lsmod` |
| 3012  | PAM Multi-Stage Authentication & Shadow Expiration Governor | `AuthPrivilegeSudoDoasPam` | `shadow`, `PAM`, `chage`, `getty` |
| 3013  | GNU Coreutils Human-Readable Sizes & Custom Stat Formatting | `GnuCoreutilsBsdUserland` | `df`, `du`, `stat`, `sort`, `tr` |
| 3014  | findutils Time Predicates & Side-by-Side Diff Formatting | `FindutilsDiffutils` | `find`, `diff` |
| 3015  | Procps Top Dynamic Sorting & Killall Exact Signal Engine | `ProcpsPsmiscManagement` | `top`, `killall`, `fuser` |
| 3016  | Auth Suite Doas Conf & Getty Dynamic MOTD Renderer | `AuthPrivilegeSudoDoasPam` | `doas`, `getty`, `login` |
| 3017  | GNU & BSD Text Slicing, Hashing & Timeout Suite | `GnuCoreutilsBsdUserland` | `comm`, `paste`, `cut`, `uniq`, `xargs`, `base64`, `sha256sum`, `timeout`, `seq` |
| 3018  | BusyBox Standalone Shell Evaluation & Dynamic Registration | `BusyBoxMulticall` | `busybox`, `sh` |
| 3019  | util-linux Column Formatting, Fallocate & HW Clock Sync | `UtilLinuxSubsystem` | `column`, `fallocate`, `flock`, `hwclock` |
| 3020  | Procps Watch Loop Simulation, Slabtop Cache & I/O Telemetry | `ProcpsPsmiscManagement` | `watch`, `slabtop`, `procps` |
| 3021  | Documentation Apropos Search, HTML Transpilation & Texinfo Xref | `DocumentationLessManDb` | `man-db`, `mandoc`, `texinfo` |
| 3022  | Auth Privilege Timestamp Ticket Validation & Password Quality | `AuthPrivilegeSudoDoasPam` | `sudo`, `login`, `shadow`, `pam` |

---

## 4. Verification & Testing

The implementation is verified via standalone unit tests in `src/userland/coreutils_pr_gateway.rs`:

```bash
mkdir -p build
rustc --test --edition=2021 src/userland/coreutils_pr_gateway.rs -o build/test_coreutils_pr_gateway
./build/test_coreutils_pr_gateway
```

All 25 unit tests pass cleanly, validating all core utilities engines and PR submission workflows.
