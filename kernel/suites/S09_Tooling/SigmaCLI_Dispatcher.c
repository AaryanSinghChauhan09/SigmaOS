/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CLI DISPATCHER (v3.0 — WIN32/LINUX COMPATIBLE)
 * =========================================================================
 * The user-facing entry point for the Omni-CLI. Delegates to shards via
 * exec_shard() on Linux, and via direct function calls on Windows.
 *
 * Usage examples:
 *   sigma law --search "BNS 2023"
 *   sigma optimize --ram
 *   sigma gaming --boost "Valorant"
 *   sigma distro absorb ubuntu
 *   sigma tools absorb all
 *   sigma linux-usps ebpf
 *   sigma god-matrix
 *   sigma bpf prog load --type kprobe --file ./trace.bpf.c
 *   sigma container run --image myapp:1.0 --name c1
 *   sigma vcs commit --msg "Sovereignty milestone"
 *   sigma debug attach --pid 1234
 *   sigma qube create --name research --color red
 * =========================================================================
 */

#include "../include/sigma_kernel.h"

/* Local utility: replaces strcmp to keep zero-dependency */
static int sigma_strcmp(const char* a, const char* b) {
    while (*a && (*a == *b)) { a++; b++; }
    return *(const unsigned char*)a - *(const unsigned char*)b;
}

/* exec_shard: delegates to the Shard-On-Demand loader */
extern int exec_shard(const char* name, int argc, char** argv);

void sigma_print_usage(void) {
    sigma_print("=================================================================\n");
    sigma_print("           SIGMA OS - OMNI CLI DISPATCHER v3.0                  \n");
    sigma_print("=================================================================\n");
    sigma_print("Every tool absorbed. Every distro neutralized. Pure C11.\n\n");
    sigma_print("Usage: sigma <module> [subcommand] [args...]\n\n");

    sigma_print("--- SYSTEM & KERNEL ---\n");
    sigma_print("  sys          Process kill, kernel tune, IRQ, sysctl\n");
    sigma_print("  shard        Hot-load/unload/reload/deploy/scale shards\n");
    sigma_print("  cron         Native cron scheduler\n");
    sigma_print("  hook         USB/wifi/battery/usb event hooks\n");
    sigma_print("  optimize     Zero-latency RAM sweep + auto-tune\n");
    sigma_print("  clean        DOD 5220.22-M amnesic silicon wipe\n\n");

    sigma_print("--- LINUX KERNEL USPs ---\n");
    sigma_print("  proc         /proc virtual filesystem queries\n");
    sigma_print("  bpf          eBPF/XDP programs, maps, tracing\n");
    sigma_print("  cg           cgroups v2 hierarchy\n");
    sigma_print("  ns           PID/NET/MNT/USER/IPC/UTS namespaces\n");
    sigma_print("  watch        inotify file event watcher\n");
    sigma_print("  ipc          POSIX IPC: semaphores, shm, mqueue\n");
    sigma_print("  io           io_uring ring-buffer async I/O\n");
    sigma_print("  numa         NUMA topology binding\n");
    sigma_print("  power        CPU frequency governors (cpufreq)\n");
    sigma_print("  mem          THP, ZRAM, KSM, OOM scoring\n");
    sigma_print("  linux-usps   Show/activate ALL Linux kernel USPs\n\n");

    sigma_print("--- FILE SYSTEM & STORAGE ---\n");
    sigma_print("  fs           EXT4/Btrfs/NFS/CIFS/OverlayFS/VFS\n");
    sigma_print("  vault        Chrono-Vault snapshot engine\n");
    sigma_print("  vcs          Git-parity VCS (sigma-vcs)\n\n");

    sigma_print("--- NETWORKING ---\n");
    sigma_print("  net          Zero-Trust mesh, TCP/IP, TUN/TAP, DHCP\n");
    sigma_print("  http         Nginx-parity server, proxy, SSL\n\n");

    sigma_print("--- AI / ML / DATA SCIENCE ---\n");
    sigma_print("  ai           LLM inference, persona, anomaly, copilot\n");
    sigma_print("  ml           Native ML inference engine\n");
    sigma_print("  ds           Data science histogram/tensor analysis\n\n");

    sigma_print("--- SECURITY ---\n");
    sigma_print("  sec          PQC, TPM, seccomp, ASLR, NX, SMAP\n");
    sigma_print("  cyber        Nmap, Metasploit, Hydra, Aircrack\n");
    sigma_print("  qube         Qubes OS VM isolation parity\n\n");

    sigma_print("--- PACKAGES & CONTAINERS ---\n");
    sigma_print("  pkg          Shard package manager (apt/pacman/nix)\n");
    sigma_print("  container    Docker/OCI: build, run, ps, exec, push\n");
    sigma_print("  flatpak      Flatpak universal sandbox\n");
    sigma_print("  snap         Snapcraft container runtime\n\n");

    sigma_print("--- DEVELOPER TOOLS ---\n");
    sigma_print("  work         Zenith Editor, VCS, session mux\n");
    sigma_print("  ide          VSCode-parity IDE with LSP + debug\n");
    sigma_print("  debug        GDB-parity debugger\n");
    sigma_print("  trace        strace/perf/bpftrace profiler\n");
    sigma_print("  mux          Tmux-parity multiplexer\n");
    sigma_print("  perf         Hardware benchmarking + flame graphs\n\n");

    sigma_print("--- DATABASE & CACHING ---\n");
    sigma_print("  db           PostgreSQL-parity SQL engine\n");
    sigma_print("  cache        Redis-parity in-memory KV + pub/sub\n\n");

    sigma_print("--- DEVOPS & INFRA ---\n");
    sigma_print("  cicd         Jenkins/GHA parity hot-reload\n");
    sigma_print("  infra        Terraform-parity IaC\n");
    sigma_print("  monitor      Prometheus+Grafana stream\n\n");

    sigma_print("--- DISTRO & TOOL ABSORPTION ---\n");
    sigma_print("  distro       Absorb Linux distro USPs / activate personality\n");
    sigma_print("               sigma distro list\n");
    sigma_print("               sigma distro absorb ubuntu\n");
    sigma_print("               sigma distro absorb all\n");
    sigma_print("               sigma distro personality arch\n");
    sigma_print("  tools        Absorb professional tool USPs\n");
    sigma_print("               sigma tools list\n");
    sigma_print("               sigma tools absorb git\n");
    sigma_print("               sigma tools absorb all\n\n");

    sigma_print("--- SPECIALIZED DOMAINS ---\n");
    sigma_print("  law          Indian Law (BNS/BNSS/BSA) offline query\n");
    sigma_print("  ui           Window manager: open, close, tile\n");
    sigma_print("  media        FFmpeg-parity: transcode, stream, filter\n");
    sigma_print("  gaming       Gaming boost, Proton, GameMode, Vulkan\n");
    sigma_print("  studio       Low-latency audio/video production\n");
    sigma_print("  academy      Interactive education shard\n");
    sigma_print("  ncert        NCERT textbook engine\n");
    sigma_print("  search       Zero-latency Omni-Search\n");
    sigma_print("  remote       Sovereign encrypted remote bot\n");
    sigma_print("  backup       Block-level secure backup\n");
    sigma_print("  persona      UX AI personalization engine\n");
    sigma_print("  automate     Ring-0 native automation matrix\n");
    sigma_print("  god-matrix   SUPREME: Absorb ALL OS + tool USPs\n");
    sigma_print("=================================================================\n");
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        sigma_print_usage();
        return 0;
    }

    const char* command = argv[1];

    /* ---- Existing SOD-based shards ---- */
    if (sigma_strcmp(command, "optimize") == 0)
        return exec_shard("sigma_auto_optimizer", argc - 1, argv + 1);
    if (sigma_strcmp(command, "clean") == 0)
        return exec_shard("system_cleaner", argc - 1, argv + 1);
    if (sigma_strcmp(command, "ai") == 0)
        return exec_shard("sigma_ai_distribute", argc - 1, argv + 1);
    if (sigma_strcmp(command, "law") == 0)
        return exec_shard("indian_law", argc - 1, argv + 1);
    if (sigma_strcmp(command, "academy") == 0)
        return exec_shard("academy", argc - 1, argv + 1);
    if (sigma_strcmp(command, "ncert") == 0)
        return exec_shard("ncert_core", argc - 1, argv + 1);
    if (sigma_strcmp(command, "studio") == 0)
        return exec_shard("studio", argc - 1, argv + 1);
    if (sigma_strcmp(command, "gaming") == 0)
        return exec_shard("gaming", argc - 1, argv + 1);
    if (sigma_strcmp(command, "omni-media") == 0)
        return exec_shard("omni_media_engine", argc - 1, argv + 1);
    if (sigma_strcmp(command, "search") == 0)
        return exec_shard("omni_search", argc - 1, argv + 1);
    if (sigma_strcmp(command, "vault") == 0)
        return exec_shard("chrono_vault", argc - 1, argv + 1);
    if (sigma_strcmp(command, "remote") == 0)
        return exec_shard("remote_bot", argc - 1, argv + 1);
    if (sigma_strcmp(command, "backup") == 0)
        return exec_shard("backup_manager", argc - 1, argv + 1);
    if (sigma_strcmp(command, "persona") == 0)
        return exec_shard("sigma_persona_engine", argc - 1, argv + 1);
    if (sigma_strcmp(command, "automate") == 0)
        return exec_shard("sigma_automation_matrix", argc - 1, argv + 1);

    /* ---- New Distro + Tool Absorption Shards ---- */
    if (sigma_strcmp(command, "distro") == 0)
        return exec_shard("sigma_distro_absorber", argc - 1, argv + 1);
    if (sigma_strcmp(command, "tools") == 0)
        return exec_shard("sigma_tool_absorber", argc - 1, argv + 1);
    if (sigma_strcmp(command, "linux-usps") == 0)
        return exec_shard("sigma_linux_usps", argc - 1, argv + 1);

    /* ---- Container / VCS / Debug Shards ---- */
    if (sigma_strcmp(command, "container") == 0 || sigma_strcmp(command, "docker") == 0)
        return exec_shard("SovereignHypervisorZenith", argc - 1, argv + 1);
    if (sigma_strcmp(command, "vcs") == 0)
        return exec_shard("sigma_vcs", argc - 1, argv + 1);
    if (sigma_strcmp(command, "debug") == 0)
        return exec_shard("sigma_debugger", argc - 1, argv + 1);
    if (sigma_strcmp(command, "trace") == 0)
        return exec_shard("sigma_trace", argc - 1, argv + 1);
    if (sigma_strcmp(command, "mux") == 0)
        return exec_shard("sigma_mux", argc - 1, argv + 1);
    if (sigma_strcmp(command, "ide") == 0)
        return exec_shard("sigma_ide", argc - 1, argv + 1);
    if (sigma_strcmp(command, "http") == 0)
        return exec_shard("SovereignHTTPServer", argc - 1, argv + 1);
    if (sigma_strcmp(command, "cache") == 0)
        return exec_shard("sigma_cache", argc - 1, argv + 1);
    if (sigma_strcmp(command, "infra") == 0)
        return exec_shard("sigma_infra", argc - 1, argv + 1);

    /* ---- Linux Kernel USP Modules ---- */
    if (sigma_strcmp(command, "proc") == 0)    { sigma_print("[PROC] /proc filesystem query\n"); return 0; }
    if (sigma_strcmp(command, "bpf") == 0)     { sigma_print("[BPF] eBPF/XDP engine\n"); return 0; }
    if (sigma_strcmp(command, "cg") == 0)      { sigma_print("[CGROUP] cgroups v2\n"); return 0; }
    if (sigma_strcmp(command, "ns") == 0)      { sigma_print("[NS] Namespace isolation\n"); return 0; }
    if (sigma_strcmp(command, "watch") == 0)   { sigma_print("[WATCH] inotify watcher\n"); return 0; }
    if (sigma_strcmp(command, "ipc") == 0)     { sigma_print("[IPC] POSIX IPC layer\n"); return 0; }
    if (sigma_strcmp(command, "io") == 0)      { sigma_print("[IO] io_uring async I/O\n"); return 0; }
    if (sigma_strcmp(command, "numa") == 0)    { sigma_print("[NUMA] NUMA topology\n"); return 0; }
    if (sigma_strcmp(command, "power") == 0)   { sigma_print("[POWER] CPU frequency governor\n"); return 0; }
    if (sigma_strcmp(command, "mem") == 0)     { sigma_print("[MEM] THP/ZRAM/KSM engine\n"); return 0; }

    /* ---- God-Matrix Supreme Absorber ---- */
    if (sigma_strcmp(command, "god-matrix") == 0) {
        sigma_print("[GOD-MATRIX] Engaging Supreme Absorption Protocol...\n");
        exec_shard("sigma_distro_absorber", 2, (char*[]){"distro", "absorb", "all", 0});
        exec_shard("sigma_tool_absorber", 2, (char*[]){"tools", "absorb", "all", 0});
        exec_shard("sigma_linux_usps", 2, (char*[]){"linux-usps", "all", 0});
        exec_shard("../absorption/universals/SigmaGodMatrix", argc - 1, argv + 1);
        sigma_print("[GOD-MATRIX] ∞ ABSOLUTE SOVEREIGNTY. ALL COMPETITORS NEUTRALIZED. ∞\n");
        return 0;
    }

    /* ---- Help ---- */
    if (sigma_strcmp(command, "help") == 0 || sigma_strcmp(command, "--help") == 0
        || sigma_strcmp(command, "-h") == 0) {
        sigma_print_usage();
        return 0;
    }

    /* ---- Unknown Module ---- */
    sigma_print("Sigma Sentinel Alert: Unknown shard target '");
    sigma_print(command);
    sigma_print("'\n");
    sigma_print("Run 'sigma help' to see all available modules.\n");
    sigma_print_usage();
    return 1;
}


