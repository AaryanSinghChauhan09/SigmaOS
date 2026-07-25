# SigmaOS — Quality, Stability, Performance & Ease of Use Roadmap

Concrete, file-level engineering plan across every dimension that makes an OS
trustworthy, fast, and pleasant. Grounded in the June 2026 codebase audit.

---

## Current State (Honest Assessment)

| Dimension | Current grade | Biggest blocker |
|-----------|--------------|-----------------|
| **Stability** | D — cannot boot on real hardware | Kernel scheduler/MM/syscall stubs |
| **Performance** | C — good design, no real measurements | No bootable ISO to benchmark |
| **Quality / Testing** | C — CI defined, some tests real | QEMU tests are `echo` stubs |
| **Ease of use** | C — shell parser real, no TTY | No GPU driver → no GUI |
| **Security** | B — architecture excellent | PQC uses PRNG placeholders |
| **Accessibility** | D — AT-SPI2 header only | No real screen reader |
| **Documentation** | B — wiki 300+ pages | No Doxygen API reference |
| **Developer DX** | C — CI exists | No debugger, no profiler |

---

## Stability Roadmap

### S1 — Kernel Crash Prevention (all branches, blocks everything)

The single largest stability gap: stubs that silently return 0 instead of failing gracefully.

| Task | File | Branch | Fix |
|------|------|--------|-----|
| Replace all `return 0` syscall stubs with `ENOSYS` | `kernel/syscalls/sigma_syscalls.cpp` | `kernel-exp` | Return `-ENOSYS` (38), log unimplemented call |
| Panic handler with full register dump | `kernel/core/sigma_panic.cpp` | `kernel-exp` | Print RIP/RSP/CR3/backtrace to serial before halt |
| Stack overflow detection (guard pages) | `kernel/mm/sigma_vmm.cpp` | `kernel-exp` | Map guard page below each kernel stack |
| Double-fault handler | `arch/x86_64/sigma_idt.asm` | `kernel-exp` | IDT vector 8 — print stack frame, halt |
| Kernel ASLR + stack canaries | `kernel/core/sigma_start.cpp` | `kernel-exp` | `-fstack-protector-strong` + KASLR at boot |
| Memory corruption detector (shadow allocator) | `klib/sigma_slab_debug.cpp` | `kernel-exp` | Magic headers on alloc/free, detect overruns |
| Use-after-free detection | `klib/sigma_slab_debug.cpp` | `kernel-exp` | Poison freed memory with `0xDEADBEEF` pattern |
| Watchdog timer (hang detection) | `kernel/core/sigma_watchdog.cpp` | `kernel-exp` | APIC NMI if kernel silent for > 10 s |
| Rollback counter resilience | `kernel/core/sigma_kernel_main.c` | all | Already wired — ensure rollback fires after 3 failed boots |

### S2 — Driver Crash Isolation (SDF robustness)

```
Current: SDF drivers crash → silence (no recovery)
Target:  SDF driver crash → sigma-heal restarts it in < 500 ms, no data loss
```

| Task | File | Branch | Fix |
|------|------|--------|-----|
| sigma-heal driver restart hook | `kernel/diagnostics/sigma_crash_reporter.cpp` | `drivers-dev` | On Ring-3 driver segfault → send SIGKILL + restart |
| Driver health heartbeat | `hal/sigma_hal_watchdog.cpp` | `drivers-dev` | Each SDF driver must ping watchdog every 1 s |
| Driver state snapshot on crash | `hal/sigma_hal_watchdog.cpp` | `drivers-dev` | Save TX/RX ring state before kill for restore |
| NIC driver fail-open (maintain link) | `kernel/core/drivers/SovereignE1000.cpp` | `drivers-dev` | On driver restart, re-init hardware without link drop |
| GPU driver fail → VGA fallback | `drivers/graphics/sigma_kms.cpp` | `drivers-dev` | If i915 crashes, fall back to VESA text mode |

### S3 — Filesystem Consistency

| Task | File | Branch | Fix |
|------|------|--------|-----|
| Journal replay on unclean unmount | `fs/sigmafs/sigma_journal.cpp` | `fs-dev` | Replay log on next mount if dirty flag set |
| Atomic rename (rename-then-link) | `kernel/vfs/sigma_vfs.cpp` | `fs-dev` | Use journal: write new inode, update dir atomically |
| Fsck integration test | `tests/integration/test_fsck.sh` | `fs-dev` | Corrupt partition → sigma-fsck recovers |
| dm-verity on root partition | `kernel/fs/sigma_dmverity.cpp` | all `release/*` | Detect bit-flip corruption on every read |
| Ext4 journal flush on poweroff | `fs/ext4/sigma_ext4_ro.cpp` | `fs-dev` | Write barrier before shutdown syscall |

### S4 — Boot Reliability

| Task | File | Branch | Fix |
|------|------|--------|-----|
| A/B boot slot with rollback | `kernel/resilience/sigma_rollback.cpp` | all | Already exists — wire to UEFI EFI variable |
| Safe-mode profile (SIGMA_MINIMAL_MODE) | `kernel/core/sigma_kernel_main.c` | all | Already wired — verify fires after 3 failed boots |
| "Fix it" recovery menu at boot | `kernel/core/boot/sigma_boot_recovery_menu.c` | all | Text-mode menu: rollback/repair/reinstall |
| Bootable ISO CI gate | `.github/workflows/sigma_ci.yml` | all | Real QEMU boot test, assert prompt within 30 s |
| Boot success rate target | CI metric | all | ≥ 99% on all 3 QEMU profiles |

---

## Performance Roadmap

### P1 — Boot Time (target < 2 s on NVMe SSD)

| Current | Ubuntu 24.04 | Fedora 41 | SteamOS | SigmaOS target |
|---------|-------------|-----------|---------|----------------|
| Unknown (doesn't boot) | 43 s | 9 s | 8 s | **< 2 s** |

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Parallel shard ignition (ASI) | `kernel/core/boot/sigma_boot.c` | `kernel-exp` | Boot independent shards concurrently |
| sigma-init parallel service launch | `init/sigma_init.cpp` | `kernel-exp` | Dependency-ordered parallel start |
| Lazy driver init (probe only on use) | `hal/SovereignHAL.cpp` | `drivers-dev` | Defer driver init until first I/O request |
| Pre-computed page table (no runtime alloc) | `kernel/core/sigma_start.cpp` | `kernel-exp` | Static PML4 for first 4 GB at link time |
| Boot time measurement in CI | `.github/workflows/sigma_ci.yml` | all | `time qemu-boot.sh; assert < 2s` |
| sigma-dna early hardware profile | `kernel/core/sigma_dna.cpp` | `kernel-exp` | CPUID at boot → skip incompatible drivers |

### P2 — Context Switch Latency (target < 50 ns)

| Current | Linux generic | PREEMPT_RT | SigmaOS target |
|---------|--------------|-----------|----------------|
| Unknown | ~1,000 ns | ~200 ns | **< 50 ns** |

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Hand-tuned SYSCALL/SYSRET entry | `arch/x86_64/sigma_syscall_entry.asm` | `kernel-exp` | Minimal register save: RSP/RBP/RDI/RSI/RDX/RCX/R11 only |
| Avoid `mfence`/`lfence` on hot path | `arch/x86_64/sigma_syscall_entry.asm` | `kernel-exp` | Use `LFENCE` only where strictly needed |
| Per-CPU runqueue (no cross-CPU lock) | `kernel/sched/sigma_runqueue.cpp` | `performance-optimized` | Lock-free CAS queue per core |
| TLB flush minimisation (ASID) | `kernel/mm/sigma_vmm.cpp` | `kernel-exp` | Assign PCID per process, skip TLB flush on switch |
| Context switch benchmark CI | `tests/perf/bench_sched.cpp` | `performance-optimized` | 1M switches, assert p99 < 50 ns |

### P3 — Memory Performance (target: 0 unnecessary copies)

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Zero-copy DMA: NIC → userspace | `kernel/net/sigma_net.c` | `drivers-dev` | Map DMA buffer into user VA, no `memcpy` |
| Zero-copy DMA: NVMe → userspace | `drivers/storage/sigma_nvme.cpp` | `drivers-dev` | io_uring-style DMA-direct read |
| UBC: map page into user VA (no copy) | `kernel/fs/sigma_ubc.cpp` | `fs-dev` | Share physical page via VMM |
| GPU DMA-BUF for compositor | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | App → compositor zero-copy via DMA-BUF |
| Slab pre-warming at boot | `klib/sigma_slab_lockfree.cpp` | `performance-optimized` | Pre-alloc hot slab sizes (64B, 256B, 4KB) |
| Huge pages for kernel text | `kernel/core/sigma_start.cpp` | `kernel-exp` | Map kernel `.text` with 2 MB pages → fewer TLB misses |
| Memory pressure test | `tests/kernel/test_mm.sh` | `kernel-exp` | Alloc 90% RAM, verify no hang or silent corruption |

### P4 — Network Throughput (target: wire-speed on 1 GbE)

| Task | File | Branch | Detail |
|------|------|--------|--------|
| ZCLN (zero-copy LAN) implementation | `net/sigma_tcp_ip.cpp` | `drivers-dev` | `ZclnPacketDescriptor` queue already exists — wire it |
| TCP segmentation offload (TSO) | `kernel/net/sigma_net.c` | `drivers-dev` | Offload TCP checksum + segmentation to NIC |
| Receive-side scaling (RSS) | `kernel/core/drivers/SovereignE1000.cpp` | `drivers-dev` | Distribute RX descriptors across CPU queues |
| UDP fast path (< 10 µs RTT) | `kernel/net/sigma_net_socket.cpp` | `drivers-dev` | Bypass TCP for UDP, direct ring buffer write |
| Network benchmark CI | `tests/net/bench_net.sh` | `drivers-dev` | `iperf3` inside QEMU, assert ≥ 900 Mbps |

### P5 — Cryptographic Performance (target: PQC with zero overhead penalty)

| Current | Target |
|---------|--------|
| Kyber: PRNG placeholder (~∞ ms) | Kyber AVX-512: ≥ 5.8 M ops/sec |
| Dilithium: PRNG placeholder | Dilithium AVX-512: ≥ 1.5 M sign/sec |

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Real Kyber NTT (liboqs backend) | `crypto/SovereignKyber.cpp` | `performance-optimized` | Replace PRNG with liboqs pqcrystals-kyber |
| AVX-512 polynomial multiply | `crypto/sigma_kyber_avx512.cpp` | `performance-optimized` | 13× faster than reference C |
| ARM NEON NTT | `crypto/sigma_kyber_neon.cpp` | `release/mobile` | 5.7× faster on Cortex-A76 |
| Async Dilithium verification | `crypto/SovereignDilithium5.cpp` | `performance-optimized` | Background attestation thread, non-blocking boot |
| PQC benchmark CI | `tests/perf/bench_pqc.cpp` | `performance-optimized` | Assert ≥ 5.8M Kyber ops/sec on AVX-512 |
| Constant-time comparison | `crypto/SovereignKyber.cpp` | `performance-optimized` | Use `crypto_memcmp` to prevent timing side-channels |

### P6 — GPU / Rendering Performance (target: 1 frame @ 120 Hz, 8.3 ms)

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Vulkan triple-buffer pre-record | `zenith_desktop/compositor/sigma_vk_frame.cpp` | `release/standalone` | Pre-record command buffers, submit on vblank |
| SIMD matrix scaling (AVX-512) | `zenith_desktop/sigma_simd_scale.cpp` | `performance-optimized` | Replace scalar loops in transform path |
| Font atlas pre-built at start | `zenith_desktop/compositor/sigma_font.cpp` | `release/standalone` | Upload glyph atlas once, no per-frame uploads |
| Window compositing SIMD blend | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | NEON/AVX-512 alpha blend in `composite_window()` |
| Frame time metric in sigma-perf | `userland/tools/sigma_perf_cli.cpp` | `performance-optimized` | `sigma-zenith fps` shows p50/p95/p99 frame times |

---

## Quality & Testing Roadmap

### Q1 — Fix Fake CI Tests

**Biggest quality debt:** QEMU tests use `echo "Simulating..."` instead of real QEMU.

| Task | File | Branch | Fix |
|------|------|--------|-----|
| Replace echo stubs with real QEMU | `.github/workflows/sigma_qemu.yml` | all | `qemu-system-x86_64 -cdrom SigmaOS.iso -serial stdio` |
| Assert boot reaches shell prompt | `.github/workflows/sigma_ci.yml` | all | `grep -q "sigma-login" output.log` within 30 s |
| QEMU test on 3 profiles | `.github/workflows/sigma_ci.yml` | all | standalone, microkernel, cloud |
| Remove `|| true` from blocking tests | `.github/workflows/sigma_ci.yml` | all | Let CI actually fail on test failures |
| SPDX header enforcement (fail, not warn) | `.github/workflows/sigma_ci.yml` | all | Change `|| true` to real exit code |
| ABI stability gate | `Makefile` | `tools-dev` | `make check-abi` fails if `SIGMA_STABLE` symbol changes |

### Q2 — Unit Test Coverage (target: 80% critical paths)

**Current:** Tests exist in `tests/unit/`, `tests/kernel/`, `tests/fuzz/` but coverage unknown.

| Task | File | Branch | Target |
|------|------|--------|--------|
| Buddy allocator exhaustive test | `tests/kernel/test_allocator.cpp` | `kernel-exp` | Alloc every order, verify no leak, test OOM path |
| MCS scheduler budget test | `tests/kernel/test_sched.cpp` | `kernel-exp` | Verify budget exhaustion + replenish |
| VMM page table test | `tests/kernel/test_vmm.cpp` | `kernel-exp` | Map/unmap 1000 pages, verify CoW copy |
| VFS round-trip test | `tests/kernel/test_vfs.cpp` | `fs-dev` | Write → fsync → read → verify bytes match |
| TCP state machine test | `tests/net/test_tcp.cpp` | `drivers-dev` | SYN/SYN-ACK/ACK, FIN/FIN-ACK, RST paths |
| PE loader test | `tests/compat/win32/test_pe_loader.cpp` | `tools-dev` | Parse known PE files, verify section map |
| NT syscall table test | `tests/compat/win32/test_nt_syscall.cpp` | `tools-dev` | All mapped NT calls return correct NTSTATUS |
| sigma-ca GST compute test | `tests/userland/test_sigma_ca.cpp` | `release/standalone` | Compute CGST/SGST/IGST, verify rounding |
| sigma-agri MSP test | `tests/userland/test_sigma_agri.cpp` | `release/standalone` | All 26 crops × FY2025-26 values |
| PQC keygen-encap-decap round-trip | `tests/perf/bench_pqc.cpp` | `performance-optimized` | Verify shared secret matches after decap |

### Q3 — Fuzz Testing (existing targets need expansion)

**Current:** `tests/fuzz/fuzz_sigma_tcp.cpp` and `tests/fuzz/fuzz_sigma_pkg.cpp` exist.

| Task | File | Branch | Target |
|------|------|--------|--------|
| Fuzz syscall dispatcher | `tests/fuzz/fuzz_syscalls.cpp` | `kernel-exp` | Random syscall numbers + args |
| Fuzz PE loader | `tests/fuzz/fuzz_pe_loader.cpp` | `tools-dev` | Random bytes as PE input |
| Fuzz sigma-ca GST input | `tests/fuzz/fuzz_sigma_ca.cpp` | `release/standalone` | Random GSTIN + voucher JSON |
| Fuzz sigma-agri input | `tests/fuzz/fuzz_sigma_agri.cpp` | `release/standalone` | Random crop names + state inputs |
| Fuzz VFS path | `tests/fuzz/fuzz_vfs.cpp` | `fs-dev` | Random file paths with `../` traversal |
| Fuzz NT path normalizer | `tests/fuzz/fuzz_nt_path.cpp` | `tools-dev` | Random `\??\C:\...` paths |
| AFL++ CI (10 min budget) | `.github/workflows/sigma_ci.yml` | all | Increase from 30 s to 10 min on nightly |

### Q4 — Static Analysis & Linting

**Current:** `sigma_quality.yml` runs clang-tidy + cppcheck. Needs expansion.

| Task | File | Branch | Fix |
|------|------|--------|-----|
| clang-tidy: add `-warnings-as-errors` | `.github/workflows/sigma_quality.yml` | all | All warnings become CI failures |
| AddressSanitizer build variant | `.github/workflows/sigma_ci.yml` | `kernel-exp` | `-fsanitize=address` build, run unit tests |
| UBSan build variant | `.github/workflows/sigma_ci.yml` | `kernel-exp` | `-fsanitize=undefined` build |
| ThreadSanitizer for sigma-bus IPC | `.github/workflows/sigma_ci.yml` | `kernel-exp` | `-fsanitize=thread` on IPC code |
| MemorySanitizer for crypto | `.github/workflows/sigma_ci.yml` | `performance-optimized` | `-fsanitize=memory` on Kyber/Dilithium |
| CodeQL on all C++ | `.github/workflows/codeql-analysis.yml` | all | Already wired — verify it actually runs |
| Reproducible build verification | `.github/workflows/sigma_ci.yml` | all | Two builds same SOURCE_DATE_EPOCH → identical SHA256 |
| SBOM generation (CycloneDX) | `scripts/gen_sbom.sh` | `prepare-sigmaos-launch` | Auto-generate SBOM on every release |

### Q5 — Regression Prevention

**Current:** `scripts/regression_check.sh` exists. `tests/regression/` directory exists.

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Performance regression gate | `.github/workflows/sigma_ci.yml` | `performance-optimized` | If boot time regresses > 10%, CI fails |
| Context switch regression test | `tests/perf/bench_sched.cpp` | `performance-optimized` | Assert < 100 ns (2× budget) |
| Memory usage regression gate | `.github/workflows/sigma_ci.yml` | all | If idle RAM > 200 MB, warn |
| Kernel image size gate | `Makefile` | `release/microkernel` | `size vmlinuz-sigma` must be < 512 KB |
| Smoke test on every PR | `.github/workflows/sigma_ci.yml` | all | 5 min: build + boot + basic syscall |
| Full regression suite on merge to main | `.github/workflows/regression.yml` | `main` | 30 min: all scenarios |

---

## Ease of Use Roadmap

### U1 — First-Boot Experience (target: Indian user productive in < 5 minutes)

| Task | File | Branch | Detail |
|------|------|--------|--------|
| OOBE (Out-of-Box Experience) wizard | `userland/installer/sigma_oobe.cpp` | `release/standalone` | Language → timezone → DID login → profile → done |
| Language selection screen (22 Indian langs) | `userland/installer/sigma_oobe.cpp` | `release/standalone` | Sorted by speaker count; default = system locale |
| DID onboarding flow | `userland/installer/sigma_oobe.cpp` | `release/standalone` | QR scan → ABHA link → profession credential |
| Hardware auto-detection summary | `userland/installer/sigma_oobe.cpp` | `release/standalone` | "Found: Intel i7, 16 GB RAM, 512 GB NVMe, WiFi" |
| Profile auto-suggestion | `userland/installer/sigma_oobe.cpp` | `release/standalone` | Detect profession from DigiLocker → suggest sigma-ca / sigma-health |
| Internet connectivity check | `userland/installer/sigma_oobe.cpp` | `release/standalone` | WiFi setup during OOBE if driver available |
| OOBE completion in < 3 minutes | CI timer | `prepare-sigmaos-launch` | Script + assert |

### U2 — Shell Usability (sigma-sh)

**Current:** Parser and builtins are real, but no TTY read.

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Connect TTY via `sigma_sys_read(0,…)` | `userland/shell/sigma_shell.cpp` | `tools-dev` | Remove `line[0]='\0'` placeholder |
| Fish-style auto-suggestion (ghost text) | `userland/shell/sigma_shell.cpp` | `tools-dev` | `history_suggest(prefix)` already implemented — render ghost |
| Real tab completion (VFS readdir) | `userland/shell/sigma_shell.cpp` | `tools-dev` | TAB → `vfs_readdir(PWD)` for file/cmd completion |
| Syntax highlighting (VT100) | `userland/shell/sigma_shell.cpp` | `tools-dev` | Colorize keywords/paths on input line |
| Ctrl+C / Ctrl+D / Ctrl+Z | `userland/shell/sigma_shell.cpp` | `tools-dev` | SIGINT → clear line; Ctrl+D → EOF/exit |
| Multiline command continuation `\` | `userland/shell/sigma_shell.cpp` | `tools-dev` | `if [\` continuation prompt |
| `sigma-sh --login` profile load | `userland/shell/sigma_shell.cpp` | `tools-dev` | Source `~/.sigma_profile` at login |
| Comprehensive builtins (source/which/type/kill/jobs) | `userland/shell/sigma_shell.cpp` | `tools-dev` | Standard POSIX builtins |
| Error messages in user's language | `userland/shell/sigma_shell.cpp` | `release/standalone` | `sigma_locale` for Hindi/Tamil/Telugu error strings |

### U3 — Zenith Desktop Usability

| Task | File | Branch | Detail |
|------|------|--------|--------|
| App launcher (Rofi-style) | `zenith_desktop/launcher/sigma_launcher.cpp` | `release/standalone` | Super key → fuzzy search installed apps |
| Taskbar with workspace switcher | `zenith_desktop/taskbar/sigma_taskbar.cpp` | `release/standalone` | Show workspace 1–10, active app titles |
| System tray (clock, network, volume) | `zenith_desktop/taskbar/sigma_systray.cpp` | `release/standalone` | NTP clock, WiFi signal, audio level |
| Notification daemon | `zenith_desktop/notifications/sigma_notify.cpp` | `release/standalone` | Toast notifications via sigma-bus events |
| Right-click desktop context menu | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | New file / terminal / settings |
| Screenshot tool (Flameshot-style) | `userland/tools/sigma_screenshot.cpp` | `release/standalone` | Region select, copy to clipboard, save |
| Clipboard manager | `zenith_desktop/clipboard/sigma_clipboard.cpp` | `release/standalone` | History of last 20 clipboard entries |
| Font rendering with HarfBuzz | `zenith_desktop/compositor/sigma_font.cpp` | `release/standalone` | Complex script shaping for Devanagari/Tamil |
| HiDPI scaling (1×/1.5×/2×) | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | Auto-detect DPI, scale UI accordingly |
| Dark/light theme toggle | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | Respect `~/.sigma_profile theme=dark|light` |
| Keyboard shortcut legend overlay | `zenith_desktop/launcher/sigma_launcher.cpp` | `release/standalone` | `?` key shows all shortcuts |

### U4 — sigma-cli Discoverability

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `--help` on every subcommand | `userland/tools/sigma_cli.cpp` | `tools-dev` | Every verb shows usage + examples |
| `--json` output flag | `userland/tools/sigma_cli.cpp` | `tools-dev` | Machine-readable JSON for scripting |
| `--dry-run` preview | `userland/tools/sigma_cli.cpp` | `tools-dev` | Show what would happen without executing |
| `sigma-cli help <topic>` | `userland/tools/sigma_cli.cpp` | `tools-dev` | Topic-based help (gst, pods, wine, etc.) |
| Tab completion in sigma-sh | `userland/shell/sigma_shell.cpp` | `tools-dev` | Complete `sigma-cli pro<TAB>` → `sigma-cli profile` |
| Interactive mode (`sigma-cli wizard`) | `userland/tools/sigma_cli.cpp` | `release/standalone` | TUI wizard for common tasks (first GST filing, etc.) |
| Multilingual CLI output | `userland/tools/sigma_cli.cpp` | `release/standalone` | `SIGMA_LANG=hi sigma-agri msp` prints in Hindi |

### U5 — Package Manager UX

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Progress bar on install | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Live download progress + hash verify |
| Dependency conflict resolution | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Explain conflict, suggest fix, not just fail |
| Rollback on failed install | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Atomic staging swap — bad install never leaves system broken |
| Install summary (what changed) | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Print added/removed files + disk delta |
| `sigma-pkg why <name>` | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Show which package depends on this |
| Offline install from .spkg file | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `release/mobile` | `sigma-pkg install ./sigma-agri.spkg` |
| Mirror auto-selection by latency | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Ping all mirrors, use fastest |

### U6 — Error Messages & Recovery

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Human-readable kernel panic | `kernel/core/sigma_panic.cpp` | `kernel-exp` | "Kernel panic: null pointer in sigma-net driver. Rebooting in 10 s." |
| Actionable sigma-heal suggestions | `userland/ai/sigma_heal_ai.cpp` | `release/standalone` | "Crash caused by iwlwifi driver. Try: sigma-drv reload iwlwifi" |
| sigma-sh error context | `userland/shell/sigma_shell.cpp` | `tools-dev` | "Command not found: sigam-cli. Did you mean: sigma-cli?" |
| sigma-pkg install failure guidance | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | "Failed: disk full. Free 1.2 GB with sigma-disk clean." |
| Multilingual error messages (Phase H) | `userland/locales/` | `release/standalone` | All errors translated to 22 Indian languages |
| Recovery wizard on repeated crash | `kernel/core/boot/sigma_boot_recovery_menu.c` | all | After 3 crashes → auto-suggest rollback or repair |

---

## Security Quality Roadmap

### SE1 — Fix Known Critical Security Stubs

| Issue | File | Branch | Fix |
|-------|------|--------|-----|
| CryptFS `derive_key()` returns 32 zero bytes (Issue #44) | `crypto/SovereignCryptFS.cpp` | `kernel-exp` | Real Argon2id (time=3, mem=65536, threads=4) |
| Kyber/Dilithium use PRNG not NTT | `crypto/SovereignKyber.cpp` | `performance-optimized` | Integrate liboqs real lattice arithmetic |
| `sigma_attestation_verify()` always returns `true` | `security/SovereignAttestation.cpp` | `kernel-exp` | Real TPM2 PCR measurement check |
| Hardcoded test credentials in any source | All | all | CI grep gate: fail on `password="`, `secret="`, `api_key="` |
| VFS path traversal not guarded | `kernel/vfs/sigma_vfs.cpp` | `fs-dev` | Reject `../` traversal attempts, fuzz-tested |
| NT path normalizer unchecked | `runtime/compat/win32/sigma_ntdll.cpp` | `tools-dev` | Validate `\??\` prefix, reject path traversal |

### SE2 — Security CI Gates

| Task | File | Branch | Detail |
|------|------|--------|--------|
| CodeQL on every PR | `.github/workflows/codeql-analysis.yml` | all | Already defined — verify it passes |
| Secrets scan (fail on hardcoded creds) | `.github/workflows/sigma_ci.yml` | all | Already in security-scan job — make it fail |
| SPDX license headers enforced | `.github/workflows/sigma_ci.yml` | all | Fail CI if any `.cpp`/`.h` missing `SPDX-License-Identifier` |
| sigma-mac policy verified on boot | `kernel/security/sigma_mac.cpp` | all | Assert `.sigma-policy` present and Dilithium3-verified |
| PQC round-trip CI test | `tests/perf/bench_pqc.cpp` | `performance-optimized` | keygen → encap → decap → verify match |

---

## Accessibility Roadmap

### A1 — Screen Reader (sigma-a11y)

**Current:** AT-SPI2 header present; no implementation. sigma-bhashini offline TTS exists as a header.

| Task | File | Branch | Detail |
|------|------|--------|--------|
| AT-SPI2 accessibility tree walker | `userland/a11y/sigma_a11y.cpp` | `release/standalone` | Walk Zenith window tree, enumerate widgets |
| Screen reader TTS via sigma-bhashini | `userland/a11y/sigma_a11y.cpp` | `release/standalone` | Widget name + role → sigma-bhashini TTS → HDA audio |
| Keyboard navigation (no mouse required) | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | Tab through widgets, Enter to activate |
| High-contrast theme preset | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | WCAG 2.2 AA contrast ratio ≥ 4.5:1 |
| Large text mode | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | `sigma-zenith scale 2.0` — all UI elements 2× |
| Braille display output (BRLTTY bridge) | `userland/a11y/sigma_braille.cpp` | `release/standalone` | USB Braille display via BRLTTY protocol |
| Switch access (single-switch scanning) | `userland/a11y/sigma_switch.cpp` | `release/standalone` | Dwell-time + switch button scanning |
| WCAG 2.2 AA release gate | CI check | `prepare-sigmaos-launch` | Automated aXe scan of Zenith UI elements |

### A2 — Indian Language Accessibility

| Task | File | Branch | Detail |
|------|------|--------|--------|
| 22-language UI strings | `userland/locales/sigma_l10n.cpp` | `release/standalone` | All system messages translated |
| Devanagari font rendering | `zenith_desktop/compositor/sigma_font.cpp` | `release/standalone` | HarfBuzz complex script shaping |
| Tamil / Telugu / Bengali font | `zenith_desktop/compositor/sigma_font.cpp` | `release/standalone` | Full Unicode block coverage |
| Voice navigation (sigma-bhashini ASR) | `userland/a11y/sigma_a11y.cpp` | `release/standalone` | Speak command → shell executes |
| USSD accessibility mode | `userland/sigma_ultra.cpp` | `release/mobile` | Text-only mode for low-vision users on feature phones |

---

## Developer Experience (DX) Roadmap

### D1 — Debugging Tools

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-gdb (source-level debugger) | `userland/devtools/sigma_gdb.cpp` | `tools-dev` | GDB-compatible protocol, DWARF debug info |
| sigma-strace (syscall tracer) | `userland/devtools/sigma_strace.cpp` | `tools-dev` | Print syscall name + args on every call |
| sigma-perf (hardware PMU profiler) | `userland/tools/sigma_perf_cli.cpp` | `performance-optimized` | RDPMC for cycles/cache-misses/branch-mispred |
| sigma-ltrace (library call tracer) | `userland/devtools/sigma_ltrace.cpp` | `tools-dev` | Intercept sigma-sdk calls |
| Kernel oops prettifier | `kernel/core/sigma_panic.cpp` | `kernel-exp` | Decode RIP → function name + line via DWARF |
| Core dump to SigmaFS | `kernel/diagnostics/sigma_crash_reporter.cpp` | `kernel-exp` | Dump process memory on SIGSEGV → `/sigma/cores/` |
| sigma-memcheck (Valgrind-style) | `userland/devtools/sigma_memcheck.cpp` | `tools-dev` | Shadow memory tracking for heap use-after-free |

### D2 — Build System Quality

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `make help` target | `Makefile` | all | Print all available targets + descriptions |
| `make check` target | `Makefile` | all | Build + unit tests + static analysis in one command |
| `make iso` produce real bootable image | `Makefile` | `kernel-exp` | Currently broken — fix after Phase 0 kernel |
| `make PROFILE=pgo iso` | `Makefile` | `performance-optimized` | PGO-instrumented build |
| `make check-abi` | `Makefile` | `tools-dev` | Verify no `SIGMA_STABLE` symbol changed |
| `make check-stubs` | `Makefile` | all | Count remaining stubs, warn if > threshold |
| `make docs` | `Makefile` | `docs-update` | Run Doxygen → `docs/api/html/` |
| Incremental compilation (ccache) | `Makefile` | all | `export CCACHE_DIR=.cache/ccache` |
| Cross-compile for ARM64 | `toolchain-aarch64-elf.cmake` | `release/mobile` | `cmake -DCMAKE_TOOLCHAIN_FILE=toolchain-aarch64-elf.cmake` |
| Dependency install script | `scripts/setup.sh` | all | `./scripts/setup.sh` installs all build deps |

### D3 — API Documentation

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Doxygen configured for all subsystems | `Doxyfile` | `docs-update` | `INPUT = kernel/ userland/ include/ crypto/` |
| `docs/api/html/` published to gh-pages | `.github/workflows/sigma_ci.yml` | `gh-pages` | Auto-publish on merge to `main` |
| `sigma_error.h` standard error codes | `include/sigma_error_codes.h` | `tools-dev` | Consistent `sigma_err_t` return type everywhere |
| Man pages for all 55 profession CLIs | `docs/man/` | `docs-update` | `sigma-ca(1)`, `sigma-agri(1)`, etc. |
| Getting started in 5 commands | `wiki_repo/Getting-Started.md` | `docs-update` | `sigma-pkg install sigma-ca` → working demo |
| Architecture diagram (auto-generated) | `scripts/gen_arch_diagram.sh` | `docs-update` | Graphviz from `sigma-bus` topic map |
| India Stack API quick reference | `wiki_repo/India-Stack-API-Reference.md` | `docs-update` | ABDM/GSTN/UPI endpoints + curl examples |

### D4 — sigma-sdk Quality

| Task | File | Branch | Detail |
|------|------|--------|--------|
| ABI stability CI (`make check-abi`) | `Makefile` | `tools-dev` | `SIGMA_STABLE` symbols never change |
| C++ wrapper for all syscalls | `include/sigma_sdk.h` | `tools-dev` | `sigma::fs::open()` wraps raw syscall |
| Error handling guide | `docs/CONTRIBUTING.md` | `docs-update` | Every API returns `sigma_err_t`, no exceptions |
| Example app skeleton | `docs/examples/hello_sigma.cpp` | `docs-update` | 50-line app that uses VFS + sigma-bus |
| India Stack SDK bindings | `include/india/sigma_abdm.h` | `tools-dev` | Type-safe C++ wrappers for all India Stack APIs |
| Rust bindings (Phase 9) | `userland/rs/sigma_sdk.rs` | `kernel-exp` | `extern "C"` wrappers exposed to Rust |

---

## Per-Branch Quality Targets

### `kernel-exp` quality gates
```
[ ] Kernel boots in QEMU with no errors on serial
[ ] Buddy allocator passes 10,000-iteration stress test
[ ] VMM maps/unmaps 1,000 pages without leak
[ ] No `return 0` stubs remain in syscall.cpp
[ ] Panic handler prints register dump on fault
[ ] Watchdog fires on 10-second hang
[ ] KASLR enabled: base address differs on each boot
```

### `drivers-dev` quality gates
```
[ ] NIC TX/RX: ping 10.0.2.2 in QEMU, no packet loss
[ ] NVMe: read 1 GB file, SHA256 matches original
[ ] GPU: VirtIO-GPU renders Zenith frame at ≥ 60 FPS
[ ] SDF driver crash → restarted in < 500 ms
[ ] Driver health heartbeat CI: 60-second stress run
```

### `fs-dev` quality gates
```
[ ] VFS write → fsync → read roundtrip, bytes match
[ ] Journal replay after abrupt poweroff: no data loss
[ ] fuzz_vfs: 10-minute AFL++ run, no crashes
[ ] dm-verity: 1-byte corruption detected on read
[ ] sigma-fsck repairs 10 known corruption patterns
```

### `performance-optimized` quality gates
```
[ ] Context switch p99 < 100 ns (2× budget)
[ ] Kyber-1024 AVX-512: ≥ 5.8 M ops/sec
[ ] Dilithium-5 sign: ≥ 1.5 M sig/sec
[ ] Boot time < 2 s on NVMe SSD (CI timer)
[ ] Idle RAM < 150 MB (sigma-mem stats)
[ ] No performance regression on merge to main
```

### `release/standalone` quality gates
```
[ ] OOBE complete in < 3 minutes
[ ] All 55 profession apps installable via sigma-pkg
[ ] sigma-ca computes GSTR-1 for 1,000 invoices < 1 s
[ ] sigma-agri MSP lookup for all 26 crops passes
[ ] Zenith compositor: 1-frame latency at 120 Hz
[ ] WCAG 2.2 AA: all UI elements pass contrast check
[ ] Screen reader announces all interactive widgets
[ ] Hindi/Tamil/Telugu text renders without tofu
```

### `release/cloud` quality gates
```
[ ] sigma-pod OOM-kills at exactly --mem limit
[ ] sigma-pod CPU throttled at --cpu limit
[ ] dm-verity detects corrupted .spkg image
[ ] sigma-fleet agent registers + pulls policy
[ ] 100 concurrent containers: no inter-container leak
```

### `release/mobile` quality gates
```
[ ] Boots on Raspberry Pi 4 in < 10 s
[ ] sigma-ultra boots on Pi Zero in < 5 s
[ ] USSD menu responds in < 2 s over 2G
[ ] Neon Kyber: ≥ 2.1 M ops/sec on Cortex-A76
[ ] Battery drain < 0.4 W idle on Pi Zero
```

### `release/rtos` quality gates
```
[ ] IRQ latency p99 < 10 µs
[ ] EDF: zero missed deadlines in 60-second stress
[ ] sigma-mining alert: DGMS report within 2 hours
[ ] Priority inheritance: no priority inversion
```

---

## Cumulative Benchmark Targets vs. Competitors

| Metric | Ubuntu 24.04 | Windows 11 | SteamOS | **SigmaOS Target** |
|--------|-------------|------------|---------|---------------------|
| Boot time (NVMe SSD) | 43 s | 35 s | 8 s | **< 2 s** |
| Idle RAM (desktop) | 847 MB | 2,100 MB | 600 MB | **< 150 MB** |
| Context switch p99 | ~1,000 ns | ~1,200 ns | ~300 ns | **< 50 ns** |
| Kyber-1024 ops/sec | N/A | N/A | N/A | **5.8 M (AVX-512)** |
| Kernel CVE patch | Reboot | Reboot | Reboot | **No reboot (kpatch)** |
| App launch cold | 1.5 s | 2.0 s | 1.2 s | **< 0.5 s** |
| PQC by default | ❌ | ❌ | ❌ | **✅ ML-KEM + ML-DSA** |
| Driver crash → recovery | BSOD / panic | BSOD | panic | **< 500 ms** |
| Screen reader languages | English + 10 | English + 40 | English | **22 Indian languages** |
| OOBE time | 10 min | 15 min | 5 min | **< 3 min** |

---

## Master Checklist — Quality, Stability, Performance, UX

### Phase Q0 — Must have before any public release
```
[S1] [ ] Kernel: no silent return-0 stubs in syscall handler
[S1] [ ] Kernel: panic handler prints register dump
[S1] [ ] Kernel: watchdog fires on 10-second hang
[S2] [ ] SDF driver crash → restarted, not kernel panic
[S3] [ ] Journal replay: no data loss on poweroff
[S4] [ ] Bootable ISO: QEMU reaches shell in < 30 s
[Q1] [ ] CI QEMU test: real QEMU, not echo stubs
[SE1][ ] CryptFS: real Argon2id (fix Issue #44)
[SE1][ ] No hardcoded credentials in any source file
[U3] [ ] Zenith: compositor renders a frame on real GPU
```

### Phase Q1 — Before v16.0 Apex
```
[P1] [ ] Boot time < 2 s on NVMe SSD (CI timer)
[P2] [ ] Context switch < 100 ns (2× budget)
[P3] [ ] Zero-copy DMA for NIC and NVMe
[P5] [ ] Real Kyber NTT ≥ 5.8 M ops/sec (AVX-512)
[Q2] [ ] 80% unit test coverage on critical paths
[Q3] [ ] Fuzz 7 targets for 10 min each in nightly CI
[U1] [ ] OOBE: first-boot wizard in < 3 minutes
[U2] [ ] sigma-sh: real TTY read, tab completion, Ctrl+C
[U3] [ ] App launcher, taskbar, system tray working
[A1] [ ] Screen reader: sigma-bhashini TTS on AT-SPI2
```

### Phase Q2 — Before v17.0 Sovereign
```
[P6] [ ] Vulkan compositor: 1-frame latency @ 120 Hz
[Q4] [ ] All warnings-as-errors in clang-tidy
[Q5] [ ] No performance regression gate on CI
[U6] [ ] All error messages human-readable + actionable
[A2] [ ] 22-language UI strings complete
[D1] [ ] sigma-gdb + sigma-strace working
[D3] [ ] Doxygen API reference published to gh-pages
[SE2][ ] PQC round-trip CI test passing
```

### Phase Q3 — v18.0 Singularity
```
[P1] [ ] Boot time < 1 s (parallel ASI fully tuned)
[Q5] [ ] Formal verification: IPC + scheduler (IIT/IISc)
[A1] [ ] WCAG 2.2 AA certified on every Zenith release
[D4] [ ] Rust bindings for sigma-sdk
[SE1][ ] FIPS 203/204/205 final standard bindings
```

---

*See also: [Gap Analysis](Gap-Analysis) · [Feature Branch Roadmap](Feature-Branch-Roadmap) · [Branch Development Roadmap](Branch-Development-Roadmap) · [Windows Parity Roadmap](Windows-Parity-Roadmap) · [Development Roadmap](Development-Roadmap)*

---

## Energy Efficiency Roadmap

### E1 — Power Management Stack

**Target:** sigma-ultra < 0.4 W idle; laptop < 2.5 W idle (screen off).

| Task | File | Branch | Detail |
|------|------|--------|--------|
| ACPI P-state governor | `kernel/power/sigma_perf_governor.cpp` | `performance-optimized` | Write `IA32_PERF_CTL` MSR; ondemand/powersave/performance modes |
| ACPI C-state idle | `kernel/power/sigma_power_manager.cpp` | `performance-optimized` | `HLT` in idle loop; C3/C6/C8 for deeper sleep |
| Wakeup source accounting | `kernel/power/sigma_wakeup.cpp` | `performance-optimized` | Every wakeup attributed to a driver/process |
| Suspend-to-RAM (S3) | `kernel/power/sigma_suspend.cpp` | `release/standalone` | TPM2 state preserved across S3 |
| Display DPMS via DRM/KMS | `drivers/graphics/sigma_kms.cpp` | `drivers-dev` | Screen off after 3 min idle |
| USB autosuspend | `drivers/usb/sigma_xhci.cpp` | `drivers-dev` | Suspend idle USB devices after 2 s |
| Runtime PM per driver | `hal/sigma_rpm.cpp` | `drivers-dev` | Idle drivers cut power automatically |
| Thermal governor | `kernel/power/sigma_thermal.cpp` | `performance-optimized` | Throttle before hitting thermal limit |
| Battery status daemon | `userland/daemons/sigma_battery.cpp` | `release/standalone` | `sigma-power status` shows %, rate, ETA |
| Power regression test | `tests/perf/bench_power.sh` | `performance-optimized` | 60-second idle run; assert < 2.5 W via RAPL |

### E2 — sigma-ultra Power Optimisation (ARM)

| Task | File | Branch | Detail |
|------|------|--------|--------|
| ARM WFI idle | `arch/arm64/sigma_idle.asm` | `release/mobile` | `WFI` instruction in idle loop |
| big.LITTLE core parking | `kernel/sched/sigma_numa.cpp` | `release/mobile` | Park LITTLE cores when load < 20 % |
| Display off after 30 s idle | `arch/arm64/sigma_bcm2711.cpp` | `release/mobile` | HDMI blanking for Pi 4 |
| Offline-first: no polling | `userland/sigma_ultra.cpp` | `release/mobile` | Wake on event, not 1-second poll loop |

---

## Reliability Engineering Roadmap

### R1 — Mean Time Between Failures (MTBF) Targets

| Component | Current | Target | Method |
|-----------|---------|--------|--------|
| Kernel panic rate | Unknown | 0 panics / 1,000 boot-hours | Watchdog + crash stats |
| SDF driver crash rate | Unknown | < 1 crash / 10,000 hours | Driver heartbeat monitoring |
| Filesystem corruption | Unknown | 0 silent corruptions | dm-verity on every read |
| Network drop rate | Unknown | < 0.01 % packet loss | RX/TX counters in sigma-net |
| Package install failure | Unknown | < 0.1 % of installs | dm-verity + atomic rollback |

### R2 — Chaos Engineering

Deliberately break components to verify recovery paths work.

| Test | File | Branch | What it checks |
|------|------|--------|----------------|
| Kill NIC driver mid-transfer | `tests/chaos/test_nic_crash.sh` | `drivers-dev` | sigma-heal restarts driver, connection resumes |
| Corrupt 1 block of SigmaFS | `tests/chaos/test_fs_corrupt.sh` | `fs-dev` | dm-verity detects, sigma-fsck repairs |
| OOM during package install | `tests/chaos/test_oom_install.sh` | `tools-dev` | Install rolls back cleanly, no partial state |
| Kill compositor mid-render | `tests/chaos/test_compositor_crash.sh` | `release/standalone` | sigma-heal restarts, windows reappear |
| Force 3 failed boots | `tests/chaos/test_rollback.sh` | all | Automatic rollback to known-good boot |
| Kill sigma-bus mid-IPC | `tests/chaos/test_bus_crash.sh` | `release/cloud` | IPC clients reconnect automatically |
| Inject network partition | `tests/chaos/test_net_partition.sh` | `release/distributed` | Raft leader election recovers |

### R3 — Longevity Testing

| Test | Duration | Branch | Pass criteria |
|------|----------|--------|---------------|
| QEMU continuous boot | 72 hours | all | No kernel panic or hang |
| File I/O stress (bonnie++) | 8 hours | `fs-dev` | No data corruption, no journal errors |
| Network throughput soak | 24 hours | `drivers-dev` | No packet loss, throughput ≥ 800 Mbps |
| Profession app soak | 4 hours | `release/standalone` | 10,000 GST invoices, zero errors |
| sigma-pod lifecycle | 1 hour | `release/cloud` | 1,000 create/start/stop/destroy cycles |
| Memory soak | 8 hours | `kernel-exp` | malloc/free 1M cycles, no leak |

---

## Observability & Monitoring Roadmap

### O1 — sigma-observatory (native monitoring dashboard)

**New file:** `userland/tools/sigma_observatory.cpp`

```
sigma-observatory             # launch full TUI dashboard
sigma-observatory --json      # machine-readable JSON stream
sigma-observatory cpu         # per-core utilization + frequency
sigma-observatory mem         # RAM usage, slab sizes, page cache
sigma-observatory net         # per-interface TX/RX bytes/packets
sigma-observatory io          # per-device IOPS, latency histogram
sigma-observatory proc        # process list sorted by CPU/mem
sigma-observatory temp        # CPU + GPU temperature
sigma-observatory pqc         # PQC ops/sec live counter
sigma-observatory scheduler   # runqueue depth per CPU
sigma-observatory power       # RAPL energy counters, battery %
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `/proc/sigma/` virtual filesystem | `kernel/vfs/sigma_procfs.cpp` | `kernel-exp` | Expose kernel stats as readable files |
| Per-CPU stats in procfs | `kernel/vfs/sigma_procfs.cpp` | `kernel-exp` | `/proc/sigma/cpu/N/stat` |
| Memory stats in procfs | `kernel/vfs/sigma_procfs.cpp` | `kernel-exp` | `/proc/sigma/meminfo` — buddy orders, slab |
| Network stats in procfs | `kernel/vfs/sigma_procfs.cpp` | `kernel-exp` | `/proc/sigma/net/dev` — RX/TX per interface |
| sigma-observatory TUI | `userland/tools/sigma_observatory.cpp` | `performance-optimized` | VT100-based dashboard, 1 s refresh |
| Prometheus metrics endpoint | `userland/tools/sigma_observatory.cpp` | `release/cloud` | `sigma-observatory --prometheus :9090` |
| OpenTelemetry export | `userland/sigma_otel_export.cpp` | `release/cloud` | Forward metrics to Splunk/Grafana |

### O2 — sigma-audit (tamper-evident logging)

```
sigma-audit log                    # recent audit entries
sigma-audit log --follow           # real-time stream
sigma-audit log --filter kernel    # filter by subsystem
sigma-audit verify                 # verify Dilithium3 chain
sigma-audit export audit.json      # export for CERT-In
sigma-audit push                   # push to sigma-fleet
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| WORM audit register | `kernel/security/sigma_immutable_audit_trail.cpp` | all | Write-once hardware registers for forensic profile |
| Dilithium3 on every log entry | `kernel/security/sigma_immutable_audit_trail.cpp` | all | ML-DSA-87 sign each event |
| Log rotation with integrity | `userland/tools/sigma_audit_cli.cpp` | `tools-dev` | Rotate on size limit; keep chain of custody |
| CERT-In JSON export format | `userland/tools/sigma_audit_cli.cpp` | `release/standalone` | 6-hour mandatory incident disclosure format |

---

## Release Engineering Roadmap

### RE1 — Release Pipeline

**Current:** `scripts/release.sh`, `scripts/sign_release.sh`, `.github/workflows/sigma_release.yml` exist but are not fully wired.

```bash
# Full release pipeline:
sigma-release check          # all quality gates pass
sigma-release iso            # reproducible build
sigma-release sign           # ML-DSA-87 sign ISO + packages
sigma-release publish        # upload to GitHub Releases
sigma-release notes v16.0    # auto-generate release notes
sigma-release verify v16.0-rc1.iso  # verify signature
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Reproducible build gate | `.github/workflows/sigma_ci.yml` | all | Two builds → identical SHA256 (SOURCE_DATE_EPOCH fixed) |
| ML-DSA-87 sign ISO | `scripts/sign_release.sh` | `prepare-sigmaos-launch` | `pqc_sign(iso_hash, sk)` → `.sig` file |
| SBOM generation (CycloneDX) | `scripts/gen_sbom.sh` | `prepare-sigmaos-launch` | Auto-generate Software Bill of Materials |
| Release notes from git log | `scripts/gen_changelog.sh` | `docs-update` | Conventional commits → CHANGELOG.md |
| Version bump script | `scripts/bump_version.sh` | `prepare-sigmaos-launch` | Update version in Makefile + headers atomically |
| Release artifact checksums | `.github/workflows/sigma_release.yml` | `prepare-sigmaos-launch` | SHA256 + Blake3 checksums published with release |
| India CDN mirror push | `scripts/release.sh` | `prepare-sigmaos-launch` | Push to `packages.sigmaos.dev` + NIC mirror |

### RE2 — Branch Management Quality

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Branch parity CI (existing) | `scripts/ci_branch_check.sh` | all | Already works — run on every PR |
| Protected branches policy | `.github/` settings | all | `main` + `release/*` require CI pass + review |
| PR template enforces checklist | `.github/PULL_REQUEST_TEMPLATE.md` | all | Tests / docs / CURRENT_PROBLEMS_MANIFEST updated |
| Auto-label by subsystem | `.github/workflows/` | all | Label `net`, `kernel`, `zenith`, `compat` by file path |
| Stale branch cleanup | `.github/workflows/` | all | Auto-close PRs inactive > 90 days |
| Tag signing with ML-DSA | `scripts/sign_release.sh` | all | `git tag -s` using Dilithium3 key |

---

## Community & Contribution Quality

### C1 — Contribution Pipeline

| Task | File | Branch | Detail |
|------|------|--------|--------|
| CONTRIBUTING.md completeness | `CONTRIBUTING.md` | `docs-update` | Setup → build → test → PR → review cycle |
| Code review checklist | `.github/PULL_REQUEST_TEMPLATE.md` | all | Security / performance / docs / test evidence |
| Good first issues labelled | GitHub issues | all | Tag 20+ `good-first-issue` items from Phase G list |
| Architecture decision records | `docs/adr/` | `docs-update` | One ADR per major design decision |
| `sigma_error.h` standard | `include/sigma_error_codes.h` | `tools-dev` | Consistent `sigma_err_t` return everywhere |
| SPDX header on all new files | CI gate | all | Block merge if header missing |

### C2 — Developer Onboarding

```bash
# 5-command new contributor setup:
git clone https://github.com/AaryanSinghChauhan09/SigmaOS
cd SigmaOS
./scripts/setup.sh          # install deps (Ubuntu 22.04+)
make PROFILE=microkernel    # build smallest profile
make test                   # run unit tests
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `setup.sh` installs all deps | `scripts/setup.sh` | all | `apt install build-essential nasm cmake clang` + cross-toolchain |
| Dev container (`.devcontainer/`) | `.devcontainer/devcontainer.json` | all | Already exists — verify it builds cleanly |
| `make test` single command | `Makefile` | all | Runs all unit tests, prints pass/fail |
| GitHub Codespaces ready | `.devcontainer/devcontainer.json` | all | One-click cloud dev environment |
| Regression test on PR | `.github/workflows/sigma_ci.yml` | all | Every PR runs smoke + unit tests |
| Architecture video walkthrough | `docs/videos/` | `docs-update` | 10-min video: kernel → SDF → zenith → profession apps |

---

## Per-Branch Summary Table (All Dimensions)

| Branch | Stability | Performance | Quality | Ease of Use | Energy | Observability |
|--------|-----------|-------------|---------|-------------|--------|---------------|
| `kernel-exp` | 🔴 Panic handler, no-return-0 stubs, guard pages | 🔴 Context switch < 50 ns, KASLR, ASID | 🔴 Real QEMU CI, ASan/UBSan | 🟡 Human-readable panic | 🟡 ACPI C-state idle | 🟠 `/proc/sigma/` procfs |
| `drivers-dev` | 🔴 SDF crash → restart < 500 ms | 🔴 Zero-copy DMA NIC/NVMe | 🟠 NIC/NVMe fuzz tests | 🟡 Driver hot-reload CLI | 🟠 USB autosuspend, DPMS | 🟡 Driver stats in procfs |
| `fs-dev` | 🔴 Journal replay, dm-verity | 🟠 UBC zero-copy, read-ahead | 🟠 VFS fuzz, journal crash test | 🟡 sigma-fsck guided repair | 🟡 Flush on poweroff | 🟡 Cache hit/miss stats |
| `tools-dev` | 🟠 Rollback on failed install | 🟠 Package install < 0.5 s | 🟠 ABI gate, stub count gate | 🔴 TTY read, tab complete, --help | 🟢 N/A | 🟡 sigma-audit log |
| `performance-optimized` | 🟡 Memory soak tests | 🔴 All P1–P6 targets | 🔴 PQC CI benchmark, regression gate | 🟡 sigma-observatory TUI | 🔴 RAPL power gate | 🔴 Prometheus endpoint |
| `release/standalone` | 🟠 Compositor crash → self-heal | 🟠 Vulkan 120 Hz | 🔴 WCAG 2.2 AA gate | 🔴 OOBE < 3 min, app launcher | 🟠 S3 suspend, display DPMS | 🟡 sigma-battery daemon |
| `release/cloud` | 🟠 cgroup OOM at exact limit | 🟠 100 concurrent containers | 🟠 Container leak test | 🟡 sigma-fleet UX | 🟡 N/A | 🔴 Prometheus + OTEL |
| `release/mobile` | 🟠 Pi4 boots reliably | 🟠 NEON Kyber ≥ 2.1 M ops/sec | 🟠 Pi Zero battery regression | 🔴 OOBE in < 2 min, sigma-ultra USSD | 🔴 < 0.4 W idle Pi Zero | 🟡 RAPL on ARM (MMIO) |
| `release/rtos` | 🔴 Zero missed RT deadlines | 🔴 IRQ < 10 µs | 🟠 60-second stress test | 🟡 sigma-rt CLI | 🟡 Power profiling | 🟡 Jitter histogram |
| `release/microkernel` | 🟠 Kernel < 512 KB | 🟠 Boot < 1 s | 🟠 Formal verification hooks | 🟡 Minimal 8-command CLI | 🟡 WFI idle | 🟡 sigma-bus stats |
| `release/distributed` | 🟠 Raft consensus liveness | 🟠 Raft < 100 ms election | 🟡 Network partition chaos test | 🟡 sigma-cloudfs CLI | 🟢 N/A | 🟠 Raft leader metrics |
| `release/dual-boot` | 🟡 NTFS read no corruption | 🟡 Installer < 5 min | 🟡 Partition resize fuzz | 🔴 Guided installer TUI | 🟢 N/A | 🟢 N/A |
| `docs-update` | 🟢 N/A | 🟢 N/A | 🟠 markdownlint, link check | 🔴 5-command onboarding | 🟢 N/A | 🟢 N/A |
| `gh-pages` | 🟢 N/A | 🟡 Page load < 2 s | 🟠 Lighthouse score ≥ 90 | 🔴 Interactive demos | 🟢 N/A | 🟢 N/A |
| `prepare-sigmaos-launch` | 🔴 All Q0 gates pass | 🔴 All P1–P3 targets | 🔴 SBOM, reproducible build | 🔴 All U1–U4 gates | 🟠 Energy profile published | 🟠 Observatory working |

**Priority:** 🔴 Must-have · 🟠 Important · 🟡 Nice-to-have · 🟢 Not applicable

---

## Improvement Tracking: `CURRENT_PROBLEMS_MANIFEST.md` Policy

Every quality, stability, performance, or UX regression must be tracked here:

```bash
# Add a problem:
echo "## Phase Q (Quality) — New Issues" >> CURRENT_PROBLEMS_MANIFEST.md
echo "- [#XXXX] Area: description — file.cpp" >> CURRENT_PROBLEMS_MANIFEST.md

# Required before any PR merges to main:
./scripts/ci_branch_check.sh
./scripts/sigma_automation.sh recovery-check
# CURRENT_PROBLEMS_MANIFEST.md must reflect new status
```

**Release gate:** `CURRENT_PROBLEMS_MANIFEST.md` must have zero open 🔴 items before any `release/*` tag is created.

---

## Quick Reference — All Quality Commands

```bash
# Build + test (all in one)
make check                         # build + unit tests + static analysis

# Stability
make check-stubs                   # count remaining stubs
./tests/chaos/test_rollback.sh     # 3 failed boots → rollback

# Performance
sigma-perf bench pqc               # Kyber/Dilithium ops/sec
sigma-perf bench sched             # context switch latency
sigma-observatory                  # live dashboard

# Quality
./scripts/run_static_analysis.sh   # clang-tidy + cppcheck
./scripts/regression_check.sh      # regression suite

# Security
sigma-sec status                   # security posture
sigma-audit verify                 # verify log chain of custody
sigma-pqc status                   # PQC algorithms + FIPS level

# Release
./scripts/sigma_automation.sh wiki-sync    # mirror docs
./scripts/sigma_automation.sh backup      # source backup
./scripts/ci_branch_check.sh              # branch parity
sigma-release check                # all gates before tagging
```

---

*See also: [Gap Analysis](Gap-Analysis) · [Feature Branch Roadmap](Feature-Branch-Roadmap) · [Branch Development Roadmap](Branch-Development-Roadmap) · [CLI Commands Roadmap](CLI-Commands-Roadmap) · [India Profession Tools Roadmap](India-Profession-Tools-Roadmap) · [Development Roadmap](Development-Roadmap)*
