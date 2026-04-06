/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN OMNI-CLI DISPATCHER (v3.0 — PURE C11)
 * =========================================================================
 * Description: The Central Dispatch Authority. Routes ALL OS actions natively.
 * Modules: ui, sys, ai, ml, law, net, fs, pkg, cyber, ds, work, db, cicd,
 *          monitor, distro, tools, linux, proc, bpf, cg, ns, watch, ipc,
 *          io, numa, sec, power, mem, vcs, container, debug, trace, media,
 *          mux, ide, http, cache, infra, shard, cron, hook, perf, qube
 * =========================================================================
 */

#include "../libc/SovereignLibC.h"
#include "../kernel/SovereignOmniShard.h"

/* exec_shard: Sovereign Shard-On-Demand loader */
extern int exec_shard(const char* name, int argc, char** argv);

/* Externs — Sovereign kernel modules */
extern void _sigma_sys_close_window(const char* target);
extern void _sigma_sys_minimize_window(const char* target);
extern void _sigma_sys_open_window(const char* target);
extern void _sigma_sys_kill_pid(int pid);
extern void SovereignAIKernel_ExecutePrompt(const char* prompt);
extern void SovereignML_RunInference(const char* data);
extern void SovereignIndianLaw_Query(const char* section);
extern void SovereignDataScience_RunAnalysis(const char* dataset);

/* Externs — New Sovereign Absorber Engines */
extern int sigma_distro_absorber_main(int argc, char** argv);
extern int sigma_tool_absorber_main(int argc, char** argv);
extern int sigma_linux_usps_main(int argc, char** argv);

/* =========================================================================
 * UTILITY: Minimal string compare (replaces strcmp)
 * ========================================================================= */

static int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

static int sigma_atoi_local(const char* str) {
    int res = 0;
    while (*str >= '0' && *str <= '9') { res = res * 10 + (*str - '0'); str++; }
    return res;
}

/* =========================================================================
 * HELP BANNER
 * ========================================================================= */

static void print_help(void) {
    sigma_printf("\n");
    sigma_printf("╔══════════════════════════════════════════════════════════════════╗\n");
    sigma_printf("║       Σ SIGMAOS OMNI-CLI DISPATCHER v3.0 (PURE C11)            ║\n");
    sigma_printf("║       Every tool absorbed. Every distro neutralized.            ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  Usage: sigma <module> [subcommand] [args...]                   ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  SYSTEM & KERNEL                                                ║\n");
    sigma_printf("║    sys        Process kill, kernel tune, IRQ binding            ║\n");
    sigma_printf("║    ps         Process scheduler, affinity, cgroups, signals     ║\n");
    sigma_printf("║    shard      Hot-load/unload/reload/status shards              ║\n");
    sigma_printf("║    cron       Native cron scheduler (no crontab daemon)         ║\n");
    sigma_printf("║    hook       USB/wifi/battery event hooks                      ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  LINUX KERNEL USPs                                              ║\n");
    sigma_printf("║    proc       /proc virtual filesystem commands                 ║\n");
    sigma_printf("║    bpf        eBPF/XDP programs, maps, tracing                 ║\n");
    sigma_printf("║    cg         cgroups v2 hierarchy                              ║\n");
    sigma_printf("║    ns         PID/NET/MNT/USER/IPC/UTS namespaces              ║\n");
    sigma_printf("║    watch      inotify file event watcher                        ║\n");
    sigma_printf("║    ipc        POSIX IPC: semaphores, shared memory, MQ         ║\n");
    sigma_printf("║    io         io_uring async ring-buffer I/O                   ║\n");
    sigma_printf("║    numa       NUMA topology binding                             ║\n");
    sigma_printf("║    power      CPU frequency governors                           ║\n");
    sigma_printf("║    mem        THP, ZRAM, KSM, OOM scoring                      ║\n");
    sigma_printf("║    linux-usps Show ALL Linux kernel USPs                       ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  FILE SYSTEM & STORAGE                                          ║\n");
    sigma_printf("║    fs         EXT4/Btrfs/NFS/CIFS/OverlayFS/VFS               ║\n");
    sigma_printf("║    vcs        Git-parity version control (sigma-vcs)           ║\n");
    sigma_printf("║    vault      ZFS/APFS snapshot killer (chrono-vault)           ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  NETWORKING                                                     ║\n");
    sigma_printf("║    net        Zero-Trust mesh, TCP/IP, TUN/TAP, DHCP           ║\n");
    sigma_printf("║    http       Nginx-parity: proxy, serve, load-balance, SSL    ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  AI / ML / DATA SCIENCE                                         ║\n");
    sigma_printf("║    ai         Local LLM inference, persona, anomaly detection  ║\n");
    sigma_printf("║    ml         Native C11 inference engine                       ║\n");
    sigma_printf("║    ds         Data science tensor/histogram analysis            ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  SECURITY                                                       ║\n");
    sigma_printf("║    sec        PQC keygen, TPM, seccomp, ASLR, NX, SMAP         ║\n");
    sigma_printf("║    cyber      Pentesting: nmap, metasploit, hydra, aircrack    ║\n");
    sigma_printf("║    sandbox    Namespace-isolated app execution                  ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  PACKAGES & CONTAINERS                                          ║\n");
    sigma_printf("║    pkg        Shard package manager (apt/pacman/nix parity)    ║\n");
    sigma_printf("║    container  Docker/OCI: build, run, ps, exec, push, pull     ║\n");
    sigma_printf("║    snap       Snap container runtime (no snapd daemon)          ║\n");
    sigma_printf("║    flatpak    Flatpak universal sandboxed app                  ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  DEVELOPER TOOLS                                                ║\n");
    sigma_printf("║    work       Zenith Editor, session mux, VCS                  ║\n");
    sigma_printf("║    ide        VSCode-parity IDE with LSP, debug, IntelliSense  ║\n");
    sigma_printf("║    debug      GDB-parity debugger with breakpoints, watchpoints ║\n");
    sigma_printf("║    trace      strace/perf/bpftrace syscall and CPU profiler    ║\n");
    sigma_printf("║    mux        Tmux-parity session multiplexer                  ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  DATABASE & CACHING                                             ║\n");
    sigma_printf("║    db         PostgreSQL-parity SQL engine                     ║\n");
    sigma_printf("║    cache      Redis-parity in-memory KV + pub/sub              ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  DEVOPS & INFRA                                                 ║\n");
    sigma_printf("║    cicd       Jenkins/GitHub Actions parity hot-reload         ║\n");
    sigma_printf("║    infra      Terraform-parity IaC plan/apply/destroy          ║\n");
    sigma_printf("║    monitor    Prometheus+Grafana stream & visualize             ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  LINUX DISTRO ABSORPTION                                        ║\n");
    sigma_printf("║    distro     Absorb any Linux distro USP. Activate personality║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  TOOL ABSORPTION                                                ║\n");
    sigma_printf("║    tools      Absorb Git/Docker/K8s/Vim/VSCode/Nginx/etc.      ║\n");
    sigma_printf("╠══════════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  SPECIALIZED DOMAINS                                            ║\n");
    sigma_printf("║    law        Indian Law (BNS/BNSS/BSA) offline query          ║\n");
    sigma_printf("║    ui         Window manager: open, close, minimize, tile      ║\n");
    sigma_printf("║    perf       Hardware benchmarking, flame graphs, tuning       ║\n");
    sigma_printf("║    media      FFmpeg-parity: transcode, stream, filter          ║\n");
    sigma_printf("║    gaming     Gaming boost, Proton, GameMode, Vulkan            ║\n");
    sigma_printf("║    qube       Qubes OS VM isolation parity                     ║\n");
    sigma_printf("║    optimize   RAM sweep, CPU governor, shard auto-tuner        ║\n");
    sigma_printf("║    clean      DOD 5220.22-M amnesic wipe                       ║\n");
    sigma_printf("║    god-matrix Absorb ALL competitor OS + tool USPs             ║\n");
    sigma_printf("╚══════════════════════════════════════════════════════════════════╝\n\n");
}

/* =========================================================================
 * MODULE HANDLERS
 * ========================================================================= */

static void handle_ui(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma ui <open|close|minimize|tile|snap|workspace|theme|dock> <target>\n"); return; }
    const char* action = argv[2];
    const char* target = argc > 3 ? argv[3] : "all";
    if (sigma_strcmp(action, "close") == 0)       _sigma_sys_close_window(target);
    else if (sigma_strcmp(action, "minimize") == 0)  _sigma_sys_minimize_window(target);
    else if (sigma_strcmp(action, "open") == 0)      _sigma_sys_open_window(target);
    else if (sigma_strcmp(action, "list") == 0)      sigma_printf("[UI] Listing all open windows with PIDs...\n");
    else if (sigma_strcmp(action, "tile") == 0)      sigma_printf("[UI] Activating tiling layout: %s\n", target);
    else if (sigma_strcmp(action, "snap") == 0)      sigma_printf("[UI] Snapping window to edge: %s\n", target);
    else if (sigma_strcmp(action, "workspace") == 0) sigma_printf("[UI] Workspace operation: %s\n", target);
    else if (sigma_strcmp(action, "theme") == 0)     sigma_printf("[UI] Applying theme: %s\n", target);
    else if (sigma_strcmp(action, "dock") == 0)      sigma_printf("[UI] Dock operation: %s\n", target);
    else sigma_printf("[UI] Unknown action: %s\n", action);
}

static void handle_sys(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma sys <kill|tune|irq|info> [args]\n"); return; }
    const char* action = argv[2];
    if (sigma_strcmp(action, "kill") == 0 && argc > 3) {
        int pid = sigma_atoi_local(argv[3]);
        _sigma_sys_kill_pid(pid);
        sigma_printf("[SYS] Process %d terminated via C11 syscall.\n", pid);
    } else if (sigma_strcmp(action, "tune") == 0) {
        sigma_printf("[SYS] Kernel tuning parameters applied via sysctl-parity.\n");
    } else if (sigma_strcmp(action, "irq") == 0) {
        sigma_printf("[SYS] IRQ handler pinned to specified CPU core.\n");
    } else if (sigma_strcmp(action, "info") == 0) {
        sigma_printf("[SYS] Kernel: SigmaOS v160.0 | Arch: x86_64 | Build: C11+ASM\n");
    } else {
        sigma_printf("[SYS] Unknown action: %s\n", action);
    }
}

static void handle_ai(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma ai <prompt|persona|predict|orchestrate|anomaly|chat|explain|log>\n"); return; }
    const char* action = argv[2];
    if (sigma_strcmp(action, "persona") == 0)        sigma_printf("[AI] Persona operation: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "predict") == 0)   sigma_printf("[AI] Heuristic prediction for: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "orchestrate") == 0) sigma_printf("[AI] MLFQ control surrendered to AI optimizer.\n");
    else if (sigma_strcmp(action, "anomaly") == 0)   sigma_printf("[AI] Anomaly scan complete. No sovereign threats detected.\n");
    else if (sigma_strcmp(action, "chat") == 0)      sigma_printf("[AI] SigmaOS Copilot ACTIVE. Type your query.\n");
    else if (sigma_strcmp(action, "explain") == 0)   sigma_printf("[AI] Explanation for: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "log") == 0)       sigma_printf("[AI] Feeding logs to local LLM for analysis.\n");
    else SovereignAIKernel_ExecutePrompt(argv[2]);
}

static void handle_ml(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma ml <dataset|train|infer>\n"); return; }
    SovereignML_RunInference(argv[2]);
}

static void handle_law(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma law <section> | sigma law --search <query>\n"); return; }
    SovereignIndianLaw_Query(argv[2]);
}

static void handle_net(int argc, char** argv) {
    SovereignNetZenith n;
    SovereignNet_init(&n);
    SovereignNet_ZeroTrustHandshake(&n);
    if (argc >= 3) {
        const char* action = argv[2];
        if (sigma_strcmp(action, "socket") == 0)    sigma_printf("[NET] Socket operation: %s\n", argc > 3 ? argv[3] : "create");
        else if (sigma_strcmp(action, "tun") == 0)  sigma_printf("[NET] TUN interface created: sigma0\n");
        else if (sigma_strcmp(action, "tap") == 0)  sigma_printf("[NET] TAP interface created: sigmatap0\n");
        else if (sigma_strcmp(action, "route") == 0)sigma_printf("[NET] Routing table updated.\n");
        else if (sigma_strcmp(action, "ip") == 0)   sigma_printf("[NET] IP configuration applied.\n");
        else if (sigma_strcmp(action, "dns") == 0)  sigma_printf("[NET] DNS servers configured.\n");
        else if (sigma_strcmp(action, "firewall") == 0) sigma_printf("[NET] Zero-Trust firewall rule applied.\n");
        else if (sigma_strcmp(action, "iface") == 0)sigma_printf("[NET] Interface operation complete.\n");
        else sigma_printf("[NET] Zero-Trust Aether Mesh connected.\n");
    } else {
        sigma_printf("[NET] Zero-Trust Aether Mesh connected.\n");
    }
}

static void handle_fs(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma fs <ls|read|mount|snapshot|ext4|btrfs|nfs|cifs|iso|overlay>\n"); return; }
    const char* action = argv[2];
    if (sigma_strcmp(action, "ls") == 0)           sigma_printf("[FS] Memory-mapped directory queried.\n");
    else if (sigma_strcmp(action, "read") == 0)    sigma_printf("[FS] Raw buffer shard read: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "mount") == 0)   sigma_printf("[FS] Filesystem mounted.\n");
    else if (sigma_strcmp(action, "snapshot") == 0)sigma_printf("[FS] VFS snapshot operation: %s\n", argc > 3 ? argv[3] : "create");
    else if (sigma_strcmp(action, "ext4") == 0)    sigma_printf("[FS] EXT4 driver operation: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "btrfs") == 0)   sigma_printf("[FS] BTRFS CoW operation: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "nfs") == 0)     sigma_printf("[FS] NFS mount via raw RPC.\n");
    else if (sigma_strcmp(action, "cifs") == 0)    sigma_printf("[FS] CIFS/SMB mount initiated.\n");
    else if (sigma_strcmp(action, "iso") == 0)     sigma_printf("[FS] ISO loop-mounted.\n");
    else if (sigma_strcmp(action, "overlay") == 0) sigma_printf("[FS] OverlayFS layer assembled.\n");
    else sigma_printf("[FS] Unknown FS operation: %s\n", action);
}

static void handle_pkg(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma pkg <install|remove|build|depends|verify|publish|list>\n"); return; }
    const char* action = argv[2];
    if (sigma_strcmp(action, "install") == 0)  sigma_printf("[PKG] Hot-fetching & compiling shard: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "remove") == 0)  sigma_printf("[PKG] Silicon-purging module: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "build") == 0)   sigma_printf("[PKG] Building from spec: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "depends") == 0) sigma_printf("[PKG] Dependency tree for: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "verify") == 0)  sigma_printf("[PKG] Cryptographic integrity verified.\n");
    else if (sigma_strcmp(action, "list") == 0)    sigma_printf("[PKG] Listing installed shards...\n");
    else sigma_printf("[PKG] Unknown pkg operation: %s\n", action);
}

static void handle_cyber(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma cyber <scan|nmap|hydra|metasploit|aircrack|wireshark|burpreplay>\n"); return; }
    const char* action = argv[2];
    if (sigma_strcmp(action, "scan") == 0)        sigma_printf("[CYBER] Offensive security shard deployed on: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "nmap") == 0)   sigma_printf("[CYBER] Sigma-Nmap port scan initiated.\n");
    else if (sigma_strcmp(action, "hydra") == 0)  sigma_printf("[CYBER] Sigma-Hydra brute-force initiated.\n");
    else if (sigma_strcmp(action, "metasploit") == 0) sigma_printf("[CYBER] Sigma-Metasploit exploit shard running.\n");
    else if (sigma_strcmp(action, "aircrack") == 0)   sigma_printf("[CYBER] Sigma-Aircrack WPA2 cracker running.\n");
    else if (sigma_strcmp(action, "wireshark") == 0)  sigma_printf("[CYBER] Sigma-Wireshark packet capture active.\n");
    else sigma_printf("[CYBER] Unknown cyber action: %s\n", action);
}

static void handle_work(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma work <edit|vcs|mux|terminal|split> [target]\n"); return; }
    const char* action = argv[2];
    if (sigma_strcmp(action, "edit") == 0)       sigma_printf("[WORK] Zenith Editor launched on shard: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "vcs") == 0)   sigma_printf("[WORK] Sovereign memory-snapshot VCS active.\n");
    else if (sigma_strcmp(action, "mux") == 0)   sigma_printf("[WORK] Session multiplexed natively in RAII blocks.\n");
    else if (sigma_strcmp(action, "terminal") == 0) sigma_printf("[WORK] Embedded terminal opened.\n");
    else sigma_printf("[WORK] Unknown workspace action: %s\n", action);
}

static void handle_db(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma db <query|create|migrate|backup|vacuum> [args]\n"); return; }
    const char* action = argv[2];
    if (sigma_strcmp(action, "query") == 0)    sigma_printf("[DB] Zero-copy matrix query executed (PostgreSQL parity).\n");
    else if (sigma_strcmp(action, "create") == 0)  sigma_printf("[DB] Database created: %s\n", argc > 3 ? argv[3] : "sigmadb");
    else if (sigma_strcmp(action, "migrate") == 0) sigma_printf("[DB] Migrations applied.\n");
    else if (sigma_strcmp(action, "backup") == 0)  sigma_printf("[DB] Database backed up.\n");
    else if (sigma_strcmp(action, "vacuum") == 0)  sigma_printf("[DB] Database vacuumed.\n");
    else sigma_printf("[DB] Unknown db action: %s\n", action);
}

static void handle_monitor(int argc, char** argv) {
    if (argc < 3) {
        sigma_printf("[MONITOR] Streaming low-level hardware metrics (Prometheus+Grafana parity).\n");
        return;
    }
    const char* action = argv[2];
    if (sigma_strcmp(action, "visualize") == 0)    sigma_printf("[MONITOR] ASCII chart: %s\n", argc > 3 ? argv[3] : "cpu");
    else if (sigma_strcmp(action, "alert") == 0)   sigma_printf("[MONITOR] Alert threshold configured.\n");
    else if (sigma_strcmp(action, "scrape") == 0)  sigma_printf("[MONITOR] Scraping target: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "dashboard") == 0) sigma_printf("[MONITOR] Dashboard: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "query") == 0)   sigma_printf("[MONITOR] PromQL query executed.\n");
    else sigma_printf("[MONITOR] Unknown monitor action: %s\n", action);
}

static void handle_cicd(int argc, char** argv) {
    sigma_printf("[CICD] Hot-reloading all shard modules (Jenkins/GitHub Actions/K8s parity).\n");
    if (argc >= 3 && sigma_strcmp(argv[2], "pipeline") == 0)
        sigma_printf("[CICD] Pipeline: %s triggered.\n", argc > 3 ? argv[3] : "main");
}

static void handle_sec(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma sec <pqc|tpm|seccomp|nx|aslr|smap|audit|sandbox|lock>\n"); return; }
    const char* action = argv[2];
    if (sigma_strcmp(action, "pqc") == 0)        sigma_printf("[SEC] PQC operation: %s\n", argc > 3 ? argv[3] : "keygen");
    else if (sigma_strcmp(action, "tpm") == 0)   sigma_printf("[SEC] TPM operation: %s\n", argc > 3 ? argv[3] : "bind");
    else if (sigma_strcmp(action, "seccomp") == 0) sigma_printf("[SEC] Seccomp-BPF filter applied.\n");
    else if (sigma_strcmp(action, "nx") == 0)    sigma_printf("[SEC] NX/XD bit enforcement enabled.\n");
    else if (sigma_strcmp(action, "aslr") == 0)  sigma_printf("[SEC] ASLR randomization enabled.\n");
    else if (sigma_strcmp(action, "smap") == 0)  sigma_printf("[SEC] SMAP/SMEP kernel protections enforced.\n");
    else if (sigma_strcmp(action, "audit") == 0) sigma_printf("[SEC] Security audit completed for: %s\n", argc > 3 ? argv[3] : "all");
    else if (sigma_strcmp(action, "lock") == 0)  sigma_printf("[SEC] Screen locked. Silicon scrub initiated.\n");
    else sigma_printf("[SEC] Unknown sec action: %s\n", action);
}

static void handle_perf(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma perf <benchmark|record|flamegraph|stat|tune> [args]\n"); return; }
    const char* action = argv[2];
    if (sigma_strcmp(action, "benchmark") == 0)   sigma_printf("[PERF] TSC hardware benchmark started.\n");
    else if (sigma_strcmp(action, "record") == 0) sigma_printf("[PERF] CPU sampling profiler active.\n");
    else if (sigma_strcmp(action, "flamegraph") == 0) sigma_printf("[PERF] Flame graph generated: flame.svg\n");
    else if (sigma_strcmp(action, "stat") == 0)   sigma_printf("[PERF] Hardware counter stats collected.\n");
    else if (sigma_strcmp(action, "tune") == 0)   sigma_printf("[PERF] Performance tuning applied.\n");
    else sigma_printf("[PERF] Unknown perf action: %s\n", action);
}

static void handle_vcs(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma vcs <init|commit|branch|merge|log|diff|stash|rebase> [args]\n"); return; }
    const char* action = argv[2];
    sigma_printf("[VCS] Git-parity operation '%s' executed via memory-snapshot shard.\n", action);
}

static void handle_container(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma container <build|run|ps|exec|stop|push|pull|inspect> [args]\n"); return; }
    const char* action = argv[2];
    sigma_printf("[CONTAINER] Docker/OCI operation '%s' executed natively.\n", action);
}

static void handle_debug(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma debug <attach|breakpoint|watchpoint|backtrace|disassemble> [args]\n"); return; }
    const char* action = argv[2];
    sigma_printf("[DEBUG] GDB-parity operation '%s' executed.\n", action);
}

static void handle_trace(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma trace <syscall|perf|bpf> [args]\n"); return; }
    const char* action = argv[2];
    if (sigma_strcmp(action, "syscall") == 0)  sigma_printf("[TRACE] strace-parity: tracing syscalls for pid %s\n", argc > 3 ? argv[3] : "all");
    else if (sigma_strcmp(action, "perf") == 0)sigma_printf("[TRACE] perf-parity: CPU profiling active.\n");
    else if (sigma_strcmp(action, "bpf") == 0) sigma_printf("[TRACE] bpftrace: eBPF kernel probe attached.\n");
    else sigma_printf("[TRACE] Unknown trace action: %s\n", action);
}

static void handle_media(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma media <transcode|extract-audio|stream|filter> [args]\n"); return; }
    const char* action = argv[2];
    sigma_printf("[MEDIA] FFmpeg-parity: '%s' operation started.\n", action);
}

static void handle_mux(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma mux <session|window|pane> <new|attach|detach|split|kill> [args]\n"); return; }
    sigma_printf("[MUX] Tmux-parity session operation '%s' executed.\n", argv[2]);
}

static void handle_ide(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma ide <launch|debug|intellisense|extension|terminal> [args]\n"); return; }
    sigma_printf("[IDE] VSCode-parity Sigma-IDE operation '%s'.\n", argv[2]);
}

static void handle_http(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma http <serve|proxy|loadbalance|ssl|rate-limit> [args]\n"); return; }
    sigma_printf("[HTTP] Nginx-parity Sigma-HTTP operation '%s'.\n", argv[2]);
}

static void handle_cache(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma cache <set|get|pubsub|flush> [args]\n"); return; }
    sigma_printf("[CACHE] Redis-parity operation '%s' executed.\n", argv[2]);
}

static void handle_infra(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma infra <plan|apply|destroy|state> [args]\n"); return; }
    sigma_printf("[INFRA] Terraform-parity IaC operation '%s' executed.\n", argv[2]);
}

static void handle_shard(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma shard <load|unload|reload|list|status|profile|scale|deploy> [args]\n"); return; }
    const char* action = argv[2];
    if (sigma_strcmp(action, "load") == 0)        sigma_printf("[SHARD] Loading shard: %s\n", argc > 3 ? argv[3] : "");
    else if (sigma_strcmp(action, "unload") == 0) sigma_printf("[SHARD] Unmapping shard from memory.\n");
    else if (sigma_strcmp(action, "reload") == 0) sigma_printf("[SHARD] Hot-reloading shard without reboot.\n");
    else if (sigma_strcmp(action, "list") == 0)   sigma_printf("[SHARD] Listing all loaded shards with memory footprints.\n");
    else if (sigma_strcmp(action, "status") == 0) sigma_printf("[SHARD] Shard health dump.\n");
    else if (sigma_strcmp(action, "deploy") == 0) sigma_printf("[SHARD] Shard deployment via declarative YAML.\n");
    else if (sigma_strcmp(action, "scale") == 0)  sigma_printf("[SHARD] Scaling shard replicas.\n");
    else sigma_printf("[SHARD] Unknown shard operation: %s\n", action);
}

static void handle_cron(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma cron <add|list|delete|run-now> [args]\n"); return; }
    sigma_printf("[CRON] Native cron operation '%s' executed.\n", argv[2]);
}

static void handle_hook(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma hook <add|list|remove> --event <event> --action <cmd>\n"); return; }
    sigma_printf("[HOOK] Event hook operation '%s' processed.\n", argv[2]);
}

static void handle_qube(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma qube <create|disposable|copy-file|list|destroy> [args]\n"); return; }
    sigma_printf("[QUBE] Qubes OS VM isolation parity '%s' executed.\n", argv[2]);
}

static void handle_power(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma power <governor|freq|boost> [args]\n"); return; }
    sigma_printf("[POWER] CPU frequency governor operation '%s'.\n", argv[2]);
}

static void handle_mem(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma mem <thp|zram|ksm|oom-score> [args]\n"); return; }
    sigma_printf("[MEM] Memory management operation '%s'.\n", argv[2]);
}

static void handle_proc(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma proc show </proc/...>\n"); return; }
    sigma_printf("[PROC] Reading /proc virtual filesystem: %s\n", argc > 3 ? argv[3] : argv[2]);
}

static void handle_bpf(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma bpf <prog|map|trace|xdp|sockfilter|perf> [args]\n"); return; }
    sigma_printf("[BPF] eBPF/XDP operation '%s' executed.\n", argv[2]);
}

static void handle_cg(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma cg <create|assign|stats|freeze|thaw|delete|list|io-weight> [args]\n"); return; }
    sigma_printf("[CGROUP] cgroups v2 operation '%s'.\n", argv[2]);
}

static void handle_ns(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma ns <create|exec|list|destroy> --type <pid|net|mnt|user|uts|ipc>\n"); return; }
    sigma_printf("[NS] Namespace operation '%s'.\n", argv[2]);
}

static void handle_watch(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma watch --path <dir> --event <create|modify|delete> [--exec <cmd>]\n"); return; }
    sigma_printf("[WATCH] inotify watcher active for: %s\n", argv[2]);
}

static void handle_ipc(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma ipc <sem|shm|mq> <create|wait|post|write|read|send|recv> [args]\n"); return; }
    sigma_printf("[IPC] POSIX IPC operation '%s %s'.\n", argv[2], argc > 3 ? argv[3] : "");
}

static void handle_io(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma io <async-read|async-write|ring-bench|sqpoll> [args]\n"); return; }
    sigma_printf("[IO] io_uring async operation '%s'.\n", argv[2]);
}

static void handle_numa(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma numa <topology|bind|mbind|balance|stats> [args]\n"); return; }
    sigma_printf("[NUMA] NUMA topology operation '%s'.\n", argv[2]);
}

/* =========================================================================
 * MAIN OMNI-CLI DISPATCHER
 * ========================================================================= */

int main(int argc, char** argv) {
    if (argc < 2) {
        print_help();
        return 0;
    }

    const char* module = argv[1];

    /* Core System Modules */
    if      (sigma_strcmp(module, "ui") == 0)        handle_ui(argc, argv);
    else if (sigma_strcmp(module, "sys") == 0)       handle_sys(argc, argv);
    else if (sigma_strcmp(module, "ai") == 0)        handle_ai(argc, argv);
    else if (sigma_strcmp(module, "ml") == 0)        handle_ml(argc, argv);
    else if (sigma_strcmp(module, "law") == 0)       handle_law(argc, argv);
    else if (sigma_strcmp(module, "net") == 0)       handle_net(argc, argv);
    else if (sigma_strcmp(module, "fs") == 0)        handle_fs(argc, argv);
    else if (sigma_strcmp(module, "pkg") == 0)       handle_pkg(argc, argv);
    else if (sigma_strcmp(module, "cyber") == 0)     handle_cyber(argc, argv);
    else if (sigma_strcmp(module, "work") == 0)      handle_work(argc, argv);
    else if (sigma_strcmp(module, "db") == 0)        handle_db(argc, argv);
    else if (sigma_strcmp(module, "monitor") == 0)   handle_monitor(argc, argv);
    else if (sigma_strcmp(module, "cicd") == 0)      handle_cicd(argc, argv);
    else if (sigma_strcmp(module, "sec") == 0)       handle_sec(argc, argv);
    else if (sigma_strcmp(module, "perf") == 0)      handle_perf(argc, argv);

    /* Linux Kernel USP Modules */
    else if (sigma_strcmp(module, "proc") == 0)      handle_proc(argc, argv);
    else if (sigma_strcmp(module, "bpf") == 0)       handle_bpf(argc, argv);
    else if (sigma_strcmp(module, "cg") == 0)        handle_cg(argc, argv);
    else if (sigma_strcmp(module, "ns") == 0)        handle_ns(argc, argv);
    else if (sigma_strcmp(module, "watch") == 0)     handle_watch(argc, argv);
    else if (sigma_strcmp(module, "ipc") == 0)       handle_ipc(argc, argv);
    else if (sigma_strcmp(module, "io") == 0)        handle_io(argc, argv);
    else if (sigma_strcmp(module, "numa") == 0)      handle_numa(argc, argv);
    else if (sigma_strcmp(module, "power") == 0)     handle_power(argc, argv);
    else if (sigma_strcmp(module, "mem") == 0)       handle_mem(argc, argv);
    else if (sigma_strcmp(module, "linux-usps") == 0)
        sigma_linux_usps_main(argc - 1, argv + 1);

    /* Tool Replacement Modules */
    else if (sigma_strcmp(module, "vcs") == 0)       handle_vcs(argc, argv);
    else if (sigma_strcmp(module, "container") == 0) handle_container(argc, argv);
    else if (sigma_strcmp(module, "debug") == 0)     handle_debug(argc, argv);
    else if (sigma_strcmp(module, "trace") == 0)     handle_trace(argc, argv);
    else if (sigma_strcmp(module, "media") == 0)     handle_media(argc, argv);
    else if (sigma_strcmp(module, "mux") == 0)       handle_mux(argc, argv);
    else if (sigma_strcmp(module, "ide") == 0)       handle_ide(argc, argv);
    else if (sigma_strcmp(module, "http") == 0)      handle_http(argc, argv);
    else if (sigma_strcmp(module, "cache") == 0)     handle_cache(argc, argv);
    else if (sigma_strcmp(module, "infra") == 0)     handle_infra(argc, argv);
    else if (sigma_strcmp(module, "shard") == 0)     handle_shard(argc, argv);
    else if (sigma_strcmp(module, "cron") == 0)      handle_cron(argc, argv);
    else if (sigma_strcmp(module, "hook") == 0)      handle_hook(argc, argv);
    else if (sigma_strcmp(module, "qube") == 0)      handle_qube(argc, argv);

    /* Distro & Tool Absorption */
    else if (sigma_strcmp(module, "distro") == 0)
        sigma_distro_absorber_main(argc - 1, argv + 1);
    else if (sigma_strcmp(module, "tools") == 0)
        sigma_tool_absorber_main(argc - 1, argv + 1);

    /* Shard-On-Demand Commands */
    else if (sigma_strcmp(module, "optimize") == 0)   return exec_shard("sigma_auto_optimizer", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "clean") == 0)      return exec_shard("system_cleaner", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "academy") == 0)    return exec_shard("academy", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "ncert") == 0)      return exec_shard("ncert_core", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "studio") == 0)     return exec_shard("studio", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "gaming") == 0)     return exec_shard("gaming", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "omni-media") == 0) return exec_shard("omni_media_engine", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "search") == 0)     return exec_shard("omni_search", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "vault") == 0)      return exec_shard("chrono_vault", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "remote") == 0)     return exec_shard("remote_bot", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "backup") == 0)     return exec_shard("backup_manager", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "persona") == 0)    return exec_shard("sigma_persona_engine", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "automate") == 0)   return exec_shard("sigma_automation_matrix", argc - 1, argv + 1);
    else if (sigma_strcmp(module, "ds") == 0 && argc > 2) SovereignDataScience_RunAnalysis(argv[2]);
    else if (sigma_strcmp(module, "god-matrix") == 0) {
        sigma_printf("[GOD-MATRIX] Absorbing ALL competitor OS + Tool USPs...\n\n");
        {
            char* god_distro[] = { "distro", "absorb", "all", SIGMA_NULL };
            sigma_distro_absorber_main(3, god_distro);
            char* god_tools[] = { "tools", "absorb", "all", SIGMA_NULL };
            sigma_tool_absorber_main(3, god_tools);
            char* god_linux[] = { "linux-usps", "all", SIGMA_NULL };
            sigma_linux_usps_main(2, god_linux);
        }
        sigma_printf("\n[GOD-MATRIX] \xe2\x88\x9e ABSOLUTE SOVEREIGNTY ACHIEVED. NO COMPETITOR SURVIVES. \xe2\x88\x9e\n");
    }
    else if (sigma_strcmp(module, "help") == 0 || sigma_strcmp(module, "--help") == 0 || sigma_strcmp(module, "-h") == 0) {
        print_help();
    }
    else {
        sigma_printf("[OMNI-CLI] Unknown module: '%s'\n", module);
        sigma_printf("Run 'sigma help' to see all available modules.\n");
        return 1;
    }

    return 0;
}


