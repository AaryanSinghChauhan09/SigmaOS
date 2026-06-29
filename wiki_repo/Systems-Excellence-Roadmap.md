# SigmaOS — Systems Excellence Roadmap
## Gaming · Media · IoT · Developer Tools · Package Ecosystem
## Multi-Platform · Update & Recovery · Master Sprint Plan

Continues from [Advanced-Quality-Roadmap](Advanced-Quality-Roadmap).

---

## 1. Gaming & Media Quality

### GM1 — Gaming Stack (sigma-game-layer)

**Current:** `kernel/subsystems/sigma_game_layer.c` — framework only.
`sigma_proton_bridge.cpp` has `mapDxvkSurface()` stub.

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| GameMode scheduler profile | `kernel/subsystems/sigma_game_layer.c` | `performance-optimized` | Boost game process to SCHED_RR, park bg tasks | 
| DXVK D3D9 → Vulkan bridge | `runtime/compat/win32/d3d/sigma_dxvk_d3d9.cpp` | `release/standalone` | Wire `mapDxvkSurface()` stub into full D3D9 device | 
| DXVK D3D11 → Vulkan bridge | `runtime/compat/win32/d3d/sigma_dxvk_d3d11.cpp` | `release/standalone` | ID3D11Device → VkDevice command buffer | 
| vkd3d-proton D3D12 → Vulkan 1.3 | `runtime/compat/win32/d3d/sigma_dxvk_d3d12.cpp` | `release/standalone` | vkd3d-proton path for DX12 titles | 
| XInput gamepad driver | `runtime/compat/win32/sigma_xinput.cpp` | `release/standalone` | USB HID gamepad → XInput API | 
| XAudio2 → sigma-audio bridge | `runtime/compat/win32/sigma_xaudio2.cpp` | `release/standalone` | DirectSound/XAudio2 → HDA PCM output | 
| sigma-gamescope compositor mode | `zenith_desktop/gamescope/sigma_gamescope.cpp` | `release/standalone` | Dedicated gaming compositor: fullscreen, VRR, low-latency | 
| Steam client compatibility | Integration test | `release/standalone` | sigma-wine + DXVK: Steam boots, downloads a game | 
| Proton compatibility layer | `userland/compat/sigma_proton_bridge.cpp` | `release/standalone` | `sigma-wine --proton game.exe` wrapper | 
| Gaming profile (`sigma-cli profile use gaming`) | `userland/tools/sigma_cli.cpp` | `performance-optimized` | Activate: GameMode sched + Vulkan perf + disable audit | 

**Gaming performance targets:**

| Metric | Target | Measurement | 
| -------- | -------- | ------------- | 
| DX9 game frame time | < 16.7 ms (60 FPS) | `sigma-zenith fps` | 
| DX11 game frame time | < 8.3 ms (120 FPS) | `sigma-zenith fps` | 
| Input latency (gamepad) | < 5 ms | XInput poll → GPU frame | 
| Audio latency | < 20 ms | ALSA-equivalent buffer size | 
| Shader compile stall | 0 ms (pre-compiled) | DXVK shader cache | 

### GM2 — Media Stack (sigma-media)

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| HEVC/H.265 hardware decode | `drivers/graphics/sigma_kms.cpp` | `drivers-dev` | Intel Quick Sync / AMD VCN via V4L2-M2M | 
| AV1 decode | `drivers/graphics/sigma_kms.cpp` | `drivers-dev` | AV1 decode via VA-API / DXVA2 equivalent | 
| sigma-video player | `userland/apps/sigma-film/sigma_film.cpp` | `release/standalone` | HEVC/AV1/VP9 playback with hardware decode | 
| Camera capture (V4L2) | `drivers/camera/sigma_v4l2.cpp` | `drivers-dev` | USB webcam → FHIR DiagnosticReport (sigma-health) | 
| CBFC/OTT content tagging | `userland/apps/sigma-film/sigma_film.cpp` | `release/standalone` | IT Rules 2021 age rating, CBFC certificate | 
| Offline MIB media compliance | `userland/apps/sigma-film/sigma_film.cpp` | `release/standalone` | Check OTT IT Rules offline for content creators | 

### GM3 — Audio Quality

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| HDA codec enumeration | `drivers/audio/sigma_hda.cpp` | `drivers-dev` | All nodes via CORB/RIRB command | 
| PCM output stream | `drivers/audio/sigma_hda.cpp` | `drivers-dev` | BDL ring buffer, DMA, interrupt on complete | 
| sigma-audio mixer daemon | `userland/audio/sigma_audio.cpp` | `release/standalone` | Per-app volume, software mix, route to HDA | 
| sigma-bhashini TTS → HDA | `userland/bhashini/sigma_bhashini.cpp` | `release/standalone` | float32 PCM → HDA output | 
| Audio latency test | `tests/perf/bench_audio.sh` | `drivers-dev` | Glitch-free < 20 ms round-trip | 

---

## 2. Embedded & IoT Quality

### EI1 — sigma-twin (Digital Twin)

**Current:** `userland/twin/sigma_twin_iot.cpp` — header only.

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| MQTT broker client | `userland/twin/sigma_twin_iot.cpp` | `release/rtos` | Eclipse Paho MQTT port over sigma-net | 
| Modbus RTU/TCP driver | `drivers/serial/sigma_modbus.cpp` | `release/rtos` | Industrial sensors via RS-485 | 
| OPC-UA client | `userland/twin/sigma_twin_opcua.cpp` | `release/rtos` | Factory floor PLC integration | 
| ISRO Bhuvan satellite data | `userland/twin/sigma_twin_isro.cpp` | `release/standalone` | NDVI, soil moisture from Resourcesat | 
| IoT sensor dashboard | `zenith_desktop/widgets/sigma_iot_widget.cpp` | `release/standalone` | Live sensor readings in Zenith taskbar | 
| Digital twin simulation | `userland/twin/sigma_twin_sim.cpp` | `release/distributed` | OEE calculation from live PLC data | 

### EI2 — sigma-robotics (ROS 2)

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| ROS 2 DDS port | `runtime/ros2/sigma_ros2_dds.cpp` | `release/rtos` | OMG DDS XRCE over sigma-bus | 
| sigma-bus ↔ ROS 2 bridge | `runtime/ros2/sigma_ros2_bridge.cpp` | `release/rtos` | sigma-bus topics ↔ ROS 2 topics | 
| ROS 2 node lifecycle | `runtime/ros2/sigma_ros2_lifecycle.cpp` | `release/rtos` | Lifecycle state machine per ROS 2 node | 
| Hardware abstraction (HAL) | `runtime/ros2/sigma_ros2_hal.cpp` | `release/rtos` | SDF driver → ROS 2 hardware interface | 
| EDF task for ROS 2 callbacks | `kernel/sched/sigma_edf.cpp` | `release/rtos` | RT callback guaranteed deadline | 

### EI3 — sigma-space (IN-SPACe Tools)

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| ISRO NavIC NMEA parser | `drivers/serial/sigma_navic.cpp` | `release/mobile` | NavIC receiver → lat/lon/altitude | 
| NavIC vs GPS comparison | `userland/apps/sigma-agri/sigma_agri.cpp` | `release/mobile` | sigma-agri uses NavIC field boundary | 
| ISRO Bhuvan API client | `userland/indiastack/sigma_isro_client.cpp` | `release/standalone` | Fetch NDVI, DEM, soil raster tiles | 
| IN-SPACe launch tracking | `userland/apps/sigma-gov/sigma_gov.cpp` | `release/standalone` | PSLV/GSLV launch schedule API | 

---

## 3. Developer Tools Depth

### DT1 — sigma-gdb (Debugger)

**New file:** `userland/devtools/sigma_gdb.cpp`

```bash
sigma-gdb ./myapp                    # attach and run
sigma-gdb --pid 1234                 # attach to running process
sigma-gdb --core /sigma/cores/dump   # analyse core dump
(sigma-gdb) break main               # set breakpoint
(sigma-gdb) next                     # step over
(sigma-gdb) step                     # step into
(sigma-gdb) info regs                # register state
(sigma-gdb) x/16x $rsp               # memory examine
(sigma-gdb) bt                       # backtrace via DWARF
(sigma-gdb) watch *ptr               # data watchpoint
(sigma-gdb) disassemble              # disassembly
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| GDB RSP (Remote Serial Protocol) server | `userland/devtools/sigma_gdb.cpp` | `tools-dev` | `sigma-gdb --rsp :1234` for IDE integration | 
| DWARF debug info parser | `userland/devtools/sigma_dwarf.cpp` | `tools-dev` | Map RIP → function + line number | 
| Breakpoint via ptrace equivalent | `userland/devtools/sigma_gdb.cpp` | `kernel-exp` | Syscall `SIGMA_SYS_PTRACE` | 
| Hardware breakpoints (DR0-DR3) | `userland/devtools/sigma_gdb.cpp` | `kernel-exp` | x86-64 debug registers | 
| Core dump analyser | `userland/devtools/sigma_gdb.cpp` | `kernel-exp` | `sigma-gdb --core dump` → backtrace | 
| VS Code / clangd integration | `docs/developer-setup.md` | `docs-update` | GDB RSP → VS Code debugger | 

### DT2 — sigma-perf (Profiler)

**New file:** `userland/tools/sigma_perf_cli.cpp`

```bash
sigma-perf record ./myapp            # sample CPU cycles
sigma-perf record --pid 1234         # sample running process
sigma-perf stat ./myapp              # hardware PMU counters
sigma-perf top                       # live CPU usage by symbol
sigma-perf report                    # annotated profile
sigma-perf flame ./myapp             # flamegraph SVG
sigma-perf bench sched               # context switch latency
sigma-perf bench mem                 # allocator throughput
sigma-perf bench pqc                 # Kyber/Dilithium ops/sec
sigma-perf bench io                  # disk IOPS + latency
sigma-perf bench net                 # network throughput
sigma-perf kpatch status             # live kernel patches
sigma-perf governor show             # P-state + frequency
sigma-perf numa show                 # NUMA topology + stats
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Hardware PMU via `rdpmc` | `userland/tools/sigma_perf_cli.cpp` | `performance-optimized` | Cycles, cache misses, branch mispredictions | 
| Sampling via perf interrupt | `kernel/core/sigma_irq.cpp` | `performance-optimized` | NMI-based sampling at 1 kHz | 
| Flamegraph SVG generator | `userland/tools/sigma_perf_cli.cpp` | `performance-optimized` | Call stack → Brendan Gregg flamegraph SVG | 
| `sigma-perf top` live refresh | `userland/tools/sigma_perf_cli.cpp` | `performance-optimized` | VT100 sorted by CPU% per symbol, 1 s refresh | 
| perf CI benchmark baseline | `.github/workflows/sigma_ci.yml` | `performance-optimized` | Store result JSON; alert if regression > 10% | 

### DT3 — sigma-strace (Syscall Tracer)

```bash
sigma-strace ./myapp                 # trace all syscalls
sigma-strace -e read,write ./myapp   # filter by syscall
sigma-strace -p 1234                 # trace running process
sigma-strace --pqc ./myapp           # include PQC operation trace
sigma-strace --count ./myapp         # count + time per syscall
sigma-strace --json ./myapp > trace.json
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Syscall intercept via ptrace | `userland/devtools/sigma_strace.cpp` | `tools-dev` | Print syscall name + args + return value | 
| PQC operation tracing | `userland/devtools/sigma_strace.cpp` | `tools-dev` | Hook `pqc_sign/verify/encap` via sigma-bus | 
| JSON output mode | `userland/devtools/sigma_strace.cpp` | `tools-dev` | Machine-readable for analysis tooling | 
| Aggregate statistics mode | `userland/devtools/sigma_strace.cpp` | `tools-dev` | Count, total time, min/max per syscall | 

### DT4 — sigma-memcheck (Memory Analyser)

```bash
sigma-memcheck ./myapp               # run with shadow memory
sigma-memcheck --leak-check ./myapp  # detect memory leaks
sigma-memcheck --uaf ./myapp         # detect use-after-free
sigma-memcheck --oob ./myapp         # detect out-of-bounds
sigma-memcheck report <pid>          # report for running process
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Shadow memory (Valgrind-style) | `userland/devtools/sigma_memcheck.cpp` | `tools-dev` | Map every heap alloc to shadow bytes | 
| Use-after-free detection | `userland/devtools/sigma_memcheck.cpp` | `tools-dev` | Poison freed memory, catch access | 
| Leak report on exit | `userland/devtools/sigma_memcheck.cpp` | `tools-dev` | Walk live allocations on `exit()` | 
| AddressSanitizer integration | `Makefile` | `tools-dev` | `-fsanitize=address` build target | 

### DT5 — sigma-sdk (Application SDK)

```bash
# Developer workflow:
sigma-contrib new-app sigma-myapp    # scaffold new profession app
sigma-contrib new-driver sigma-mydrv # scaffold SDF driver
sigma-contrib check                  # lint + test + ABI check
sigma-contrib submit                 # format commit + open PR
sigma-contrib publish                # build .spkg + sign + push to registry
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `sigma-contrib` scaffold tool | `userland/tools/sigma_contrib_cli.cpp` | `tools-dev` | Generate app skeleton with CMakeLists, header, man page | 
| sigma-sdk C++ headers | `include/sigma_sdk.h` | `tools-dev` | Type-safe wrappers: `sigma::fs::open()`, `sigma::net::connect()` | 
| ABI stability checker | `Makefile` | `tools-dev` | `make check-abi` — nm diff of SIGMA_STABLE symbols | 
| `sigma_error.h` standard | `include/sigma_error_codes.h` | `tools-dev` | Consistent `sigma_err_t` return everywhere | 
| India Stack C++ bindings | `include/india/` | `tools-dev` | Type-safe `sigma::abdm::`, `sigma::gst::`, `sigma::upi::` | 
| Doxygen from all headers | `Doxyfile` | `docs-update` | `INPUT = include/ userland/ crypto/` → HTML docs | 

---

## 4. Package Ecosystem Maturity

### PE1 — sigma-repo-server

**Current:** No server exists. `sigma_pkg_registry/` has recipe stubs.

```bash
# sigma-repo-server (Go HTTPS):
sigma-repo-server start --port 8443 --db /sigma/data/packages.db
sigma-repo-server index build        # scan .spkg files, generate index
sigma-repo-server sign               # sign index with ML-DSA-87
sigma-repo-server mirror sync        # sync to NIC CDN mirror
sigma-repo-server stats              # download counts, popular packages
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Go HTTPS server | `sigmad/repo/main.go` | `tools-dev` | Serve `.spkg` + `index.sigma` (Dilithium3-signed) | 
| Package index format | `sigmad/repo/index.go` | `tools-dev` | JSON: name, version, sha256, sig, deps, description | 
| ML-DSA-87 index signature | `sigmad/repo/sign.go` | `tools-dev` | Sign whole index with repo private key | 
| India CDN deployment | CI/infra | `prepare-sigmaos-launch` | `packages.sigmaos.dev` + NIC mirror endpoint | 
| Bootstrap package set (50 pkgs) | `sigma_pkg_registry/recipes/` | `tools-dev` | bash, coreutils, curl, git, Python 3.12, GCC 14, Go 1.23, vim, nano, htop, delta, ripgrep, fzf, jq, wget, tmux, make, cmake, ninja, clang, lldb, musl-libc, node, deno, rust, lua, sqlite3, openssl-tools, zstd, lz4, xz, tar, unzip, gzip, patch, diffutils, findutils, grep, sed, awk, bc, file, lsof, strace | 

### PE2 — .spkg Recipe Quality

```toml
# sigma_pkg_registry/recipes/sigma-agri.recipe
[package]
name        = "sigma-agri"
version     = "1.0.0"
description = "Agriculture tools for Indian farmers"
maintainer  = "sigma-team@sigmaos.dev"
license     = "GPL-2.0-or-later"

[source]
url     = "https://github.com/AaryanSinghChauhan09/SigmaOS"
commit  = "abc123def"           # exact commit — no floating refs
sha256  = "deadbeef..."

[build]
system  = "cmake"
flags   = ["-DSIGMA_PROFILE=standalone"]
targets = ["sigma-agri"]

[install]
bin     = ["sigma-agri"]
man     = ["docs/man/sigma-agri.1"]
data    = ["userland/apps/sigma-agri/data/msp_2025.db"]

[depends]
runtime = ["sigma-net", "sigma-sqlite"]
build   = ["cmake", "ninja", "clang"]

[signature]
algorithm  = "ML-DSA-87"
public_key = "sigma-team.mldsa87.pub"
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Recipe format spec | `sigma_pkg_registry/RECIPE_FORMAT.md` | `tools-dev` | Document TOML schema + validation rules | 
| Recipe linter | `userland/sigma-pkg/sigma_pkg_lint.cpp` | `tools-dev` | Validate recipe: required fields, exact deps, valid SPDX | 
| Recipes for all 55 profession apps | `sigma_pkg_registry/recipes/` | `tools-dev` | One `.recipe` file per app | 
| Recipe CI validation | `.github/workflows/sigma_ci.yml` | `tools-dev` | All recipes pass linter on every PR | 
| Community recipe guide | `wiki_repo/Package-Recipe-Guide.md` | `docs-update` | Step-by-step guide for community contributors | 
| Delta update support | `userland/sigma-pkg/sigma_pkg_delta.cpp` | `tools-dev` | Binary diff: only download changed blocks | 
| Auto-update daemon | `userland/daemons/sigma_pkgd.cpp` | `tools-dev` | Background check every 24 h; notify user | 

### PE3 — Package Security

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Quarantine on failed verify | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Move to `/sigma/quarantine/`, never execute | 
| Package history log | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Log every install/remove to sigma-audit | 
| Rollback to previous version | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Keep one previous version, `sigma-pkg rollback` | 
| Transitive dependency scan | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Check all deps for known CVEs via local DB | 
| Canary release channel | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | `--channel canary` for testing pre-release | 

---

## 5. Update & Recovery Systems

### UR1 — A/B Atomic Updates

**Current:** Rollback gate wired in `sigma_kernel_main.c`. No OTA update daemon.

```bash
sigma-update status                  # current slot, pending update
sigma-update check                   # fetch update manifest
sigma-update download                # download to inactive slot
sigma-update apply                   # mark inactive slot for next boot
sigma-update verify                  # verify Dilithium3 sig before apply
sigma-update rollback                # revert to previous slot
sigma-update history                 # list past updates with timestamps
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| OTA update daemon | `userland/daemons/sigma_updated.cpp` | all | Background check, download to B-slot | 
| Update manifest format | `sigmad/update/manifest.go` | all | JSON: version, sha256, sig, delta URL | 
| A/B slot manager | `kernel/core/sigma_ab_update.cpp` | `kernel-exp` | Read/write `SigmaBootSlot` EFI variable | 
| Delta update (binary diff) | `sigmad/update/delta.go` | all | Only transmit changed blocks (xdelta3) | 
| Atomic signature verification | `userland/daemons/sigma_updated.cpp` | all | Verify ML-DSA-87 sig before marking slot bootable | 
| Forced update for CVEs | `userland/daemons/sigma_updated.cpp` | all | CERT-In CVE → mandatory update flag | 
| sigma-fleet OTA push | `userland/daemons/sigma_fleet_agent.cpp` | `release/cloud` | Fleet server pushes update to all managed devices | 
| Update CI test | `tests/integration/test_ota_update.sh` | all | Download + apply + boot new slot in QEMU | 

### UR2 — Recovery & Resilience

**Current:** Rollback counter + resilient fallback shell exist. Fix-it menu partial.

```bash
# Recovery scenarios and commands:
sigma-recovery start                 # launch text-mode recovery menu
sigma-recovery rollback              # revert to last known-good boot
sigma-recovery fsck                  # check + repair all filesystems
sigma-recovery reinstall             # reinstall current version from ISO
sigma-recovery import-backup <file>  # restore from sigma-automation backup
sigma-recovery diagnose              # AI-powered crash analysis
sigma-recovery network               # minimal networking (fetch updates)
sigma-recovery shell                 # drop to recovery sigma-sh
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Fix-it menu full implementation | `kernel/core/boot/sigma_boot_recovery_menu.c` | all | Text-mode menu: rollback/fsck/reinstall/shell | 
| sigma-heal + sigma-ai crash diagnosis | `userland/ai/sigma_heal_ai.cpp` | `release/standalone` | Feed panic log → actionable fix suggestion | 
| Recovery network stack | `kernel/resilience/sigma_micro_fallback.cpp` | all | Minimal TCP for fetching updates in recovery | 
| Backup import in recovery | `kernel/core/boot/sigma_boot_recovery_menu.c` | all | Mount backup archive, restore key files | 
| Recovery CI test | `tests/chaos/test_rollback.sh` | all | 3 failed boots → auto-recovery → boots clean | 
| sigma-recover CLI | `userland/tools/sigma_recover_cli.cpp` | all | Full CLI for all recovery operations | 

### UR3 — Resilience Patterns

| Pattern | Implementation | Branch | Detail | 
| --------- | --------------- | -------- | -------- | 
| Circuit breaker for India Stack APIs | `userland/indiastack/sigma_circuit_breaker.cpp` | `release/standalone` | On 3 consecutive API failures → switch to offline mode | 
| Retry with exponential backoff | `userland/indiastack/sigma_retry.cpp` | `release/standalone` | API calls: 1 s, 2 s, 4 s, 8 s backoff | 
| Offline-first data sync (CRDT) | `net/sigma_offline_sync.cpp` | `release/distributed` | Last-write-wins for profession app data | 
| Graceful degradation | All profession apps | `release/standalone` | If API fails → show cached data + "last updated" notice | 
| Dead-letter queue | `userland/daemons/sigma_queue.cpp` | `release/cloud` | Failed sigma-bus messages → retry queue | 
| Self-healing driver restart | `kernel/diagnostics/sigma_crash_reporter.cpp` | `drivers-dev` | SDF driver crash → sigma-heal restarts in < 500 ms | 

---

## 6. Multi-Platform Quality

### MP1 — x86-64 Quality Targets

| Target | Metric | Branch | CI | 
| -------- | -------- | -------- | ---- | 
| QEMU boot time | < 2 s | `kernel-exp` | `bench_boot.sh` | 
| Real hardware boot (ThinkPad) | < 5 s | `prepare-sigmaos-launch` | Physical CI runner | 
| RAM idle (Zenith desktop) | < 150 MB | `release/standalone` | `sigma-mem stats` | 
| Context switch p99 | < 50 ns | `performance-optimized` | `bench_sched.cpp` | 
| Kyber-1024 (AVX-512) | ≥ 5.8 M ops/sec | `performance-optimized` | `bench_pqc.cpp` | 

### MP2 — ARM64 Quality Targets

| Target | Device | Metric | Branch | 
| -------- | -------- | -------- | -------- | 
| Raspberry Pi 4 boot | BCM2711 | < 10 s | `release/mobile` | 
| Raspberry Pi 5 boot | BCM2712 | < 8 s | `release/mobile` | 
| Pi Zero 2W idle | sigma-ultra | < 0.4 W | `release/mobile` | 
| JioBook Wi-Fi connect | mt7921 | < 10 s assoc + DHCP | `release/mobile` | 
| Neon Kyber-1024 | Cortex-A76 | ≥ 2.1 M ops/sec | `release/mobile` | 
| Hindi ASR latency | ARM | < 500 ms | `release/standalone` | 

### MP3 — RISC-V Quality Targets

| Target | Device | Metric | Branch | 
| -------- | -------- | -------- | -------- | 
| VisionFive 2 boot | JH7110 | < 30 s | `release/mobile` | 
| Basic CLI (`sigma-sh`) | RISC-V | Works | `release/mobile` | 
| sigma-agri MSP offline | RISC-V | < 1 s | `release/mobile` | 

### MP4 — Cross-Platform CI Matrix

```yaml
# .github/workflows/sigma_qemu.yml — target matrix
strategy:
  matrix:
    arch: [x86_64, aarch64, riscv64]
    profile: [microkernel, standalone, cloud]
    exclude:
      - arch: riscv64
        profile: standalone   # too slow for RISC-V CI
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| ARM64 QEMU CI | `.github/workflows/sigma_qemu.yml` | `release/mobile` | `qemu-system-aarch64 -M virt -cpu cortex-a76` | 
| RISC-V QEMU CI | `.github/workflows/sigma_qemu.yml` | `release/mobile` | `qemu-system-riscv64 -M virt` | 
| Cross-compile ARM64 on x86 | `toolchain-aarch64-elf.cmake` | `release/mobile` | `aarch64-linux-gnu-gcc` cross toolchain | 
| RISC-V toolchain | `toolchain-riscv64-elf.cmake` | `release/mobile` | `riscv64-linux-gnu-gcc` cross toolchain | 
| Arch-neutral kernel layer | `kernel/core/sigma_kernel_main.c` | `kernel-exp` | `#ifdef SIGMA_ARCH_X86_64` / `SIGMA_ARCH_ARM64` | 

---

## 7. Master Sprint Plan

### Sprint 0 (Now → Month 1): Make It Boot

**Goal:** `qemu-system-x86_64 -cdrom SigmaOS.iso` reaches a shell.

| # | Task | Owner branch | Done when | 
| --- | ------ | ------------- | ----------- | 
| 0.1 | Round-robin scheduler body | `kernel-exp` | 2 tasks interleave in QEMU serial | 
| 0.2 | Buddy allocator connected to VMM | `kernel-exp` | `sigma_malloc(4096)` returns valid pointer | 
| 0.3 | x86-64 page table walker | `kernel-exp` | Map 1 MB region, read back correct bytes | 
| 0.4 | 30-syscall dispatch table | `kernel-exp` | `write(1,"ok\n",3)` from userland | 
| 0.5 | APIC timer → jiffies | `kernel-exp` | `sleep(100ms)` works | 
| 0.6 | VESA/GOP framebuffer | `drivers-dev` | Pixels visible in QEMU | 
| 0.7 | sigma-boot.efi UEFI loader | `kernel-exp` | QEMU boots via EFI without GRUB | 
| 0.8 | `make iso` → bootable ISO | `kernel-exp` | `qemu -cdrom SigmaOS.iso` → sigma-login | 
| 0.9 | QEMU CI real test (not echo) | all | GitHub Actions passes with real QEMU | 
| 0.10 | Argon2id CryptFS (#44) | `kernel-exp` | `derive_key()` returns real hash | 

### Sprint 1 (Month 1–3): Make It Connect

| # | Task | Owner branch | Done when | 
| --- | ------ | ------------- | ----------- | 
| 1.1 | e1000 DMA TX/RX rings | `drivers-dev` | `ping 10.0.2.2` succeeds in QEMU | 
| 1.2 | TCP state machine | `drivers-dev` | `nc` can make TCP connection | 
| 1.3 | UDP socket layer | `drivers-dev` | DNS query works | 
| 1.4 | DHCP client | `drivers-dev` | IP auto-assigned on boot | 
| 1.5 | sigma-repo-server (Go) | `tools-dev` | `sigma-pkg install vim` works | 
| 1.6 | VFS open/read/write bodies | `fs-dev` | Shell can `cat`, `echo >`, `ls` on tmpfs | 
| 1.7 | sigma-sh TTY read | `tools-dev` | Interactive shell responds to keyboard | 
| 1.8 | musl-libc bundle | `tools-dev` | Dynamic C programs run via sigma-compat | 
| 1.9 | ARP cache | `drivers-dev` | `ping google.com` resolves + reaches host | 
| 1.10 | TLS 1.3 (PQC-hybrid) | `drivers-dev` | HTTPS request to GSTN sandbox | 

### Sprint 2 (Month 3–6): Make It Visible

| # | Task | Owner branch | Done when | 
| --- | ------ | ------------- | ----------- | 
| 2.1 | VirtIO-GPU real DMA | `drivers-dev` | Zenith renders desktop frame | 
| 2.2 | DRM/KMS layer | `drivers-dev` | Native resolution on real display | 
| 2.3 | Compositor `composite_window()` | `release/standalone` | Windows alpha-blend correctly | 
| 2.4 | Input event loop | `release/standalone` | Keyboard + mouse control Zenith | 
| 2.5 | App launcher | `release/standalone` | Super key → fuzzy search apps | 
| 2.6 | sigma-bhashini ASR offline | `release/standalone` | Voice → text in Hindi | 
| 2.7 | Indian IME (Inscript) | `release/standalone` | Type Hindi in any text field | 
| 2.8 | DID login screen | `release/standalone` | QR → ABHA → desktop opens | 
| 2.9 | sigma-ai llama.cpp | `release/standalone` | `sigma-ai ask "explain GSTR-3B"` | 
| 2.10 | sigma-ca GST compute live | `release/standalone` | Compute + file GSTR-1 on GSTN sandbox | 

### Sprint 3 (Month 6–12): Make It Indian

| # | Task | Owner branch | Done when | 
| --- | ------ | ------------- | ----------- | 
| 3.1 | ABDM FHIR client | `release/standalone` | Create ABHA, push FHIR record | 
| 3.2 | GST IRN + e-Way Bill live | `release/standalone` | CA generates IRN on GSTN | 
| 3.3 | UPI pay + collect | `release/standalone` | `sigma-upi pay` completes transaction | 
| 3.4 | MGNREGS attendance API | `release/mobile` | Panchayat marks attendance | 
| 3.5 | sigma-legal eCourts live | `release/standalone` | Pull case status via eCourts API | 
| 3.6 | Real Kyber NTT (liboqs) | `performance-optimized` | ≥ 5.8 M ops/sec on AVX-512 | 
| 3.7 | sigma-pod kernel enforcement | `release/cloud` | cgroup OOM kills at exact limit | 
| 3.8 | WCAG 2.2 AA | `release/standalone` | aXe CI scan passes | 
| 3.9 | 22-language UI strings | `release/standalone` | 0 untranslated strings in 6 languages | 
| 3.10 | sigma-fleet 100 devices | `release/cloud` | Fleet manages 100 QEMU instances | 

### Sprint 4 (Month 12–18): Make It Trusted

| # | Task | Owner branch | Done when | 
| --- | ------ | ------------- | ----------- | 
| 4.1 | sigma-boot.efi + TPM2 PCR | `kernel-exp` | TPM2 seals CryptFS key to PCR | 
| 4.2 | ML-DSA FIPS 204 final | `performance-optimized` | All packages signed with FIPS 204 | 
| 4.3 | sigma-mac enforced every syscall | `kernel-exp` | Every `open()` checked against policy | 
| 4.4 | sigma-ids live | `release/cloud` | Anomaly detected, sigma-heal isolates | 
| 4.5 | Reproducible build verified | all | Two CI builds → identical SHA256 | 
| 4.6 | Physical hardware CI | `prepare-sigmaos-launch` | RPi 4 + ThinkPad pass all 9 checks | 
| 4.7 | sigma-wine W2 (Python CLI) | `tools-dev` | `sigma-wine python.exe -c "print('hi')"` | 
| 4.8 | OTA A/B update working | all | Download + apply + rollback in QEMU | 
| 4.9 | CERT-In compliance export | `release/cloud` | sigma-siem generates 6-hour report | 
| 4.10 | Zero critical bugs | all | CURRENT_PROBLEMS 🔴 count = 0 | 

---

## Quick Reference: All Roadmap Documents

| Document | Key sections | Lines | 
| ---------- | ------------- | ------- | 
| [Quality-Stability-Performance-Roadmap](Quality-Stability-Performance-Roadmap) | S1-S4, P1-P6, Q1-Q5, U1-U6, SE1-SE2, A1-A2, D1-D4 | ~1,000 | 
| [Stability-Performance-Extended](Stability-Performance-Extended) | E1-E2, R1-R3, O1-O2, RE1-RE2, NR1-NR2, IQ1-IQ3, TI1-TI3, HC1-HC2, LE1-LE3 | ~900 | 
| [Compatibility-Automation-Personalisation-Roadmap](Compatibility-Automation-Personalisation-Roadmap) | C1-C5, A1-A5, K1-K4, P1-P5 | ~700 | 
| [Advanced-Quality-Roadmap](Advanced-Quality-Roadmap) | SH1-SH5, NS1-NS3, EF1-EF4, AI1-AI3, I18N1-I18N3, EDU1, RU1-RU2, CE1-CE3 | ~700 | 
| [Systems-Excellence-Roadmap](Systems-Excellence-Roadmap) | GM1-GM3, EI1-EI3, DT1-DT5, PE1-PE3, UR1-UR3, MP1-MP4, Sprint 0-4 | ~700 | 

**Total roadmap content: ~4,000 lines across 5 documents.**

---

*See also: [Advanced Quality Roadmap](Advanced-Quality-Roadmap) · [Branch Development Roadmap](Branch-Development-Roadmap) · [Feature Branch Roadmap](Feature-Branch-Roadmap) · [CLI Commands Roadmap](CLI-Commands-Roadmap) · [Windows Compatibility Layer Roadmap](Windows-Compatibility-Layer-Roadmap) · [India Profession Tools Roadmap](India-Profession-Tools-Roadmap)*
