# SigmaOS — CLI Commands Development Roadmap (Per Branch)

Complete, file-level roadmap for every CLI command across all 19 branches.
Grounded in the actual June 2026 codebase state.

---

## Current CLI State (Ground Truth)

### What exists and works today

| Command | File | Current state |
|---------|------|---------------|
| `sigma-cli profile list/show/use` | `userland/tools/sigma_cli.cpp` | ✅ Real |
| `sigma-cli alias list/add` | `userland/tools/sigma_cli.cpp` | ✅ Real |
| `sigma-cli update/backup/sync/recovery/branch-check` | `userland/tools/sigma_cli.cpp` | ⚠️ Prints host script path only |
| `sigma-cli automation <cmd>` | `userland/tools/sigma_cli.cpp` | ⚠️ Prints host script path only |
| `sigma-cli game-mode` | `userland/tools/sigma_cli.cpp` | ⚠️ Prints stub message |
| `sigma-automation.sh backup/update/recovery-check/wiki-sync/meta-check` | `scripts/sigma_automation.sh` | ✅ Real (host-side) |
| `sigma-git-sync.sh` | `scripts/sigma_git_sync.sh` | ✅ Real |
| `ci_branch_check.sh` | `scripts/ci_branch_check.sh` | ✅ Real |
| `sigma_cli_host.sh update/backup/sync/branch-check/profile` | `scripts/sigma_cli_host.sh` | ✅ Real (host wrapper) |
| `sigma-pod create/start/stop/ps/destroy` | `userland/tools/sigma_pod_cli.cpp` | ⚠️ IPC stubs |
| `sigma-sh builtins (cd/export/alias/echo/history/pwd/exit)` | `userland/shell/sigma_shell.cpp` | ✅ Real (no TTY) |

### What is referenced but not yet implemented

`sigma-pkg`, `sigma-net`, `sigma-sec`, `sigma-heal`, `sigma-wine`,
`sigma-boot`, `sigma-perf`, `sigma-fleet`, `sigma-ai`, `sigma-monitor`,
`sigma-fsck`, `sigma-kreg`, `sigma-audit`, `sigma-trust`, `sigma-recovery`

---

## Unified CLI Architecture (Target)

All commands flow through a single surface:

```
sigma-cli <subsystem> <verb> [flags]   ← primary entry point
sigma-<tool> [flags]                   ← direct shortcuts
sigma-sh                               ← login shell (REPL)
scripts/sigma_*.sh                     ← host/dev automation
```

Every command follows: **`sigma-<tool> <verb> [options]`**
Every command must have: `--help`, `--json` (machine output), `--dry-run` (safe preview).

---

## `tools-dev` — Core CLI Commands

This is the primary CLI development branch. Everything here flows to `main`.

### sigma-cli (expand existing)

**File:** `userland/tools/sigma_cli.cpp`
**Current:** profile/alias/update/backup/sync work as print stubs. No real IPC.

```
sigma-cli profile list                  # list profiles [✅ real]
sigma-cli profile show                  # show active   [✅ real]
sigma-cli profile use <name>            # set active    [✅ real]
sigma-cli profile create <name>         # add new profile    [❌ build]
sigma-cli profile edit <name>           # edit ~/.sigma_profile [❌ build]
sigma-cli profile export <name> <file>  # export as JSON [❌ build]
sigma-cli profile import <file>         # import from JSON [❌ build]

sigma-cli alias list                    # [✅ real]
sigma-cli alias add <name> <cmd>        # [✅ real]
sigma-cli alias remove <name>           # [❌ build]
sigma-cli alias show <name>             # [❌ build]

sigma-cli pkg install <name>            # → sigma-pkg install [❌ needs sigma-pkg]
sigma-cli pkg remove <name>             # → sigma-pkg remove  [❌ needs sigma-pkg]
sigma-cli pkg list                      # → sigma-pkg list    [❌ needs sigma-pkg]
sigma-cli pkg update                    # → sigma-pkg upgrade [❌ needs sigma-pkg]
sigma-cli pkg search <query>            # → sigma-pkg search  [❌ needs sigma-pkg]

sigma-cli pod run <spkg>                # → sigma-pod run     [❌ wire real IPC]
sigma-cli pod list                      # → sigma-pod ps      [❌ wire real IPC]
sigma-cli pod stop <id>                 # → sigma-pod stop    [❌ wire real IPC]
sigma-cli pod logs <id>                 # → sigma-pod logs    [❌ build]

sigma-cli wine exec <exe> [args]        # → sigma_wine_exec() [❌ needs kernel]
sigma-cli wine prefix create <path>     # → sigma_wine_create_prefix() [❌ build]
sigma-cli wine prefix list              # list prefixes       [❌ build]
sigma-cli wine info <exe>               # PE header dump      [✅ sigma_pe_inspect() works]

sigma-cli net status                    # → sigma-net status  [❌ needs TCP stack]
sigma-cli net connect <ssid> <psk>      # → sigma-net connect [❌ needs Wi-Fi driver]
sigma-cli net disconnect                # [❌ build]
sigma-cli net ping <host>               # ICMP ping           [⚠️ ICMP stub exists]

sigma-cli sec verify                    # boot chain integrity check [❌ build]
sigma-cli sec audit                     # show audit log      [❌ build]
sigma-cli sec status                    # PQC/MAC policy      [❌ build]

sigma-cli health check                  # sigma-heal status   [❌ build]
sigma-cli health log                    # show crash log      [❌ build]
sigma-cli health diagnose               # ai crash analysis   [❌ build]

sigma-cli boot rollback                 # write rollback EFI flag [⚠️ needs EFI var write]
sigma-cli boot status                   # show A/B slot state [❌ build]
sigma-cli boot recovery                 # enter recovery menu [⚠️ stub message exists]

sigma-cli update                        # [⚠️ prints script path — wire real]
sigma-cli backup                        # [⚠️ prints script path — wire real]
sigma-cli sync                          # [⚠️ prints script path — wire real]
sigma-cli branch-check                  # [⚠️ prints script path — wire real]
sigma-cli game-mode                     # [⚠️ stub — wire to sched IPC]

sigma-cli ai ask "<prompt>"             # → sigma-ai daemon   [❌ needs LLM]
sigma-cli ai lang <code>                # set language (hi/ta/te...) [❌ build]

sigma-cli perf report                   # scheduler stats     [❌ build]
sigma-cli perf bench                    # run benchmarks      [❌ build]
sigma-cli perf top                      # live CPU/mem view   [❌ build]
```

**Implementation tasks:**

| Task | File | Detail |
|------|------|--------|
| Wire `update/backup/sync` to real sigma-bus IPC | `userland/tools/sigma_cli.cpp` | Replace `sys_print` stubs with `sigma_bus_call()` |
| Add `profile create/edit/export/import` | `userland/tools/sigma_cli.cpp` | Write to `~/.sigma_profile` via VFS |
| Add `alias remove/show` | `userland/tools/sigma_cli.cpp` | Remove from `g_aliases[]` array |
| Add `pkg` subcommands (proxy to sigma-pkg) | `userland/tools/sigma_cli.cpp` | fork + exec `sigma-pkg` binary |
| Add `pod` subcommands (proxy to sigma-pod) | `userland/tools/sigma_cli.cpp` | fork + exec `sigma-pod` binary |
| Add `wine` subcommands | `userland/tools/sigma_cli.cpp` | Call `sigma_wine_exec()` / `sigma_wine_inspect()` |
| Add `ai ask` → sigma-ai IPC | `userland/tools/sigma_cli.cpp` | sigma-bus call to sigma-ai daemon |
| Add `--json` output flag | `userland/tools/sigma_cli.cpp` | Print JSON for all subcommands |
| Add `--dry-run` flag | `userland/tools/sigma_cli.cpp` | Show what would happen without doing it |
| Add `--help` per-subcommand | `userland/tools/sigma_cli.cpp` | Per-verb help strings |

---

### sigma-sh (shell enhancements)

**File:** `userland/shell/sigma_shell.cpp`
**Current:** Parser real, builtins real, no TTY read.

```
sigma-sh                        # interactive REPL    [⚠️ no TTY read]
sigma-sh <script.sh>            # run script file     [❌ build]
sigma-sh -c "<command>"         # inline exec         [✅ via sigma_shell_exec()]
sigma-sh --login                # login shell mode    [❌ build]

# Shell builtins:
cd [dir]                        # [✅ real]
export NAME=VALUE               # [✅ real]
alias name=cmd                  # [✅ real]
history [n]                     # [✅ real]
pwd                             # [✅ real]
echo [args]                     # [✅ real]
exit [code]                     # [✅ real]
source <file>                   # [❌ build]
which <cmd>                     # [❌ build]
type <cmd>                      # [❌ build]
set / unset                     # [❌ build]
jobs / fg / bg                  # [❌ build]
kill <pid>                      # [❌ build]
```

**Implementation tasks:**

| Task | File | Detail |
|------|------|--------|
| Connect TTY via `sigma_sys_read(0,...)` | `userland/shell/sigma_shell.cpp` | Replace empty `line[0] = '\0'` |
| `source <file>` builtin | `userland/shell/sigma_shell.cpp` | Open file, execute each line |
| `which <cmd>` — PATH search | `userland/shell/sigma_shell.cpp` | Walk `PATH` env dirs via VFS |
| `type <cmd>` — builtin/alias/external | `userland/shell/sigma_shell.cpp` | Check builtins[], g_aliases[], then PATH |
| `jobs/fg/bg` job control | `userland/shell/sigma_shell.cpp` | Track PIDs in `g_jobs[]` table |
| `kill <pid>` | `userland/shell/sigma_shell.cpp` | Emit `sigma_sys_kill(pid, SIGTERM)` |
| Script mode `sigma-sh script.sh` | `userland/shell/sigma_shell.cpp` | Open file fd, loop `vfs_read` |
| Ctrl+C / Ctrl+D signal handling | `userland/shell/sigma_shell.cpp` | SIGINT → clear line; SIGEOF → exit |
| Fish-style tab completion (real) | `userland/shell/sigma_shell.cpp` | Call `vfs_readdir(PWD)` on TAB |
| Syntax highlighting (VT100 escape codes) | `userland/shell/sigma_shell.cpp` | Colorize keywords on render |

---

### sigma-automation.sh (expand)

**File:** `scripts/sigma_automation.sh`
**Current:** `backup/update/recovery-check/meta-check/wiki-sync` all real.

```bash
sigma_automation.sh backup              # [✅ real — tarball]
sigma_automation.sh update              # [✅ real — fetch + wiki-sync]
sigma_automation.sh update-check        # [✅ real — git status]
sigma_automation.sh recovery-check      # [✅ real — file existence check]
sigma_automation.sh meta-check          # [✅ real — Phase C file scan]
sigma_automation.sh wiki-sync           # [✅ real — copy docs to wiki_repo]
sigma_automation.sh release             # [❌ build — tag + sign + publish]
sigma_automation.sh sign-release        # [❌ build — Dilithium3 sign ISO]
sigma_automation.sh qemu-test           # [❌ build — boot ISO in QEMU, verify]
sigma_automation.sh perf-bench          # [❌ build — run benchmark suite]
sigma_automation.sh fuzz-pqc            # [⚠️ scripts/fuzz_pqc.sh exists]
sigma_automation.sh gen-changelog       # [⚠️ scripts/gen_changelog.sh exists]
sigma_automation.sh lint                # [⚠️ scripts/run_static_analysis.sh exists]
```

**Add to `sigma_automation.sh`:**

```bash
release)   # Tag HEAD, sign ISO with pqc_sign, upload to releases
  cmd_release ;;
sign-release)  # sigma_sign_release.sh wrapper
  "${ROOT}/scripts/sign_release.sh" ;;
qemu-test)  # Boot sigmaos.iso in QEMU, assert reaches prompt
  cmd_qemu_test ;;
perf-bench)  # Run benchmark suite, output markdown table
  cmd_perf_bench ;;
```

---

## `kernel-exp` — Kernel CLI Commands

These are commands that require a bootable kernel. None can work until Phase 0.

### sigma-sched (scheduler diagnostics)

**New file:** `userland/tools/sigma_sched_cli.cpp`

```
sigma-sched show           # print runqueue state per CPU
sigma-sched top            # live task CPU usage (like htop, text mode)
sigma-sched set-policy <pid> <rr|fifo|deadline> <priority>
sigma-sched budget <tid> <budget_ns> <period_ns>   # MCS budget
sigma-sched affinity <pid> <cpu_mask>              # NUMA affinity
sigma-sched stress --tasks 64 --duration 10s       # scheduler stress test
```

| Task | File | Detail |
|------|------|--------|
| `sigma-sched show` | `userland/tools/sigma_sched_cli.cpp` | Read `/proc/sigma/sched` via VFS |
| `sigma-sched top` (live) | `userland/tools/sigma_sched_cli.cpp` | VT100 cursor control, refresh every 1 s |
| `sigma-sched set-policy` | `userland/tools/sigma_sched_cli.cpp` | Syscall `SIGMA_SYS_SCHED_SETPOLICY(pid, policy, prio)` |
| `sigma-sched budget` | `userland/tools/sigma_sched_cli.cpp` | Call `sigma_mcs_register()` via syscall |

### sigma-mem (memory diagnostics)

**New file:** `userland/tools/sigma_mem_cli.cpp`

```
sigma-mem stats            # buddy allocator free/used per order
sigma-mem map <pid>        # show VMM address space for PID
sigma-mem leak-check <pid> # detect memory leaks (shadow allocator)
sigma-mem compact          # trigger O(1) slab compaction
sigma-mem pressure         # simulate memory pressure event
```

| Task | File | Detail |
|------|------|--------|
| `sigma-mem stats` | `userland/tools/sigma_mem_cli.cpp` | Call `sigma_mem_stats()` via syscall |
| `sigma-mem map <pid>` | `userland/tools/sigma_mem_cli.cpp` | Read `/proc/<pid>/maps` via VFS |
| `sigma-mem compact` | `userland/tools/sigma_mem_cli.cpp` | Syscall `SIGMA_SYS_MEM_COMPACT` |

### sigma-irq (interrupt diagnostics)

**New file:** `userland/tools/sigma_irq_cli.cpp`

```
sigma-irq list             # show IRQ → handler mapping
sigma-irq stats            # IRQ hit counts per vector
sigma-irq latency          # measure IRQ handler latency (RDTSC)
sigma-irq affinity <irq> <cpu>   # pin IRQ to CPU
```

### sigma-boot (boot management)

**New file:** `userland/tools/sigma_boot_cli.cpp`

```
sigma-boot status          # show A/B slot state, PCR measurements
sigma-boot rollback        # write rollback flag to EFI variable
sigma-boot verify          # verify kernel + initramfs Dilithium3 sig
sigma-boot safe-mode       # set next-boot to safe/minimal mode
sigma-boot slots           # list available boot slots
sigma-boot commit          # mark current boot as known-good
```

| Task | File | Detail |
|------|------|--------|
| `sigma-boot status` | `userland/tools/sigma_boot_cli.cpp` | Read EFI variables + TPM PCR via syscall |
| `sigma-boot rollback` | `userland/tools/sigma_boot_cli.cpp` | Write `SigmaBootSlot=B` EFI variable |
| `sigma-boot verify` | `userland/tools/sigma_boot_cli.cpp` | Call `pqc_verify()` on kernel blob |
| `sigma-boot commit` | `userland/tools/sigma_boot_cli.cpp` | Call `sigma_rollback_mark_boot_successful()` |

---

## `drivers-dev` — Driver CLI Commands

### sigma-drv (driver management)

**New file:** `userland/tools/sigma_drv_cli.cpp`

```
sigma-drv list             # list loaded SDF drivers + state
sigma-drv load <name>      # load a driver from /sigma/drivers/
sigma-drv unload <name>    # unload (sigma-heal restarts if crashed)
sigma-drv probe <pci_id>   # run probe() on specific PCI device
sigma-drv log <name>       # show driver log ring buffer
sigma-drv bench <name>     # run driver throughput benchmark
sigma-drv reload <name>    # unload + load (hot-swap)
```

| Task | File | Detail |
|------|------|--------|
| `sigma-drv list` | `userland/tools/sigma_drv_cli.cpp` | Read `/proc/sigma/drivers` via VFS |
| `sigma-drv load/unload` | `userland/tools/sigma_drv_cli.cpp` | Syscall `SIGMA_SYS_DRV_LOAD/UNLOAD` |
| `sigma-drv probe` | `userland/tools/sigma_drv_cli.cpp` | Call `driver.probe(dev)` via kernel IPC |
| `sigma-drv log <name>` | `userland/tools/sigma_drv_cli.cpp` | Read driver ring buffer from kernel |

### sigma-gpu (GPU / display)

**New file:** `userland/tools/sigma_gpu_cli.cpp`

```
sigma-gpu info             # show detected GPU + VRAM + DRM connectors
sigma-gpu modes            # list available display modes
sigma-gpu set-mode <WxH@Hz>   # apply display mode via KMS
sigma-gpu benchmark        # run Vulkan compute benchmark
sigma-gpu screenshot <file>   # capture framebuffer to PNG
sigma-gpu power <auto|max|min>  # set GPU P-state
```

### sigma-audio (audio)

**New file:** `userland/tools/sigma_audio_cli.cpp`

```
sigma-audio list           # list HDA codec nodes
sigma-audio play <file>    # play WAV/PCM file
sigma-audio record <file>  # record from mic
sigma-audio volume <0-100> # set master volume
sigma-audio test           # play 1 kHz test tone
```

### sigma-net (networking)

**File:** `userland/tools/sigma_net_cli.cpp` (new)
**Current:** `sigma-cli net status` exists as stub. ICMP exists in kernel.

```
sigma-net status           # show interfaces, IPs, link state
sigma-net up <iface>       # bring up interface
sigma-net down <iface>     # bring down interface
sigma-net connect <ssid> <psk>   # Wi-Fi connect (WPA3)
sigma-net disconnect       # Wi-Fi disconnect
sigma-net scan             # scan for Wi-Fi networks
sigma-net dhcp <iface>     # request DHCP lease
sigma-net ip set <iface> <addr/prefix>  # set static IP
sigma-net route list       # show routing table
sigma-net route add <prefix> via <gw>
sigma-net dns set <server> # set DNS resolver
sigma-net ping <host> [-c count]    # ICMP ping  [⚠️ ICMP exists]
sigma-net traceroute <host>         # hop-by-hop trace
sigma-net capture <iface> [--count n]  # packet capture (forensic profile)
sigma-net firewall list    # show sigma-firewall rules
sigma-net firewall allow <rule>
sigma-net firewall deny <rule>
```

| Task | File | Blocked by |
|------|------|------------|
| `sigma-net status` | `userland/tools/sigma_net_cli.cpp` | ARP + DHCP in kernel |
| `sigma-net connect` | `userland/tools/sigma_net_cli.cpp` | iwlwifi/mt7921 driver |
| `sigma-net ping` | `userland/tools/sigma_net_cli.cpp` | ICMP exists — wire CLI to it |
| `sigma-net firewall *` | `userland/tools/sigma_net_cli.cpp` | sigma-firewall kernel module |

---

## `fs-dev` — Filesystem CLI Commands

### sigma-fs (filesystem management)

**New file:** `userland/tools/sigma_fs_cli.cpp`

```
sigma-fs list              # list mounted filesystems
sigma-fs mount <dev> <mnt> [type]   # mount filesystem
sigma-fs umount <mnt>      # unmount
sigma-fs mkfs <dev> [--type sigmafs|ext4|fat32]   # format partition
sigma-fs check <dev>       # fsck — check + repair (sigma-fsck)
sigma-fs info <dev>        # show superblock, inode count, free space
sigma-fs snapshot <mnt>    # create immutable snapshot
sigma-fs restore <snap>    # restore from snapshot
sigma-fs compact <mnt>     # defragment / compact SigmaFS
sigma-fs verity enable <dev>    # enable dm-verity on partition
sigma-fs verity check <dev>     # verify dm-verity hash tree
sigma-fs cache stats        # show UBC page cache hit/miss
sigma-fs cache drop         # drop page cache (for benchmarking)
```

| Task | File | Blocked by |
|------|------|------------|
| `sigma-fs list` | `userland/tools/sigma_fs_cli.cpp` | VFS mount table |
| `sigma-fs mount/umount` | `userland/tools/sigma_fs_cli.cpp` | `sigma_mount()` syscall |
| `sigma-fs mkfs` | `userland/tools/sigma_fs_cli.cpp` | `sigma_mkfs` for SigmaFS |
| `sigma-fs check` | `userland/tools/sigma_fs_cli.cpp` | `sigma-fsck` binary |
| `sigma-fs snapshot` | `userland/tools/sigma_fs_cli.cpp` | SigmaFS snapshot support |
| `sigma-fs verity` | `userland/tools/sigma_fs_cli.cpp` | `sigma_dmverity.cpp` |
| `sigma-fs cache stats` | `userland/tools/sigma_fs_cli.cpp` | UBC counters via `/proc/sigma/ubc` |

### sigma-disk (block device management)

**New file:** `userland/tools/sigma_disk_cli.cpp`

```
sigma-disk list            # list block devices (like lsblk)
sigma-disk info <dev>      # NVMe controller info, model, serial
sigma-disk smart <dev>     # SMART health data
sigma-disk bench <dev>     # sequential read/write throughput
sigma-disk part list <dev> # list GPT/MBR partitions
sigma-disk part create <dev> <start> <size> [type]
sigma-disk part delete <dev> <num>
sigma-disk part format <dev><num> --type sigmafs
sigma-disk wipe <dev>      # secure erase (passes zeros + Dilithium-attested)
```

---

## `performance-optimized` — Performance CLI Commands

### sigma-perf (performance analysis)

**New file:** `userland/tools/sigma_perf_cli.cpp`

```
sigma-perf top             # live CPU/mem/IO stats (sigma-observatory TUI)
sigma-perf record <pid> [duration]   # hardware PMU sample collection
sigma-perf report          # analyze recorded profile
sigma-perf stat <cmd>      # run command with perf counters
sigma-perf bench pqc       # Kyber/Dilithium ops/sec benchmark
sigma-perf bench sched     # context-switch latency benchmark
sigma-perf bench mem       # memory bandwidth + allocator benchmark
sigma-perf bench net       # network throughput benchmark
sigma-perf bench io        # disk read/write IOPS benchmark
sigma-perf flame <pid>     # generate flamegraph SVG
sigma-perf kpatch status   # list active live patches
sigma-perf kpatch apply <patch>  # apply sigma-kpatch live
sigma-perf governor show   # current P-state + frequency
sigma-perf governor set <performance|powersave|auto>
sigma-perf numa show       # NUMA topology + per-node memory stats
sigma-perf numa bind <pid> <node>    # bind process to NUMA node
```

| Task | File | Detail |
|------|------|--------|
| `sigma-perf top` | `userland/tools/sigma_perf_cli.cpp` | Read `/proc/sigma/stats`, VT100 refresh |
| `sigma-perf bench pqc` | `userland/tools/sigma_perf_cli.cpp` | Call `kyber_keygen()` 10000×, report ops/sec |
| `sigma-perf bench sched` | `userland/tools/sigma_perf_cli.cpp` | RDTSC before/after context switch via IPC |
| `sigma-perf kpatch apply` | `userland/tools/sigma_perf_cli.cpp` | Syscall `SIGMA_SYS_KPATCH_APPLY(patch_fd)` |
| `sigma-perf governor set` | `userland/tools/sigma_perf_cli.cpp` | Write `IA32_PERF_CTL` MSR via privileged syscall |
| `sigma-perf numa show` | `userland/tools/sigma_perf_cli.cpp` | Read ACPI SRAT topology from kernel |

### sigma-pqc (crypto diagnostics)

**New file:** `userland/tools/sigma_pqc_cli.cpp`

```
sigma-pqc status           # show active algorithms + FIPS compliance level
sigma-pqc bench            # run Kyber + Dilithium benchmark
sigma-pqc keygen           # generate a fresh ML-KEM-1024 keypair (test)
sigma-pqc sign <file>      # sign file with ML-DSA-87
sigma-pqc verify <file> <sig> <pubkey>  # verify signature
sigma-pqc upgrade           # migrate legacy RSA keys to PQC
sigma-pqc rotate           # rotate system keys
sigma-pqc audit            # audit all package signatures
```

---

## `release/standalone` — Desktop CLI Commands

### sigma-pkg (package manager)

**New file:** `userland/sigma-pkg/sigma_pkg_cli.cpp`

```
sigma-pkg install <name> [--version x.y.z]
sigma-pkg remove <name> [--purge]
sigma-pkg list             # installed packages
sigma-pkg list --available # packages in repo
sigma-pkg search <query>
sigma-pkg update           # refresh package index
sigma-pkg upgrade          # upgrade all installed
sigma-pkg upgrade <name>   # upgrade single package
sigma-pkg hold <name>      # prevent upgrade
sigma-pkg unhold <name>
sigma-pkg info <name>      # show metadata, deps, signature
sigma-pkg verify <name>    # re-verify Dilithium3 signature
sigma-pkg deps <name>      # dependency tree
sigma-pkg clean            # remove orphaned packages
sigma-pkg rollback <name>  # revert to previous version
sigma-pkg audit            # check for signature violations
sigma-pkg mirror list      # list configured mirrors
sigma-pkg mirror add <url> # add mirror
sigma-pkg repo list        # list enabled repositories
sigma-pkg build <recipe>   # build .spkg from recipe file
```

| Task | File | Blocked by |
|------|------|------------|
| `sigma-pkg install` | `userland/sigma-pkg/sigma_pkg_cli.cpp` | sigma-repo-server + VFS write |
| `sigma-pkg verify` | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `pqc_verify()` real Dilithium |
| `sigma-pkg rollback` | `userland/sigma-pkg/sigma_pkg_cli.cpp` | Atomic staging swap on install |
| `sigma-pkg build` | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `.spkg` recipe parser in `sigma_pkg_recipe.c` |

### sigma-wine (Windows compatibility)

**New file:** `userland/tools/sigma_wine_cli.cpp`

```
sigma-wine exec <exe> [args]             # run Windows EXE
sigma-wine exec --d3d11 <game.exe>       # force D3D11→Vulkan
sigma-wine exec --debug <exe>            # trace NT syscalls
sigma-wine info <exe>                    # PE header dump  [✅ works now]
sigma-wine prefix create <path> [--arch win64]
sigma-wine prefix list                   # list all prefixes
sigma-wine prefix delete <path>
sigma-wine prefix shell <path>           # open sigma-sh inside prefix
sigma-wine dll list                      # show registered DLL stubs
sigma-wine dll override <dll> builtin|native
sigma-wine reg get HKCU\\Software\\App\\key
sigma-wine reg set HKCU\\Software\\App\\key <type> <value>
sigma-wine reg list HKCU\\Software\\App
sigma-wine version                       # sigma-wine capabilities + compat version
```

| Task | File | Detail |
|------|------|--------|
| `sigma-wine exec` | `userland/tools/sigma_wine_cli.cpp` | Call `sigma_wine_exec()` from `sigma_wine.h` |
| `sigma-wine info` | `userland/tools/sigma_wine_cli.cpp` | Call `sigma_wine_inspect()` — works today |
| `sigma-wine prefix *` | `userland/tools/sigma_wine_cli.cpp` | Call `sigma_wine_create_prefix()` |
| `sigma-wine reg *` | `userland/tools/sigma_wine_cli.cpp` | Call `sigma_reg_*()` from `sigma_reg.h` |
| `sigma-wine dll override` | `userland/tools/sigma_wine_cli.cpp` | Call `sigma_wine_override_dll()` |

### sigma-zenith (desktop control)

**New file:** `userland/tools/sigma_zenith_cli.cpp`

```
sigma-zenith restart        # restart compositor (safe)
sigma-zenith heal           # trigger compositor self-healing
sigma-zenith layout bsp|columns|grid|master-stack|floating|monocle
sigma-zenith gaps inner <n> outer <n>
sigma-zenith focus left|right|up|down
sigma-zenith fullscreen     # toggle focused window fullscreen
sigma-zenith float          # toggle focused window floating
sigma-zenith workspace <n>  # switch to workspace 1–10
sigma-zenith move-to <n>    # move window to workspace n
sigma-zenith screenshot [file]     # capture screen to PNG
sigma-zenith theme set <name>      # apply theme from ~/.sigma/themes/
sigma-zenith theme list            # list available themes
sigma-zenith theme create <name>   # create new theme skeleton
sigma-zenith bar show|hide         # taskbar visibility
sigma-zenith scale <factor>        # HiDPI scaling (1.0, 1.5, 2.0)
sigma-zenith fps                   # show live frame rate
sigma-zenith debug                 # overlay: window ids, frame times
```

| Task | File | Detail |
|------|------|--------|
| `sigma-zenith layout` | `userland/tools/sigma_zenith_cli.cpp` | IPC to `sigma_wm_layout()` |
| `sigma-zenith gaps` | `userland/tools/sigma_zenith_cli.cpp` | IPC to `sigma_wm_gaps()` |
| `sigma-zenith focus *` | `userland/tools/sigma_zenith_cli.cpp` | IPC to `sigma_wm_focus()` |
| `sigma-zenith workspace *` | `userland/tools/sigma_zenith_cli.cpp` | IPC to `sigma_wm_switch_ws()` |
| `sigma-zenith restart/heal` | `userland/tools/sigma_zenith_cli.cpp` | IPC to `zenith_compositor_heal()` |
| `sigma-zenith theme set` | `userland/tools/sigma_zenith_cli.cpp` | Write theme to `~/.sigma_profile`, send IPC |
| `sigma-zenith screenshot` | `userland/tools/sigma_zenith_cli.cpp` | DMA-BUF read from compositor, encode PNG |

### sigma-ai (on-device LLM)

**New file:** `userland/ai/sigma_ai_cli.cpp`

```
sigma-ai ask "<prompt>"                  # query local LLM
sigma-ai ask --lang hi "<prompt>"        # query in Hindi
sigma-ai heal <crashdump>               # analyze kernel crash dump
sigma-ai lex <gazette.pdf>              # parse Gazette of India
sigma-ai status                          # daemon alive? model loaded?
sigma-ai model list                      # installed GGUF models
sigma-ai model load <name>              # switch active model
sigma-ai model download <name>          # download from sigma-pkg
sigma-ai perf                            # inference ops/sec benchmark
sigma-ai bhashini asr <audio.wav>       # offline speech→text
sigma-ai bhashini tts "<text>" [lang]   # offline text→speech WAV
sigma-ai bhashini translate <text> --from hi --to en
```

| Task | File | Blocked by |
|------|------|------------|
| `sigma-ai ask` | `userland/ai/sigma_ai_cli.cpp` | llama.cpp integration |
| `sigma-ai heal` | `userland/ai/sigma_ai_cli.cpp` | sigma-ai daemon + crash dump parser |
| `sigma-ai lex` | `userland/ai/sigma_ai_cli.cpp` | PDF parser + sigma-ai daemon |
| `sigma-ai bhashini *` | `userland/ai/sigma_ai_cli.cpp` | sigma-bhashini offline models |

### sigma-ime (input method)

**New file:** `userland/ime/sigma_ime_cli.cpp`

```
sigma-ime list             # list available input methods
sigma-ime set <method>     # set active IME (inscript-hi, phonetic-hi, etc.)
sigma-ime get              # show active IME
sigma-ime test "<text>"    # test conversion: "namaste" → "नमस्ते"
sigma-ime toggle           # toggle IME on/off (or Ctrl+Space)
sigma-ime lang list        # all supported language codes
```

---

## `release/cloud` — Cloud & Container CLI Commands

### sigma-pod (container orchestration)

**File:** `userland/tools/sigma_pod_cli.cpp`
**Current:** `create/start/stop/ps/destroy` exist with IPC stubs.

```
sigma-pod create --name <n> --mem <mb> --cpu <shares>   [⚠️ stub]
sigma-pod start <id>                                     [⚠️ stub]
sigma-pod stop <id>                                      [⚠️ stub]
sigma-pod ps                                             [⚠️ stub]
sigma-pod destroy <id>                                   [⚠️ stub]
sigma-pod run-native <spkg> --all-ns --cpu=250 --mem=128 [❌ build]
sigma-pod exec <id> <cmd>     # exec inside running container
sigma-pod logs <id> [--follow]
sigma-pod stats <id>          # real-time CPU/mem/IO usage
sigma-pod inspect <id>        # JSON dump of container config
sigma-pod export <id> <file>  # export container to .spkg
sigma-pod import <file>       # import .spkg as container image
sigma-pod images list         # list available .spkg images
sigma-pod network list        # list container network namespaces
sigma-pod network connect <id> <net>
sigma-pod volume create <name>
sigma-pod volume list
sigma-pod volume mount <vol> <id> <path>
sigma-pod pause <id>          # freeze (SIGSTOP all processes)
sigma-pod resume <id>         # unfreeze (SIGCONT)
sigma-pod checkpoint <id>     # CRIU-style checkpoint
sigma-pod restore <checkpoint>
```

| Task | File | Detail |
|------|------|--------|
| Wire `run-native` to kernel orchestrator | `userland/tools/sigma_pod_cli.cpp` | Call kernel IPC: create namespaces + cgroup |
| `sigma-pod exec` | `userland/tools/sigma_pod_cli.cpp` | Fork into container namespace, exec cmd |
| `sigma-pod logs` | `userland/tools/sigma_pod_cli.cpp` | Read pod log ring buffer from VFS |
| `sigma-pod stats` | `userland/tools/sigma_pod_cli.cpp` | Read `/proc/sigma/cgroup/<id>/stat` |
| `sigma-pod inspect` | `userland/tools/sigma_pod_cli.cpp` | Return JSON from container registry |
| `sigma-pod checkpoint` | `userland/tools/sigma_pod_cli.cpp` | Syscall `SIGMA_SYS_POD_CHECKPOINT` |

### sigma-fleet (enterprise device management)

**New file:** `userland/tools/sigma_fleet_cli.cpp`

```
sigma-fleet status                      # agent heartbeat + health
sigma-fleet register <server> <token>   # register device with fleet server
sigma-fleet deregister
sigma-fleet policy get                  # fetch + apply .sigma-policy
sigma-fleet policy show                 # current active policy
sigma-fleet update pull                 # pull OS update from fleet server
sigma-fleet update apply                # apply pulled update (A/B)
sigma-fleet update status               # show pending update info
sigma-fleet inventory                   # report hardware inventory
sigma-fleet audit                       # send audit log to fleet server
sigma-fleet lock                        # lock device (remote wipe capable)
sigma-fleet unlock <token>
sigma-fleet list                        # list managed devices (from server)
sigma-fleet logs push                   # push sigma-audit log to fleet
```

### sigma-kube (SovereignCluster orchestration)

**New file:** `userland/tools/sigma_kube_cli.cpp`

```
sigma-kube node list                    # list cluster nodes
sigma-kube node add <ip> <token>        # join node to cluster
sigma-kube node remove <id>
sigma-kube pod deploy <spkg> --replicas n
sigma-kube pod list
sigma-kube pod scale <name> --replicas n
sigma-kube pod delete <name>
sigma-kube service expose <pod> --port n
sigma-kube service list
sigma-kube cluster status               # cluster health summary
sigma-kube cluster init                 # bootstrap new cluster
sigma-kube cluster join <leader-ip>     # join existing cluster
sigma-kube namespace create <name>
sigma-kube namespace list
sigma-kube logs <pod> [--follow]
sigma-kube exec <pod> <cmd>
```

---

## `release/distributed` — Distributed & Storage CLI Commands

### sigma-cloudfs (distributed block storage)

**New file:** `userland/tools/sigma_cloudfs_cli.cpp`

```
sigma-cloudfs status         # cluster node count + replication state
sigma-cloudfs init <node-list>          # bootstrap Raft cluster
sigma-cloudfs join <leader-ip>          # join existing cluster
sigma-cloudfs leave                     # graceful node removal
sigma-cloudfs sync                      # force immediate sync
sigma-cloudfs check                     # verify replication integrity
sigma-cloudfs volume create <name> <size>
sigma-cloudfs volume list
sigma-cloudfs volume delete <name>
sigma-cloudfs volume snapshot <name>
sigma-cloudfs volume restore <snapshot>
sigma-cloudfs bench read|write          # throughput benchmark
sigma-cloudfs encrypt enable <volume>   # ML-KEM encrypt at rest
```

### sigma-mesh (distributed compute grid)

**New file:** `userland/tools/sigma_mesh_cli.cpp`

```
sigma-mesh status           # show mesh grid nodes + capacity
sigma-mesh join <network-id>
sigma-mesh leave
sigma-mesh submit <job.sigma>           # submit compute job
sigma-mesh jobs list
sigma-mesh jobs cancel <id>
sigma-mesh jobs logs <id>
sigma-mesh nodes list       # all nodes in the grid
sigma-mesh bench            # compute benchmark (matrix multiply)
```

### sigma-zkvm (zero-knowledge VM)

**New file:** `userland/tools/sigma_zkvm_cli.cpp`

```
sigma-zkvm prove --claim "income > 500000" --input balance.json
sigma-zkvm verify <proof.json> <pubkey>
sigma-zkvm compile <circuit.zkvm>
sigma-zkvm run <circuit> <input>
sigma-zkvm benchmark
```

---

## `release/microkernel` — Minimal CLI Commands

This profile is CLI-only. Zenith and desktop commands are excluded.
Only essential system commands ship.

### Essential system commands for microkernel profile

```
sigma-sh                    # login shell (REPL)           [✅ core]
sigma-cli profile use minimal                              [✅ core]
sigma-sched show            # runqueue state               [kernel-exp]
sigma-mem stats             # buddy allocator stats         [kernel-exp]
sigma-irq list              # IRQ table                    [kernel-exp]
sigma-boot status           # A/B boot slot               [kernel-exp]
sigma-boot rollback         # emergency rollback          [kernel-exp]
sigma-pkg install <pkg>     # package install             [tools-dev]
sigma-fs list               # mount table                 [fs-dev]
sigma-net status            # basic NIC status            [drivers-dev]
sigma-net ping <host>       # ICMP ping                   [drivers-dev]
sigma-bus list              # IPC channel list            [kernel-exp]
sigma-bus ping <service>    # test sigma-bus endpoint     [kernel-exp]
sigma-audit log             # show tamper-evident log     [security]
sigma-pqc status            # PQC engine state            [perf-optimized]
```

### sigma-bus (IPC diagnostics)

**New file:** `userland/tools/sigma_bus_cli.cpp`

```
sigma-bus list              # list all registered sigma-bus services
sigma-bus ping <service>    # send ping to service, measure latency
sigma-bus call <service> <method> [args]   # raw IPC call
sigma-bus monitor           # live IPC message trace
sigma-bus trace <service>   # trace messages to/from a service
sigma-bus stats             # message throughput + latency histograms
sigma-bus register <service>            # register current process as service
sigma-bus capabilities <service>        # list capability tokens
```

---

## `release/rtos` — Real-Time CLI Commands

### sigma-rt (real-time scheduler control)

**New file:** `userland/tools/sigma_rt_cli.cpp`

```
sigma-rt list               # list RT tasks + deadlines
sigma-rt set <pid> --policy edf --deadline <ns> --period <ns>
sigma-rt set <pid> --policy fifo --priority <1-99>
sigma-rt set <pid> --policy rr --timeslice <ns>
sigma-rt budget <tid> <budget_ns> <period_ns>   # MCS
sigma-rt latency test       # measure IRQ + scheduler latency
sigma-rt latency report     # histogram of latency samples
sigma-rt stress --tasks 64  # generate RT scheduling stress
sigma-rt trace start|stop|report
sigma-rt jitter show        # show scheduling jitter stats
```

### sigma-rt CI commands

```bash
# scripts/standalone_rt_validate.sh — already exists
./scripts/standalone_rt_validate.sh       # real-time profile validation

sigma-rt latency test --max-jitter 10us  # CI gate: < 10 µs jitter
sigma-rt stress --duration 60s --assert-no-miss  # no deadline misses
```

---

## `release/mobile` — ARM64 Mobile CLI Commands

### sigma-ultra (USSD / feature phone interface)

**New file:** `userland/sigma_ultra_cli.cpp`

```
sigma-ultra menu            # show top-level USSD menu
sigma-ultra health          # health status check
sigma-ultra pay <vpa> <amount>          # UPI payment
sigma-ultra balance         # check IPPB/UPI balance
sigma-ultra weather         # current weather (offline-first)
sigma-ultra ration          # PDS ration status
sigma-ultra mgnregs <id>    # MGNREGS attendance/payment
sigma-ultra update          # OTA update over 2G
sigma-ultra lang <code>     # switch language (hi/ta/te/bn/mr)
```

### sigma-arm (ARM64 hardware control)

**New file:** `userland/tools/sigma_arm_cli.cpp`

```
sigma-arm cpu-info          # show ARM64 core types (big.LITTLE)
sigma-arm freq list         # available CPU frequencies
sigma-arm freq set <core> <hz>
sigma-arm thermal           # core temperatures
sigma-arm dts info          # device tree blob info
sigma-arm gpio list         # GPIO pin states (RPi GPIO)
sigma-arm gpio set <pin> <high|low>
sigma-arm i2c scan <bus>    # scan I2C devices on bus
sigma-arm spi list          # SPI devices
sigma-arm power suspend     # suspend to RAM (S3)
sigma-arm power hibernate   # suspend to disk
```

---

## `release/dual-boot` — Dual-Boot CLI Commands

### sigma-install (installer / dual-boot management)

**New file:** `userland/installer/sigma_install_cli.cpp`

```
sigma-install list-disks    # list physical disks + partition tables
sigma-install detect-os     # detect existing OS installations
sigma-install plan <disk>   # show proposed partition layout
sigma-install run <disk> [--dual-boot] [--erase]   # full install
sigma-install uninstall     # remove SigmaOS, restore boot entry
sigma-install repair        # repair EFI entry + grub config
sigma-install migrate <dir> # migrate existing data

sigma-boot-entry list       # list EFI boot entries
sigma-boot-entry add <label> <loader>
sigma-boot-entry remove <num>
sigma-boot-entry set-default <num>
```

---

## `release/browser` + `release/app` — Web/App CLI Commands

### sigma-web (browser + web API control)

**New file:** `sigma-web/sigma_web_cli.cpp`

```
sigma-web open <url>        # open URL in sigma-browser
sigma-web api list          # list available Web API drivers
sigma-web api status <name> # status of a specific Web API driver
sigma-web demo              # launch QEMU-in-browser demo
sigma-web build             # rebuild sigma-web WebAssembly bundle
sigma-web test              # run Web API driver tests
```

### sigma-app (app store management)

**New file:** `userland/tools/sigma_app_cli.cpp`

```
sigma-app list              # list installed apps
sigma-app search <query>    # search app store
sigma-app install <name>    # install from app store
sigma-app remove <name>
sigma-app update            # update all apps
sigma-app info <name>       # show app metadata + permissions
sigma-app permissions <name>  # show + modify app capability grants
sigma-app sandbox <name>    # show MAC sandbox policy
sigma-app launch <name>     # launch app via sigma-pod
```

---

## `docs-update` + `prepare-sigmaos-launch` — DevOps CLI Commands

### sigma-docs (documentation tooling)

**New file:** `scripts/sigma_docs_cli.sh`

```bash
sigma-docs build            # run doxygen → HTML + wiki stubs
sigma-docs serve            # local HTTP server for docs preview
sigma-docs check            # validate all wiki links
sigma-docs sync             # mirror docs → wiki_repo/ (wiki-sync)
sigma-docs lint <file>      # markdownlint on specific file
sigma-docs gen-changelog    # auto-generate CHANGELOG from git log
sigma-docs man <tool>       # generate man page from --help output
```

### sigma-release (release pipeline)

**New file:** `scripts/sigma_release_cli.sh`

```bash
sigma-release check         # verify all release gates pass
sigma-release tag <version> # create annotated tag + sign with Dilithium
sigma-release iso           # build + verify reproducible ISO
sigma-release sign          # sign ISO + packages with ML-DSA-87
sigma-release publish       # upload to GitHub Releases
sigma-release notes <tag>   # generate release notes from commits
sigma-release verify <iso>  # verify a release ISO signature
```

### CI commands

```bash
# .github/workflows/sigma_ci.yml — already wires these:
make PROFILE=standalone iso             # build ISO
make check-stubs                        # warn on stubs
make check-abi                          # ABI stability check
./scripts/ci_branch_check.sh           # branch parity  [✅ real]
./scripts/sigma_automation.sh wiki-sync # wiki mirror    [✅ real]
./scripts/sigma_git_sync.sh --dry-run  # git sync preview [✅ real]
./scripts/sigma_branch_sync.sh --report # parity report  [✅ real]

# Still echo stubs — need to be real:
qemu-system-x86_64 -cdrom build/sigmaos.iso -serial stdio  # QEMU boot test
sigma-release verify build/sigmaos.iso  # signature verify
```

---

## `gh-pages` — Website CLI Commands

### site management

```bash
# scripts/ hooks for gh-pages:
./scripts/build_standalone.sh          # build standalone profile   [✅ exists]
./scripts/gen_iso.sh                   # generate ISO               [✅ exists]

# New:
sigma-site build            # generate static site (index.html + roadmap.html)
sigma-site serve            # local dev server
sigma-site deploy           # push to gh-pages branch
sigma-site status           # show last deploy + build stats
```

---

## Security-Focused CLI Commands (all branches)

### sigma-sec (security management)

**New file:** `userland/tools/sigma_sec_cli.cpp`

```
sigma-sec status            # overall security posture summary
sigma-sec audit log         # show Dilithium3-attested audit trail
sigma-sec audit export <file>          # export audit to JSON
sigma-sec mac status        # MAC policy engine status
sigma-sec mac policy show   # current .sigma-policy
sigma-sec mac policy set <file>        # apply new policy
sigma-sec mac suggest <cmd> # AI-suggest MAC policy for a command
sigma-sec sandbox run <cmd> # run command in capability sandbox
sigma-sec ids status        # sigma-ids anomaly detector status
sigma-sec ids alerts        # recent anomaly alerts
sigma-sec ids train         # retrain anomaly model
sigma-sec trust show        # sigma-trustd DID + certificate chain
sigma-sec trust verify <did>           # verify a DID document
sigma-sec pqc status        # PQC algorithms in use
sigma-sec pqc rotate        # rotate PQC keys
sigma-sec cve list          # local CVE database check
sigma-sec cve patch <id>    # apply kpatch for CVE
sigma-sec pentest           # run IT Act-compliant self-pentest
sigma-sec wipe              # secure erase + Dilithium-attested log
```

### sigma-audit (tamper-evident log)

**New file:** `userland/tools/sigma_audit_cli.cpp`

```
sigma-audit log             # show recent audit entries
sigma-audit log --since <timestamp>
sigma-audit log --filter <subsystem>
sigma-audit verify          # verify Dilithium3 chain of custody
sigma-audit export <file>   # export to JSON/CSV
sigma-audit push            # push to sigma-fleet server
sigma-audit worm status     # show WORM register state (forensic profile)
sigma-audit clear           # clear volatile log (WORM entries persist)
```

### sigma-trust (identity + attestation)

**New file:** `userland/tools/sigma_trust_cli.cpp`

```
sigma-trust status          # TPM2 state + PCR measurements
sigma-trust boot verify     # verify full boot chain
sigma-trust did show        # show device DID document
sigma-trust did rotate      # rotate DID keys
sigma-trust attest <nonce>  # generate TPM2 attestation quote
sigma-trust cert list       # list sigma-trustd certificates
sigma-trust cert revoke <id>
sigma-trust remote verify <url>  # remote attestation endpoint
```

---

## India Stack CLI Commands (`release/standalone`)

### sigma-gst (GST compliance)

```
sigma-gst irn <invoice.json>           # generate GST IRN
sigma-gst eway <consignment.json>      # generate e-Way Bill
sigma-gst gstr1 <period>              # file GSTR-1
sigma-gst gstr3b <period>             # file GSTR-3B
sigma-gst hsn search <query>          # search HSN/SAC codes
sigma-gst verify <qr-code>            # verify GST invoice QR
```

### sigma-abdm (health stack)

```
sigma-abdm create-id                  # create ABHA Health ID
sigma-abdm link-phr <mobile>          # link PHR app
sigma-abdm records list               # list health records
sigma-abdm records push <file>        # push FHIR document
sigma-abdm prescribe <patient-id>     # create NMC-compliant e-prescription
sigma-abdm claim <patient-id>         # submit PMJAY claim
```

### sigma-upi (payments)

```
sigma-upi pay <vpa> <amount> [--note "text"]
sigma-upi balance
sigma-upi history [--last n]
sigma-upi mandate create <vpa> <amount> <period>
sigma-upi mandate list
sigma-upi mandate cancel <id>
```

### sigma-digilocker (document access)

```
sigma-digilocker list           # list linked documents
sigma-digilocker fetch <docid>  # download document
sigma-digilocker verify <docid> # verify document signature
sigma-digilocker share <docid> <requestor>
```

---

## Master CLI Command Status Table

| Command | Branch | File | Status |
|---------|--------|------|--------|
| `sigma-cli profile *` | tools-dev | `sigma_cli.cpp` | ✅ Real |
| `sigma-cli alias *` | tools-dev | `sigma_cli.cpp` | ✅ Real |
| `sigma-cli update/backup/sync` | tools-dev | `sigma_cli.cpp` | ⚠️ Stub |
| `sigma-sh builtins` | tools-dev | `sigma_shell.cpp` | ✅ Real |
| `sigma-sh TTY read` | tools-dev | `sigma_shell.cpp` | ❌ Missing |
| `sigma-pod create/start/stop/ps` | release/cloud | `sigma_pod_cli.cpp` | ⚠️ IPC stub |
| `sigma-pod run-native` | release/cloud | `sigma_pod_cli.cpp` | ❌ Missing |
| `sigma-automation.sh *` | tools-dev | `sigma_automation.sh` | ✅ Real |
| `sigma-git-sync.sh` | tools-dev | `sigma_git_sync.sh` | ✅ Real |
| `sigma-ci-branch-check.sh` | tools-dev | `ci_branch_check.sh` | ✅ Real |
| `sigma-wine info` | tools-dev | `sigma_pe_loader.cpp` | ✅ Works |
| `sigma-wine exec` | tools-dev | `sigma_wine_loader.cpp` | ❌ Needs VMM |
| `sigma-pkg install` | tools-dev | to build | ❌ Missing |
| `sigma-net ping` | drivers-dev | ICMP exists | ⚠️ No CLI |
| `sigma-net status` | drivers-dev | to build | ❌ Missing |
| `sigma-boot rollback` | kernel-exp | to build | ❌ Needs EFI |
| `sigma-mem stats` | kernel-exp | to build | ❌ Needs kernel |
| `sigma-sched show` | kernel-exp | to build | ❌ Needs kernel |
| `sigma-perf bench pqc` | performance-optimized | to build | ❌ Needs real NTT |
| `sigma-zenith layout` | release/standalone | to build | ❌ Needs IPC |
| `sigma-zenith theme` | release/standalone | to build | ❌ Needs VFS |
| `sigma-ai ask` | release/standalone | to build | ❌ Needs llama.cpp |
| `sigma-fleet status` | release/cloud | to build | ❌ Missing |
| `sigma-kube *` | release/distributed | to build | ❌ Missing |
| `sigma-cloudfs *` | release/distributed | to build | ❌ Missing |
| `sigma-rt set` | release/rtos | to build | ❌ Needs EDF |
| `sigma-ultra menu` | release/mobile | to build | ❌ Missing |
| `sigma-install run` | release/dual-boot | to build | ❌ Missing |
| `sigma-gst irn` | release/standalone | to build | ❌ Needs API |
| `sigma-abdm create-id` | release/standalone | to build | ❌ Needs ABDM API |
| `sigma-upi pay` | release/standalone | to build | ❌ Needs UPI API |

---

*See also: [Branch Development Roadmap](Branch-Development-Roadmap) · [Feature Branch Roadmap](Feature-Branch-Roadmap) · [Windows Compatibility Layer Roadmap](Windows-Compatibility-Layer-Roadmap) · [Development Roadmap](Development-Roadmap)*
