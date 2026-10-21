# Pull Request Proposal: Core Utilities and Userland Ecosystem Parity

## Executive Summary

This Pull Request proposal introduces a comprehensive, `#![no_std]` and `alloc`-compatible Core Utilities Parity framework and PR Gateway for SigmaOS, drawing architectural inspiration from flagship Linux distributions (GNU Coreutils, util-linux, BusyBox, Alpine Linux, Gentoo, Debian, Arch Linux) and BSD operating systems (FreeBSD userland, OpenBSD coreutils, NetBSD userland, DragonFly BSD, Illumos).

The goal is to achieve 100% native parity and execution supremacy across GNU Coreutils, BSD Userland applets, BusyBox multicall binaries, util-linux system tools, findutils/diffutils, text processing utilities (grep, sed, awk), archival & multi-format compression formats (tar, gzip, bzip2, xz, zstd, lz4, 7z), process management tools (procps, psmisc), documentation & manual pagers (less, man-db, mandoc, texinfo), and authentication/privilege escalation suites (sudo, doas, shadow with Argon2id, PAM, login, getty).

---

## Targeted Tools & Subsystems

| Subsystem / Utility Suite | Distro Inspirations | Strategic Parity Implementation in SigmaOS |
| :--- | :--- | :--- |
| **GNU Coreutils & BSD Userland** | Debian, Arch Linux, FreeBSD, OpenBSD | Universal `cat`, `ls`, `cp`, `mv`, `rm`, `stat`, `df`, `wc`, `head`, `tail`, `chmod`, `chown`, `touch`, `mkdir`, `du`, `env`, `sort`, `tr` with human-readable sizes (`df -h`, `du -h`), octal & symbolic mode conversions (`octal_to_symbolic_mode`, `symbolic_to_octal_mode`), custom format strings (`stat -c`), byte offset head/tailing (`run_head_tail_bytes`), numeric/unique sorting (`run_sort`), `comm` column comparison, `paste` delimiter joining, `cut` field slicing, `uniq` duplicate filtering, `xargs` argument batching, `base64` encoding, `sha256sum` hashing, `timeout` duration control, `seq` number sequences, `mktemp` path generation, `realpath` normalization, and `expr` arithmetic evaluation. |
| **BusyBox Multicall Applets** | Alpine Linux, Buildroot, Embedded Linux | Multicall applet router, symlink map generator (`generate_symlinks`), sandboxed applet execution with CPU and RAM quotas (`run_applet_with_resource_limits`), minimal footprint analysis, standalone shell applet evaluator (`sh_applet_eval`), applet table generator (`generate_applet_table`), and dynamic applet registration (`register_custom_applet`). |
| **util-linux System Subsystem** | Fedora, Gentoo, Arch Linux | `lsblk` JSON hierarchy rendering (`lsblk_json_hierarchy`), `blkid`, `lscpu` NUMA topology summaries, `fdisk` MBR vs. GPT disk inspection (`detect_partition_table_scheme`), `losetup` free loop device lookup (`losetup_find_free`), `mount`/`umount` flag parsing (`parse_mount_flags`), `chrt`/`taskset` CPU affinity tuning, `swapon`/`swapoff` swap inspection, `uptime` load averages, `lsmod` loaded kernel module summaries, `column` table formatting (`format_column_table`), `fallocate` file preallocation (`fallocate_preallocate`), `flock` file locking (`flock_acquire`), and `hwclock` RTC synchronization (`hwclock_sync_system_time`). |
| **findutils & diffutils** | Debian, NetBSD, FreeBSD | `find` time predicate evaluation (`find_time_predicates` for `-mtime` / `-newer`), `find` action formatting (`find_exec_action_formatter`), side-by-side diff generation (`diff_side_by_side`), unified/context diff generation (`diff_context_format`), whitespace-ignoring diffs (`diff_unified_ignore_whitespace`), and dry-run patch validation (`validate_patch_dry_run`). |
| **Text Processing (grep, sed, awk)** | GNU Coreutils, OpenBSD, Illumos | Context matching (`grep -A -B -C`), extended matching (`grep_match_extended` for `-v`, `-c`, `-n`), byte offset matching (`grep_byte_offsets`), multi-command script execution (`sed_exec_script`), streaming in-place replacement (`sed -i`), hold/pattern space operations (`sed_hold_space_operations`), AWK script column extraction & key-value group aggregation (`awk_aggregate_by_key`), and field printf formatting (`awk_printf_field_formatting`). |
| **Archival & Multi-Format Compression** | Arch Linux, Clear Linux, FreeBSD | Pax/Ustar tar header parsing (`parse_pax_header`), multi-format magic byte auto-detection (`.zst`, `.lz4`, `.7z`, `.gz`, `.bz2`, `.xz`), level configuration (`configure_compression_level`), stream integrity validation (`verify_archive_integrity`), multi-threaded Zstd compression ratio estimation, custom Zstd dictionary loading (`configure_zstd_dictionary`), PAX attribute extraction (`pax_extract_attributes`), and tar archive header index building (`build_tar_header_index`). |
| **procps & psmisc Process Suite** | Ubuntu, Fedora, Illumos | EEVDF/BORE score process listing (`ps`, `top`), dynamic sorting (`top_sorted`), `pgrep`/`pkill`/`killall` signal dispatching, `fuser` PID lookup for open files and ports (`fuser_lookup_file`), `/proc/PID/statm` memory inspection (`proc_statm_summary`), `pstree` process hierarchy rendering, memory telemetry (`free`), `watch` loop execution simulation (`watch_command_simulation`), `slabtop` kernel slab cache inspection (`slabtop_kernel_cache_summary`), and `/proc/PID/io` telemetry (`proc_io_telemetry_summary`). |
| **Documentation, Pager & Manual Suite** | OpenBSD, Debian, FreeBSD | `less` search navigation (`less_search`), `man-db` section lookup (`man_db_lookup_section` for sections 1..8), `man-db` apropos keyword search (`man_db_apropos_search`), `mandoc` syntax linting (`mandoc_lint`), roff macro translation, roff-to-HTML transpilation (`mandoc_html_render`), Texinfo menu parsing (`texinfo_navigate_menu`), node navigation, and cross-reference indexing (`texinfo_xref_index_search`). |
| **Auth, Privilege & Session Suite** | OpenBSD, FreeBSD, Linux PAM | `/etc/sudoers` & `/etc/doas.conf` rule & option evaluation (`evaluate_doas_conf`, `evaluate_sudoers_with_aliases`), PAM environment variable management (`pam_environment_manage`), PAM multi-stage module chain verification (`pam_verify_module_chain`), multi-algorithm shadow password verification ($1$ MD5, $5$ SHA-256, $6$ SHA-512, $argon2id$), shadow password expiry governor (`verify_shadow_expiry`), getty TTY session initialization (`getty_init_tty_spec`), dynamic MOTD banner rendering (`getty_render_motd`), sudo timestamp ticket validation (`sudo_timestamp_ticket_validate`), login account lockout checks (`login_account_lockout_check`), and shadow password quality checks (`shadow_password_quality_check`). |

---

## Architectural Architecture & Design

### 1. GNU Coreutils & BSD Userland Engine (`GnuCoreutilsBsdUserlandEngine`)
- **Metadata Introspection**: Standardized `CoreUtilityFileInfo` and `FilesystemUsageSpec` supporting inode number, disk block count, device IDs, and 4-digit octal permissions.
- **File Utilities & Transformations**: Functional implementations for `cp` (attribute preservation), `mv`, `rm` (recursive/force options), `stat` (POSIX & custom format strings), `df` (block & human-readable `run_df_human`), `sort` (numeric/unique/reverse), `tr` (translation/deletion), and `head`/`tail` (line & byte offsets).

### 2. BusyBox Multicall Applet Engine (`BusyBoxMulticallEngine`)
- **Multicall Applet Router & Symlinks**: O(1) applet lookup across default applets with category tagging, memory footprint tracking, and symlink map generation (`generate_symlinks`).
- **Sandboxed Execution**: Isolated execution runner enforcing memory limits and CPU quotas (`run_applet_with_resource_limits`).

### 3. util-linux Subsystem Engine (`UtilLinuxSubsystemEngine`)
- **Storage & Hardware Introspection**: `lsblk` block device lists & JSON tree formatting, `blkid` UUID/filesystem labeling, `lscpu` topology summaries with NUMA nodes, `fdisk` MBR vs GPT partition scheme detection, and `losetup` free loop device finder.
- **Process & Mounting Control**: `mount` flag parsing (`rw`, `noexec`, `nosuid`, `nodev`) and `chrt`/`taskset` CPU scheduling policy and mask assignments.

### 4. Findutils & Diffutils Engine (`FindutilsDiffutilsEngine`)
- **Predicate Evaluation**: `find_eval_predicates` and `find_time_predicates` supporting name wildcards, minimum size thresholds, directory filters, and mtime/newer-than timestamps.
- **Unified Diff & Patching**: Unified diff generation (`diff_compare`), side-by-side diff formatting (`diff_side_by_side`), patch diff application (`apply_patch`), and dry-run validation (`validate_patch_dry_run`).

### 5. Text Processing Engine (`TextProcessingEngine`)
- **Regex & Extended Context Matching**: `grep_match_context` (-A -B -C context) and `grep_match_extended` (-v, -c, -n flags).
- **Streaming & Scripting**: `sed_exec_script` multi-command execution, `sed_in_place_stream` replacement counters, and `awk_aggregate_by_key` associative aggregation.

### 6. Archival & Compression Engine (`ArchivalCompressionEngine`)
- **Multi-Format Magic Detection**: Auto-detection across Zstd (`0x28B52FFD`), LZ4 (`0x04224D18`), 7z (`0x377ABCAF271C`), Gzip, Bzip2, Xz, and Tar formats.
- **PAX & Parallel Compression**: PAX tar extended header parsing (`parse_pax_header`), compression level selection, archive integrity verification, and multithreaded Zstd ratio evaluation.

### 7. Procps & Psmisc Engine (`ProcpsPsmiscEngine`)
- **EEVDF Process Telemetry & Sorting**: Process info tracking PID, PPID, CPU %, memory %, and EEVDF scheduler score with dynamic sorting (`top_sorted`).
- **Process Hierarchy, Memory & Signaling**: `pstree` rendering, `pkill`/`killall` signal dispatching, `fuser` descriptor mapping, and `/proc/PID/statm` memory auditing.

### 8. Documentation & Manual Suite (`DocumentationPagerEngine`)
- **Multi-Format Documentation**: `mandoc` roff macro transpilation & linting (`mandoc_lint`), Texinfo menu parsing (`texinfo_navigate_menu`), `less` pattern searching (`less_search`), and `man-db` section lookups (sections 1..8).

### 9. Auth & Privilege Escalation Suite (`AuthPrivilegeSessionEngine`)
- **Privilege Rule Evaluation**: Dual rule evaluation for `sudoers` with user aliases and `doas.conf` with option flags.
- **Argon2id, PAM & Session**: Multi-algorithm shadow password verification ($1$, $5$, $6$, $argon2id$), PAM environment variable handling (`pam_environment_manage`), multi-stage PAM authentication stacks, and getty dynamic MOTD rendering (`getty_render_motd`).

### 10. Core Utilities PR Gateway Engine (`CoreUtilsPullRequestGatewayEngine`)
- **Pre-Populated Proposals**: Pre-populated PR proposals (#3001..#3022) covering all 10 core utility categories with PQC Dilithium-5 digital signatures, rejection handling, search/filtering methods, unified diff retrieval, and automated CI gating pipeline.

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
- `test_gnu_coreutils_expanded_helpers`
- `test_busybox_expanded_helpers`
- `test_util_linux_expanded_helpers`
- `test_findutils_diffutils_expanded_helpers`
- `test_text_processing_expanded_helpers`
- `test_archival_compression_expanded_helpers`
- `test_procps_psmisc_expanded_helpers`
- `test_documentation_pager_expanded_helpers`
- `test_auth_privilege_expanded_helpers`
- `test_coreutils_pr_gateway_filters`
- `test_gnu_bsd_extended_helpers`
- `test_busybox_sh_eval`
- `test_util_linux_column_flock`
- `test_procps_watch_slabtop`
- `test_auth_privilege_ticket_quality_check`

Status: **Ready for Merge**.
