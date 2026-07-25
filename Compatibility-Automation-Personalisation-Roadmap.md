# SigmaOS — Compatibility, Automation, Customisation & Personalisation Roadmap

Concrete, file-level engineering plan for the four dimensions not yet
covered in previous roadmaps: app/platform compatibility, automation
pipelines, system customisation, and per-user personalisation.

---

## Current State

| Dimension | State | Key gap |
|-----------|-------|---------|
| **App compatibility** | Linux ELF compat exists (`sigma_linux_compat.cpp`), Win32 loader skeleton | No live execution — VMM not wired |
| **Platform compatibility** | x86-64 QEMU only | No real hardware, no ARM64, no RISC-V |
| **Automation** | `sigma_automation.sh` real (backup/update/wiki-sync) | No scheduled tasks, no event-triggered automation |
| **Customisation** | Config.sigma, `.sigma_profile` template | No VFS read of profile, no live apply |
| **Personalisation** | Profiles (desktop/minimal) + 64-alias table | No per-user DID identity, no profession-aware defaults |

---

## Part 1 — Compatibility Roadmap

### C1 — Linux Application Compatibility

**Current:** `runtime/containers/sigma_linux_compat.cpp` — ELF64 parser +
15-syscall translator. VMM section mapping is `s.mem = NULL` (TODO).

#### Stage L1 — Static Linux binaries (no libc dependency)

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Wire VMM to ELF segment map | `runtime/containers/sigma_linux_compat.cpp` | `tools-dev` | Call `sigma_vmm_map_region(va, size, perms)` per PT_LOAD |
| ELF base relocation | `runtime/containers/sigma_linux_compat.cpp` | `tools-dev` | Apply `.rela.dyn` if load address ≠ preferred |
| Expand syscall table to 50 calls | `runtime/containers/sigma_linux_compat.cpp` | `tools-dev` | Add: `stat`, `fstat`, `lseek`, `pipe`, `dup2`, `clone`, `wait4`, `execve`, `mprotect`, `getcwd`, `chdir`, `mkdir`, `rmdir`, `unlink`, `rename`, `readdir`, `socket`, `connect`, `send`, `recv`, `nanosleep`, `futex`, `set_robust_list`, `get_tid_address` |
| vDSO shim (`clock_gettime`, `gettimeofday`) | `runtime/containers/sigma_linux_compat.cpp` | `tools-dev` | Map sigma_tsc → CLOCK_MONOTONIC, CLOCK_REALTIME |
| `/proc/self/maps` stub | `kernel/vfs/sigma_procfs.cpp` | `kernel-exp` | Needed by glibc startup |
| `/etc/hostname`, `/etc/os-release` stubs | `kernel/vfs/sigma_procfs.cpp` | `kernel-exp` | Many apps read these at startup |
| `uname()` returns "SigmaOS 6.0" | `runtime/containers/sigma_linux_compat.cpp` | `tools-dev` | Already stubbed — verify string length |

**Exit test:** `sigma-compat exec /bin/ls` (statically compiled) lists `/` in QEMU.

#### Stage L2 — Dynamic Linux binaries (glibc/musl)

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-ldso (ELF dynamic linker) | `userland/ldso/sigma_ldso.cpp` | `tools-dev` | Load `ld-linux.so.2` or sigma-provided musl-libc |
| PLT/GOT relocation | `userland/ldso/sigma_ldso.cpp` | `tools-dev` | Resolve `R_X86_64_JUMP_SLOT`, `R_X86_64_GLOB_DAT` |
| musl-libc static bundle | `userland/ldso/sigma_musl.cpp` | `tools-dev` | Ship musl as sigma-pkg — covers 95% of CLI apps |
| `LD_PRELOAD` equivalent | `userland/ldso/sigma_ldso.cpp` | `tools-dev` | `SIGMA_PRELOAD=path.so sigma-compat exec app` |
| Thread-local storage (TLS) | `runtime/containers/sigma_linux_compat.cpp` | `tools-dev` | `arch_prctl(ARCH_SET_FS)` already stubbed — connect to TEB |
| `SIGMA_COMPAT_STRACE=1` debug mode | `runtime/containers/sigma_linux_compat.cpp` | `tools-dev` | Print every translated syscall to serial |

**Exit test:** `sigma-compat exec python3 -c "print('hello')"` (dynamically linked) works.

#### Stage L3 — Container isolation (sigma-pod + Linux compat)

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Linux ELF inside sigma-pod namespace | `userland/tools/sigma_pod_cli.cpp` | `release/cloud` | `sigma-pod run-native --compat-linux /usr/bin/nginx` |
| Linux filesystem layout inside pod | `kernel/core/process/sigma_namespace.cpp` | `release/cloud` | Mount `/proc`, `/dev`, `/tmp` in pod mount namespace |
| Seccomp-BPF filter for Linux compat | `kernel/security/sigma_seccomp.cpp` | `release/cloud` | Restrict to mapped syscalls only |
| sigma-linux-compat CI test | `tests/compat/linux/run_linux_compat.sh` | `tools-dev` | Run 5 static binaries: ls, echo, cat, grep, wc |

### C2 — Windows Application Compatibility (sigma-wine)

**Current:** PE loader parses headers, NT API has 20 functions, kernel32 console I/O done.
VMM integration pending.

| Stage | Target app | Blocker | Branch |
|-------|-----------|---------|--------|
| W1 | `hello.exe` (static Win32) | VMM section mapping | `tools-dev` |
| W2 | Python 3 CLI for Windows | sigma-msvcrt printf/malloc | `tools-dev` |
| W2 | Git for Windows CLI | Winsock2 TCP | `tools-dev` |
| W3 | Notepad.exe | sigma-user32 message pump | `release/standalone` |
| W4 | VSCode (Electron) | D3D11→Vulkan DXVK | `release/standalone` |
| W5 | DX11 Steam game | vkd3d-proton D3D12 | `release/standalone` |

**Next W1 tasks:**
| Task | File | Branch | Detail |
|------|------|--------|--------|
| VMM region alloc for PE sections | `runtime/compat/win32/sigma_pe_loader.cpp` | `tools-dev` | `sigma_vmm_map_region(va, memsz, perms)` per section |
| Base relocation walk | `runtime/compat/win32/sigma_pe_loader.cpp` | `tools-dev` | `IMAGE_BASE_RELOCATION` chain → patch absolute addresses |
| IAT stub patching | `runtime/compat/win32/sigma_pe_loader.cpp` | `tools-dev` | Fill IAT entries with sigma-ntdll function pointers |
| sigma-kernel32 file I/O | `runtime/compat/win32/kernel32/sigma_kernel32_file.cpp` | `tools-dev` | `CreateFile`, `ReadFile`, `WriteFile` via NtXxx |
| sigma-msvcrt printf | `runtime/compat/win32/crt/sigma_msvcrt.cpp` | `tools-dev` | Format string → `NtWriteFile(stdout)` |
| sigma-wine W1 CI test | `tests/compat/win32/run_hello_exe.sh` | `tools-dev` | `sigma-wine hello.exe` → "Hello, SigmaOS!" |

### C3 — POSIX Compatibility

**Current:** `userland/compat/POSIXShim.cpp` — open/read/write/close/fork/execve stubs.

| Task | File | Branch | Detail |
|------|------|--------|--------|
| POSIX open() → VFS full path | `userland/compat/POSIXShim.cpp` | `tools-dev` | Normalise path, call `sigma_sys_open` |
| POSIX fork() + execve() | `userland/compat/POSIXShim.cpp` | `tools-dev` | `sigma_sys_fork` + `sigma_sys_execve` |
| POSIX signal handling | `userland/compat/POSIXShim.cpp` | `tools-dev` | `SIGINT/SIGTERM/SIGSEGV` → sigma signal primitives |
| POSIX mmap() → sigma-vmm | `userland/compat/POSIXShim.cpp` | `tools-dev` | MAP_ANON, MAP_FILE via `sigma_sys_mmap` |
| POSIX pthreads stub | `userland/compat/sigma_pthreads.cpp` | `tools-dev` | `pthread_create/join/mutex` → sigma thread primitives |
| Optional install (`sigma-posix-compat` pkg) | `sigma_pkg_registry/recipes/sigma-posix-compat.recipe` | `tools-dev` | Not shipped by default; opt-in install |

### C4 — File Format Compatibility

| Format | App | Task | Branch |
|--------|-----|------|--------|
| Tally XML import/export | sigma-accounts | `sigma_accounts_import_tally()` body | `release/standalone` |
| Excel .xlsx read | sigma-accounts | libxlsxwriter port | `release/standalone` |
| PDF generation | sigma-ca, sigma-health | Cairo/Poppler port or sovereign PDF writer | `release/standalone` |
| DICOM (X-ray images) | sigma-health, sigma-dental | USB DICOM driver + FHIR ImagingStudy | `release/standalone` |
| GST JSON schema v1.4 | sigma-accounts, sigma-ca | Auto-update from GSTN schema API | `release/standalone` |
| FHIR R4 bundles | sigma-health | Validate against ABDM profile | `release/standalone` |
| ODF / DOCX render | sigma-legal | LibreOffice core port or own renderer | `release/standalone` |
| HL7 v2 → FHIR R4 | sigma-health | Migration tool for hospital data | `release/dual-boot` |

### C5 — Package Format Compatibility

| Format | Task | Branch | Detail |
|--------|------|--------|--------|
| `.deb` install via sigma-compat | `userland/sigma-pkg/sigma_pkg_deb.cpp` | `tools-dev` | Extract `.deb` into sigma-posix-compat prefix |
| `.rpm` install via sigma-compat | `userland/sigma-pkg/sigma_pkg_rpm.cpp` | `tools-dev` | RPM cpio extraction |
| Flatpak run via sigma-pod | `userland/sigma-pkg/sigma_pkg_flatpak.cpp` | `release/cloud` | sigma-pod + Linux compat → run Flatpak |
| AppImage via sigma-compat | `userland/sigma-pkg/sigma_pkg_appimage.cpp` | `tools-dev` | Mount squashfs, run ELF inside sigma-compat |
| `.spkg` as primary format | `sigma_pkg_registry/` | all | Native sigma format — always preferred |


---

## Part 2 — Automation Roadmap

### A1 — System Automation Engine (sigma_automation.sh expansion)

**Current:** `scripts/sigma_automation.sh` — backup, update, recovery-check, meta-check, wiki-sync, quality-check all real.

#### New automation commands

```bash
sigma_automation.sh release          # tag + sign + publish release ISO
sigma_automation.sh qemu-test        # boot ISO in QEMU, assert prompt
sigma_automation.sh perf-bench       # run all benchmarks, store results
sigma_automation.sh sign-release     # ML-DSA-87 sign ISO + packages
sigma_automation.sh lint             # clang-tidy + markdownlint
sigma_automation.sh gen-changelog    # git log → CHANGELOG.md
sigma_automation.sh sbom             # generate CycloneDX SBOM
sigma_automation.sh india-sync       # sync MSP/HSN/ICD-10 offline data
sigma_automation.sh fleet-sync       # push policy to all sigma-fleet devices
sigma_automation.sh clean            # remove build artefacts + tmp files
```

| Task | File | Detail |
|------|------|--------|
| `cmd_qemu_test()` | `scripts/sigma_automation.sh` | Boot ISO, assert "sigma-login" in output within 30 s |
| `cmd_perf_bench()` | `scripts/sigma_automation.sh` | Run bench_sched + bench_pqc, write JSON to `.sigma/bench/` |
| `cmd_sign_release()` | `scripts/sigma_automation.sh` | Call `pqc_sign(iso_sha256, dilithium_sk)` via sigma-pqc CLI |
| `cmd_india_sync()` | `scripts/sigma_automation.sh` | Fetch latest MSP/HSN/ICD-10 from government APIs, update offline SQLite |
| `cmd_sbom()` | `scripts/sigma_automation.sh` | `cyclonedx-cli generate` → `sigma_sbom_$(date).json` |
| `cmd_clean()` | `scripts/sigma_automation.sh` | `rm -rf build/ dist/ .sigma/tmp/` |

### A2 — Scheduled Task Engine (sigma-cron)

**New file:** `userland/daemons/sigma_cron.cpp`

```bash
# /sigma/etc/sigma-cron.conf syntax (similar to crontab):
@daily    sigma_automation.sh india-sync          # refresh offline India data
@weekly   sigma_automation.sh perf-bench          # benchmark trend
@monthly  sigma_automation.sh gen-changelog        # update CHANGELOG
0 2 * * * sigma_automation.sh backup              # nightly source backup
0 6 * * * sigma-pkg update                        # check for package updates
@reboot   sigma_automation.sh quality-check        # quality gate on every boot
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-cron daemon | `userland/daemons/sigma_cron.cpp` | `tools-dev` | Parse crontab, fire at wall-clock time via APIC timer |
| sigma-cron CLI | `userland/tools/sigma_cron_cli.cpp` | `tools-dev` | `sigma-cron list/add/remove/run <job>` |
| sigma-cron sigma-bus integration | `userland/daemons/sigma_cron.cpp` | `tools-dev` | Jobs published to sigma-bus as events |
| sigma-cron audit logging | `userland/daemons/sigma_cron.cpp` | `tools-dev` | Every job run logged to sigma-audit |
| Persistent crontab in SigmaFS | `userland/daemons/sigma_cron.cpp` | `fs-dev` | Store at `/sigma/etc/sigma-cron.conf` |

### A3 — Event-Driven Automation (sigma-hook)

**New file:** `userland/daemons/sigma_hook.cpp`

```bash
# Event hooks — fire on system events:
sigma-hook add --on "package.installed" --run "sigma-sec verify --pkg %PKG"
sigma-hook add --on "network.connected" --run "sigma_automation.sh india-sync"
sigma-hook add --on "boot.success"      --run "sigma_automation.sh quality-check"
sigma-hook add --on "profession.ca"     --run "sigma-ca dashboard --startup"
sigma-hook add --on "file.changed:/etc/sigma-policy" --run "sigma-sec mac reload"
sigma-hook add --on "sensor.temperature.high" --run "sigma-rt jitter show"
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-hook daemon | `userland/daemons/sigma_hook.cpp` | `tools-dev` | Subscribe to sigma-bus topics, execute command on match |
| Hook definition format | `/sigma/etc/sigma-hooks.conf` | `tools-dev` | TOML: `[[hook]] event = "..." command = "..."` |
| sigma-hook CLI | `userland/tools/sigma_hook_cli.cpp` | `tools-dev` | `sigma-hook list/add/remove/test <event>` |
| Profession auto-launch hooks | `userland/daemons/sigma_hook.cpp` | `release/standalone` | On login, if DID credential = CA → launch sigma-ca |
| Security event hooks | `userland/daemons/sigma_hook.cpp` | all | On sigma-ids alert → run sigma-sec audit |

### A4 — CI/CD Pipeline Automation

**Current:** `.github/workflows/` has 16 workflow files. Several tests are `echo` stubs.

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Replace QEMU echo stubs | `.github/workflows/sigma_qemu.yml` | all | Real `qemu-system-x86_64 -cdrom SigmaOS.iso -serial stdio` |
| Nightly benchmark CI | `.github/workflows/sigma_ci.yml` | `performance-optimized` | Scheduled `@daily` benchmark run + trend diff |
| India Stack sandbox CI | `.github/workflows/sigma_ci.yml` | `release/standalone` | Weekly GSTN/ABDM sandbox integration tests |
| Physical hardware CI | `.github/workflows/sigma_ci.yml` | `prepare-sigmaos-launch` | Self-hosted RPi4 + x86 runners |
| sigma-wine W1 CI | `.github/workflows/sigma_wine_ci.yml` | `tools-dev` | `sigma-wine hello.exe` passes in QEMU |
| Reproducible build diff | `.github/workflows/sigma_ci.yml` | all | Two builds → identical SHA256 |
| Auto-PR labelling | `.github/workflows/` | all | Label by subsystem based on files changed |
| Release candidate pipeline | `.github/workflows/sigma_release.yml` | `prepare-sigmaos-launch` | tag → ISO → sign → upload → notify |

### A5 — sigma_git_sync.sh Enhancements

**Current:** `scripts/sigma_git_sync.sh` — commit/push + wiki mirror. Real.

```bash
sigma_git_sync.sh                  # current: commit + push
sigma_git_sync.sh --dry-run        # preview without push
sigma_git_sync.sh --wiki-only      # only update wiki_repo/
sigma_git_sync.sh --verify-sig     # verify last commit's Dilithium sig
sigma_git_sync.sh --sync-all-branches  # push all local branches
sigma_git_sync.sh --release v16.0  # tag + sign + push release
```

| Task | File | Detail |
|------|------|--------|
| `--verify-sig` flag | `scripts/sigma_git_sync.sh` | Run `sigma-pqc verify` on HEAD commit signature |
| `--sync-all-branches` flag | `scripts/sigma_git_sync.sh` | Push all local branches that are ahead of origin |
| `--release <version>` flag | `scripts/sigma_git_sync.sh` | `git tag -s v$VERSION && git push --tags` |
| Signed commits by default | `scripts/sigma_git_sync.sh` | `git commit -S` using Dilithium3 key if available |


---

## Part 3 — Customisation Roadmap

### K1 — Config.sigma (declarative system configuration)

**Current:** `Config.sigma` exists as a file. Parser in `userland/ignite/sigma_ignite.cpp`.
VFS read not yet wired.

#### Config.sigma format (TOML + Dilithium attestation)

```toml
# /sigma/etc/Config.sigma
[system]
hostname    = "sigma-node-01"
timezone    = "Asia/Kolkata"
locale      = "hi_IN.UTF-8"

[boot]
profile     = "desktop"          # desktop | minimal | cloud | forensic | gaming
safe_mode   = false
ab_slot     = "A"

[network]
hostname    = "sigma-local"
dns         = ["1.1.1.1", "8.8.8.8"]
ntp         = "time.google.com"

[desktop]
theme       = "zenith-dark"
font_size   = 12
scale       = 1.0
layout      = "master-stack"
gap_inner   = 4
gap_outer   = 8
wallpaper   = "/sigma/data/wallpapers/india-satellite.jpg"

[security]
mac_policy  = "/sigma/etc/sigma-policy/default.sigma-policy"
audit_mode  = true
pqc_level   = "ml-kem-1024"     # ml-kem-512 | ml-kem-768 | ml-kem-1024

[india]
gst_profile = "regular"         # regular | composition | unregistered
state_code  = "27"              # 27 = Maharashtra
preferred_language = "hi"
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| VFS read Config.sigma at boot | `userland/ignite/sigma_ignite.cpp` | `kernel-exp` | `vfs_open("/sigma/etc/Config.sigma")` after VFS init |
| TOML parser (zero-dependency) | `userland/ignite/sigma_toml.cpp` | `tools-dev` | Lightweight TOML parser, no external deps |
| Dilithium3 signature verification | `userland/ignite/sigma_ignite.cpp` | `tools-dev` | Verify `Config.sigma.sig` before applying |
| Apply network config at boot | `userland/ignite/sigma_ignite.cpp` | `kernel-exp` | Set hostname, DNS, NTP from config |
| Apply desktop config to Zenith | `zenith_desktop/zenith_unified_init.cpp` | `release/standalone` | Set theme, gaps, layout from config |
| Apply security policy | `userland/ignite/sigma_ignite.cpp` | all | Load `.sigma-policy` from config path |
| `sigma-config get/set <key> <val>` CLI | `userland/tools/sigma_config_cli.cpp` | `tools-dev` | Read/write Config.sigma keys |
| `sigma-config apply` | `userland/tools/sigma_config_cli.cpp` | `tools-dev` | Apply all config changes without reboot |
| `sigma-config diff` | `userland/tools/sigma_config_cli.cpp` | `tools-dev` | Show what changed since last boot |
| `sigma-config rollback` | `userland/tools/sigma_config_cli.cpp` | `tools-dev` | Revert to previous Config.sigma |

### K2 — Profile System (boot profiles)

**Current:** `init/sigma_profile_selector.cpp` — Minimal/Desktop/Cloud selector. Partial.

#### Profile definitions

```
developer   — debug symbols, relaxed MAC, verbose logging, sigma-gdb
desktop     — Zenith GUI, full profession apps, sigma-ai
minimal     — no GUI, 8 core commands, 64 MB RAM
cloud       — container-first, no GUI, sigma-pod + sigma-fleet
forensic    — WORM audit, write-block mounts, read-only root
gaming      — Vulkan performance mode, no audit overhead
embedded    — sigma-ultra, 16 MB RAM, USSD mode
rtos        — EDF scheduler, no non-RT services
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Profile selector boot param | `init/sigma_profile_selector.cpp` | all | `sigma.profile=cloud` on kernel cmdline |
| Per-profile service set | `init/sigma_profile_selector.cpp` | all | Only start services listed in profile manifest |
| Per-profile cgroup limits | `init/sigma_profile_selector.cpp` | `release/cloud` | Cloud profile: enforce container cgroup limits |
| Hot-swap profiles (no reboot) | `userland/tools/sigma_cli.cpp` | `tools-dev` | `sigma-cli profile use gaming` → apply changes live |
| Profile import/export | `userland/tools/sigma_cli.cpp` | `tools-dev` | Share profiles as signed `.sigma-profile` bundles |
| Profession-aware auto-profile | `userland/installer/sigma_oobe.cpp` | `release/standalone` | DID credential = CA → auto-select desktop + sigma-ca |

### K3 — Zenith Theme Customisation

**Current:** `zenith_desktop/theme/sigma_theme_engine.cpp` — partial. Light/dark theme stub.

```bash
sigma-zenith theme list                    # installed themes
sigma-zenith theme set zenith-dark         # apply built-in theme
sigma-zenith theme set /path/to/mytheme.sigma
sigma-zenith theme create mytheme          # skeleton from current settings
sigma-zenith theme edit mytheme            # open in text editor
sigma-zenith theme export mytheme.sigma    # export as signed bundle
sigma-zenith theme import theme.sigma      # install from file/URL
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Theme file format (TOML) | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | Colors, fonts, gaps, border radius, animations |
| Built-in themes | `zenith_desktop/themes/` | `release/standalone` | zenith-dark, zenith-light, zenith-india, zenith-high-contrast, zenith-mono |
| Live theme apply (no restart) | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | IPC message → compositor reloads theme in < 200 ms |
| Theme hot-reload on file change | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | inotify-equiv watch on `~/.sigma/theme/` |
| Per-app theme override | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | Different theme per window class |
| Indian festive themes | `zenith_desktop/themes/` | `release/standalone` | Diwali, Holi, Independence Day themes bundled |
| WCAG 2.2 AA contrast validation | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | Warn if foreground/background contrast < 4.5:1 |

### K4 — Kernel & Syscall Customisation

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-sysctl (runtime kernel params) | `userland/tools/sigma_sysctl_cli.cpp` | `kernel-exp` | `sigma-sysctl net.tcp.congestion=cubic` |
| `/sigma/proc/sys/` virtual fs | `kernel/vfs/sigma_sysctl.cpp` | `kernel-exp` | Read/write kernel parameters via VFS |
| Kernel module load/unload | `userland/tools/sigma_drv_cli.cpp` | `drivers-dev` | `sigma-drv load my_driver.sdf` at runtime |
| Custom syscall numbers (sigma-profile) | `kernel/core/sigma_syscall_dispatch.cpp` | `kernel-exp` | Different syscall numbers per OS profile |
| Boot parameter passthrough | `sigma-boot/sigma_boot.c` | `kernel-exp` | `sigma.debug=1 sigma.nopqc=0` on kernel cmdline |
| Live kpatch apply | `kernel/kpatch/sigma_kpatch.cpp` | `performance-optimized` | `sigma-perf kpatch apply CVE-2026-XXXX.kpatch` |

---

## Part 4 — Personalisation Roadmap

### P1 — ~/.sigma_profile (per-user config)

**Current:** `~/.sigma_profile` template in `docs/examples/sigma_profile.example`.
`sigma_profile_engine.cpp` reads it partially. VFS load stub.

#### ~/.sigma_profile format

```toml
# ~/.sigma_profile
[identity]
did             = "did:sigma:abc123..."
name            = "Aaryan Singh Chauhan"
preferred_lang  = "hi"
profession      = "software_engineer"   # or: ca, doctor, farmer, advocate, teacher

[desktop]
theme           = "zenith-dark"
layout          = "bsp"
gap_inner       = 6
gap_outer       = 12
font_family     = "Noto Sans Devanagari"
font_size       = 13
scale           = 1.0
wallpaper       = "~/Pictures/sigma-wallpaper.jpg"

[shell]
prompt          = "σ \w → "
history_size    = 2048
auto_suggest    = true
syntax_highlight= true
default_editor  = "micro"

[aliases]
ll  = "sigma-ls -lah"
gs  = "git status"
gst = "sigma-gst gstr3b"    # profession shortcut

[apps]
startup         = ["sigma-ca", "sigma-health"]   # launch on login
default_browser = "sigma-browser"
default_editor  = "micro"
default_term    = "sigma-term"

[notifications]
gst_reminder    = true   # remind 7 days before GSTR filing due
pmfby_deadline  = true   # insurance enrollment reminder
court_hearing   = true   # sigma-legal hearing reminders
mgnregs_attend  = false
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| VFS read `~/.sigma_profile` at login | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `kernel-exp` | `vfs_open("/home/<uid>/.sigma_profile")` |
| TOML parser for profile | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `tools-dev` | Reuse `sigma_toml.cpp` from Config.sigma |
| Apply desktop keys to Zenith | `zenith_desktop/zenith_unified_init.cpp` | `release/standalone` | theme/layout/gaps/font from profile |
| Apply shell keys to sigma-sh | `userland/shell/sigma_shell.cpp` | `tools-dev` | history_size, prompt, aliases |
| Profession auto-configure | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `release/standalone` | `profession=ca` → load sigma-ca, set GST aliases |
| Notification subscriptions | `userland/daemons/sigma_notify.cpp` | `release/standalone` | Subscribe to GST/PMFBY/court events per profile |
| `sigma-profile edit` TUI | `userland/tools/sigma_profile_cli.cpp` | `tools-dev` | Guided editor for `~/.sigma_profile` |
| `sigma-profile import <did>` | `userland/tools/sigma_profile_cli.cpp` | `release/standalone` | Fetch profile from DID document |
| Hot-reload profile (no reboot) | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `release/standalone` | Watch file changes → apply live |
| Profile backup/restore | `scripts/sigma_automation.sh` | `tools-dev` | Include `~/.sigma_profile` in backup |

### P2 — DID-Based Personalisation

Every user's identity is a cryptographically-verifiable DID document.
The OS reads profession + preferences from the DID and configures itself.

```bash
# First boot: scan QR on sigma-ultra phone
sigma-dm login --did-scan   # camera → QR → DID auth → desktop opens

# DID document carries:
#   profession = "ca"        → loads sigma-ca on startup
#   state      = "MH"        → sets GST state = Maharashtra
#   language   = "hi"        → UI + screen reader in Hindi
#   theme      = "zenith-dark"
#   bar_council_number = "MH/12345" → verified advocate badge
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| DID document profession field | `security/SovereignDID.cpp` | `release/standalone` | Read `profession`, `state`, `preferred_lang` |
| Auto-configure from DID on login | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `release/standalone` | Merge DID preferences into `~/.sigma_profile` |
| Professional credential badge | `zenith_desktop/taskbar/sigma_systray.cpp` | `release/standalone` | Show "CA ✓" / "Dr ✓" badge in taskbar |
| Verified professional QR | `userland/tools/sigma_trust_cli.cpp` | `release/standalone` | `sigma-trust did qr` → printable verified QR |
| Multi-device DID sync | `security/SovereignDID.cpp` | `release/standalone` | Same DID on phone + laptop, preferences roam |

### P3 — App-Level Personalisation

| App | Personalisation feature | Task | Branch |
|-----|------------------------|------|--------|
| sigma-ca | Multi-client dashboard order | Save client sort preference to `~/.sigma_profile` | `release/standalone` |
| sigma-agri | Crop favourites + MSP alerts | `crops.favourites` in profile | `release/standalone` |
| sigma-health | Patient list filters | Save view preferences per doctor | `release/standalone` |
| sigma-legal | Case type defaults | `case.default_court` in profile | `release/standalone` |
| sigma-pos | UPI QR amount presets | Quick amounts `[100, 500, 1000]` | `release/standalone` |
| sigma-edu | Class/subject defaults | Teacher-specific defaults | `release/standalone` |
| sigma-zenith | Per-app window size rules | `[window.rules]` in profile: `class=sigma-ca size=1200x900` | `release/standalone` |
| sigma-sh | Profession-specific aliases | CA gets `gst=sigma-gst`, doctor gets `rx=sigma-health prescribe` | `release/standalone` |

### P4 — Personalisation for Rural & Low-Resource Users

| Feature | Target | Task | Branch |
|---------|--------|------|--------|
| Default language from SIM card | sigma-ultra | Detect SIM PLMN → select language | `release/mobile` |
| Offline preference sync (CRDT) | sigma-ultra | Merge preferences without internet | `release/distributed` |
| Feature phone simplified profile | sigma-ultra | Only 5 settings: language/UPI/crops/health/scheme | `release/mobile` |
| Voice-configured profile | sigma-ai | `sigma-ai bhashini listen` → configure profile by speaking | `release/standalone` |
| Panchayat shared device | sigma-gram | Per-user login on shared device via Aadhaar OTP | `release/mobile` |

### P5 — Zenith Desktop Personalisation Features

```bash
sigma-zenith wallpaper set ~/Pictures/sunrise.jpg
sigma-zenith wallpaper set --slide ~/Pictures/   # slideshow
sigma-zenith font set "Noto Sans Devanagari" 13
sigma-zenith cursor set sigma-arrow-large         # HiDPI cursor
sigma-zenith animation speed fast|normal|off      # reduce motion
sigma-zenith workspace rename 1 "Work" 2 "Media" 3 "Terminal"
sigma-zenith bar position top|bottom
sigma-zenith bar clock format "%-I:%M %p"         # 12-hour IST format
sigma-zenith bar date format "%d %B %Y"           # "28 June 2026"
sigma-zenith bar language show|hide               # current IME
sigma-zenith bar network show|hide
sigma-zenith startup app add sigma-ca             # launch on login
sigma-zenith startup app remove sigma-ca
sigma-zenith keybinding set "Super+G" "sigma-agri msp"
sigma-zenith keybinding list
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Wallpaper setter + slideshow | `zenith_desktop/personalization/sigma_wallpaper.cpp` | `release/standalone` | Load PNG/JPEG → upload to compositor as background layer |
| Font picker | `zenith_desktop/personalization/sigma_font_picker.cpp` | `release/standalone` | List installed Noto fonts, set globally |
| Animation speed preference | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | Scale animation duration by preference |
| Custom keybindings | `zenith_desktop/personalization/sigma_keybindings.cpp` | `release/standalone` | TOML keybinding map → sigma-bus IPC commands |
| Workspace naming | `zenith_desktop/wm/sigma_tiling_wm.cpp` | `release/standalone` | Per-workspace name in `Workspace.name[32]` (already in struct) |
| Startup app launcher | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `release/standalone` | Read `apps.startup[]` from profile → fork + exec |
| IST clock format | `zenith_desktop/taskbar/sigma_systray.cpp` | `release/standalone` | Configurable strftime format, default IST |

---

## Per-Branch Compatibility × Automation × Customisation Summary

| Branch | Compat priority | Automation priority | Customisation priority |
|--------|----------------|--------------------|-----------------------|
| `kernel-exp` | POSIX shim VFS | Config.sigma VFS read | sigma-sysctl procfs |
| `drivers-dev` | `.deb`/AppImage compat | Hardware CI automation | sigma-drv hot-reload |
| `fs-dev` | File format (Tally/PDF) | sigma-cron persistent storage | SigmaFS per-user partition |
| `tools-dev` | Linux ELF W1 / Win32 W1 | sigma-cron + sigma-hook engine | Config.sigma TOML + profile VFS |
| `performance-optimized` | `.spkg` PGO builds | Nightly benchmark CI | Per-profile PGO target |
| `release/standalone` | Win32 W3 Notepad | India Stack scheduled sync | Full `~/.sigma_profile` engine |
| `release/cloud` | Flatpak via sigma-pod | Fleet policy automation | Per-container profile |
| `release/mobile` | sigma-ultra-lite USSD | OTA update automation | Language from SIM |
| `release/rtos` | ROS 2 compat layer | RT safety monitoring | Per-RT-task priority config |
| `release/microkernel` | Minimal syscall compat | Watchdog restart automation | Minimal 4-key config |
| `release/distributed` | CRDT sync compat | sigma-mesh job automation | Roaming profile via CRDT |
| `release/dual-boot` | Tally/EHR migration | Install automation | Import Windows user settings |
| `release/browser` | WASM compat (Emscripten) | Browser demo auto-build | Theme on web page |
| `release/app` | sigma-pkg + .spkg | App store auto-update | Per-app permission profiles |
| `docs-update` | N/A | Wiki auto-sync | Doc style customisation |
| `prepare-sigmaos-launch` | All W0 compat gates | Full release pipeline | OOBE personalisation wizard |
| `gh-pages` | Browser WASM | Site auto-deploy CI | Interactive theme picker |
| `master` | Stable compat baseline | Automation gates on merge | Stable profile schema |

---

## Master Compatibility Status Table

| Feature | File | Status |
|---------|------|--------|
| Linux ELF64 parser | `sigma_linux_compat.cpp` | ✅ Real |
| Linux syscall translator (15 calls) | `sigma_linux_compat.cpp` | ⚠️ Partial |
| Linux VMM section mapping | `sigma_linux_compat.cpp` | ❌ TODO |
| POSIX shim (6 functions) | `POSIXShim.cpp` | ⚠️ Partial |
| Win32 PE loader (parser) | `sigma_pe_loader.cpp` | ✅ Real |
| Win32 PE VMM mapping | `sigma_pe_loader.cpp` | ❌ TODO |
| Win32 NT API (20 functions) | `sigma_ntdll.cpp` | ⚠️ Partial |
| Win32 kernel32 console I/O | `sigma_kernel32_console.cpp` | ✅ Real |
| sigma-automation.sh (6 commands) | `sigma_automation.sh` | ✅ Real |
| sigma-cron daemon | `sigma_cron.cpp` | ❌ Missing |
| sigma-hook event engine | `sigma_hook.cpp` | ❌ Missing |
| Config.sigma TOML parser | `sigma_ignite.cpp` | ❌ Missing |
| Config.sigma VFS read | `sigma_ignite.cpp` | ❌ Missing |
| `~/.sigma_profile` VFS read | `sigma_profile_engine.cpp` | ⚠️ Partial |
| Hot-swap profiles | `sigma_cli.cpp` | ⚠️ Print stub |
| Zenith theme system | `sigma_theme_engine.cpp` | ⚠️ Partial |
| DID personalisation | `SovereignDID.cpp` | ⚠️ Partial |
| Profession auto-configure | `sigma_profile_engine.cpp` | ❌ Missing |
| Custom keybindings | `sigma_keybindings.cpp` | ❌ Missing |
| sigma-cron scheduled jobs | `sigma_cron.cpp` | ❌ Missing |

---

*See also: [Quality Stability Performance Roadmap](Quality-Stability-Performance-Roadmap) · [CLI Commands Roadmap](CLI-Commands-Roadmap) · [Feature Branch Roadmap](Feature-Branch-Roadmap) · [Windows Compatibility Layer Roadmap](Windows-Compatibility-Layer-Roadmap) · [India Profession Tools Roadmap](India-Profession-Tools-Roadmap)*
