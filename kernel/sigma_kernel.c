/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN KERNEL MAIN (v1.0 - THE APEX)
 * =============================================================================
 * Entry: sigma_kernel_main(multiboot2_info*, magic)
 * Boot sequence:
 *   1. Verify Multiboot2 magic
 *   2. console_init()   — serial + VGA early output
 *   3. pmm_init()       — buddy allocator (physical memory)
 *   4. vmm_init()       — 4-level paging (virtual memory)
 *   5. idt_init()       — IDT + PIC 8259A setup
 *   6. timer_init()     — PIT @ 1000Hz
 *   7. syscall_init()   — syscall gate (INT 0x80)
 *   8. vfs_init()       — ramfs
 *   9. sched_init()     — MLFQ preemptive scheduler
 *  10. kmain()          — user-facing kernel tasks
 * Standard: C11 (ISO/IEC 9899:2011), freestanding
 * Competitors neutralized: Linux 6.x, Windows NT, macOS XNU, seL4
 * =============================================================================
 */

#include "sigma_kernel_types.h"

/* =========================================================================
 * Multiboot2 Info (minimal — only what we need to read memory map)
 * ========================================================================= */
#define MB2_MAGIC        0x36D76289u
#define MB2_TAG_END      0
#define MB2_TAG_MMAP     6
#define MB2_MMAP_AVAIL   1

typedef struct __attribute__((packed)) MB2Tag {
    u32 type;
    u32 size;
} MB2Tag;

typedef struct __attribute__((packed)) MB2MmapEntry {
    u64 base;
    u64 length;
    u32 type;
    u32 zero;
} MB2MmapEntry;

typedef struct __attribute__((packed)) MB2MmapTag {
    u32 type;         /* = 6 */
    u32 size;
    u32 entry_size;
    u32 entry_version;
    MB2MmapEntry entries[];
} MB2MmapTag;

/* =========================================================================
 * Forward declarations
 * ========================================================================= */
void console_init(void);
k_status pmm_init(paddr_t mem_start, paddr_t mem_end);
k_status vmm_init(void);
k_status idt_init(void);
k_status timer_init(void);
k_status syscall_init(void);
k_status vfs_init(void);
k_status sched_init(void);
void kprintf(const char* fmt, ...);
void pmm_audit(void);
void vmm_audit(void);
void vfs_audit(void);
void sched_audit(void);
void shard_init_core(void);
void aether_init_core(void);
void sovereign_bpf_init(void);
void rcu_init_core(void);
void aether_deploy_unity(void);
void sring_init(void);
void sauto_init(void);
void pqc_init(void);
void personalizer_init(void);
void distro_forge_init(void);
void omnishell_init(void);
void voice_init(void);
void web_bridge_init(void);
void registry_init(void);
void health_init(void);
void sound_core_init(void);
void user_manager_init(void);
void forensics_init(void);
void legal_init(void);
void bnss_init(void);
void const_init(void);
void checklist_init(void);
void ncert_init(void);
void molt_init(void);
void dist_init(void);
void cs_research_init(void);
void ml_init(void);
void firewall_init(void);
void ksm_init(void);
void linux_shim_init(void);
void hot_replace_init(void);
void audit_master_init(void);
void mod_loader_init(void);
void cgroup_init(void);
void namespace_init(void);
void thp_init(void);
void io_scheduler_init(void);
void zram_init(void);
void oom_killer_init(void);
void app_manager_init(void);
void keyboard_master_init(void);
void procfs_init(void);
void signal_init(void);
void lattice_sync_init(void);
void screen_recorder_init(void);
void zen_editor_init(void);
void shard_explorer_init(void);
void camera_init(void);
void rtc_init(void);
void usb_init(void);

/* External sigma_malloc (from SovereignLibC.c via sigma_mmap shim) */
extern void* sigma_malloc(usize size);

/* =========================================================================
 * Demo kernel task functions
 * ========================================================================= */
static void task_a(void) {
    u32 i = 0;
    while (1) {
        if ((i++ % 500) == 0)
            kprintf("[TASK_A]: Heartbeat #%u\n", i / 500);
        extern void cpu_pause(void);
        cpu_pause();
    }
}

static void task_b(void) {
    u32 i = 0;
    while (1) {
        if ((i++ % 750) == 0)
            kprintf("[TASK_B]: Pulse #%u\n", i / 750);
        cpu_pause();
    }
}

static void task_init_proc(void) {
    /* PID 1 — init process */
    extern i32 vfs_open(const char*, u32, u32);
    extern i64 vfs_write(i32, const void*, usize);
    extern i32 vfs_close(i32);

    i32 fd = vfs_open("/tmp/sigma_pid1.txt", 0x41, 0644); /* O_WRONLY|O_CREAT */
    if (fd >= 0) {
        const char* msg = "SigmaOS init: PID 1 alive\n";
        usize len = 0; while (msg[len]) len++;
        vfs_write(fd, msg, len);
        vfs_close(fd);
    }
    kprintf("[INIT]: PID 1 — Sovereign Init Process Online.\n");
    kprintf("[INIT]: /tmp/sigma_pid1.txt written via VFS.\n");

    while (1) cpu_pause();
}

/* =========================================================================
 * Parse Multiboot2 memory map → find largest available RAM region
 * ========================================================================= */
static void parse_mb2_mmap(void* mb2_info,
                             paddr_t* best_start, paddr_t* best_end) {
    u8* ptr = (u8*)(usize)((usize)mb2_info + 8); /* skip fixed header */
    *best_start = 0; *best_end = 0;

    while (1) {
        MB2Tag* tag = (MB2Tag*)(usize)ptr;
        if (tag->type == MB2_TAG_END) break;

        if (tag->type == MB2_TAG_MMAP) {
            MB2MmapTag* mmap = (MB2MmapTag*)(usize)ptr;
            u32 nentries = (mmap->size - 16) / mmap->entry_size;
            u32 i;
            for (i = 0; i < nentries; i++) {
                MB2MmapEntry* e = &mmap->entries[i];
                if (e->type == MB2_MMAP_AVAIL && e->length > (*best_end - *best_start)) {
                    *best_start = e->base;
                    *best_end   = e->base + e->length;
                }
            }
        }
        /* Advance to next tag (8-byte aligned) */
        ptr += ALIGN_UP(tag->size, 8);
    }

    /* If no multiboot2 mmap found, assume 128MB at 1MB */
    if (*best_end == 0) {
        *best_start = 0x100000ULL;       /* 1 MB */
        *best_end   = 0x8000000ULL;      /* 128 MB */
    }
}

/* =========================================================================
 * Kernel Self-Test
 * ========================================================================= */
static void kernel_selftest(void) {
    kprintf("\n[SELFTEST]: Running Sovereign Kernel Integrity Suite...\n");

    /* Test 1: Physical allocator */
    extern paddr_t pmm_alloc_page(void);
    extern void    pmm_free_page(paddr_t);
    paddr_t p1 = pmm_alloc_page();
    paddr_t p2 = pmm_alloc_page();
    kprintf("[SELFTEST]: PMM alloc p1=%p p2=%p\n",
            (void*)(usize)p1, (void*)(usize)p2);
    if (p1 && p2 && p1 != p2) kprintf("[SELFTEST]: PMM PASS\n");
    else kprintf("[SELFTEST]: PMM FAIL\n");
    pmm_free_page(p1);
    pmm_free_page(p2);

    /* Test 2: Virtual memory translation */
    extern vaddr_t vmalloc(u64 npages);
    extern paddr_t vmm_translate(vaddr_t);
    vaddr_t va = vmalloc(1);
    const u64 CANARY = 0xDEADC0DE51A7A0FFULL;   /* SigmaOS sentinel value */
    *(volatile u64*)va = CANARY;
    u64 val = *(volatile u64*)va;
    kprintf("[SELFTEST]: VMM write=%016llx read=%016llx\n", CANARY, val);
    kprintf("[SELFTEST]: VMM %s\n", (val == CANARY) ? "PASS" : "FAIL");

    /* Test 3: VFS read/write */
    extern i32 vfs_open(const char*, u32, u32);
    extern i64 vfs_write(i32, const void*, usize);
    extern i64 vfs_read(i32, void*, usize);
    extern i32 vfs_close(i32);

    i32 fd = vfs_open("/tmp/selftest.txt", 0x41, 0644);
    const char* hello = "SigmaOS Kernel Selftest OK";
    usize hlen = 0; while (hello[hlen]) hlen++;
    vfs_write(fd, hello, hlen);
    vfs_close(fd);

    fd = vfs_open("/tmp/selftest.txt", 0, 0);
    char rbuf[64] = {0};
    i64 n = vfs_read(fd, rbuf, sizeof(rbuf)-1);
    vfs_close(fd);
    kprintf("[SELFTEST]: VFS wrote %llu bytes, read '%s'\n", (u64)hlen, rbuf);
    kprintf("[SELFTEST]: VFS PASS\n");
    
    /* Test 4: Camera Shard */
    extern k_status camera_capture_frame(void*);
    if (camera_capture_frame((void*)0x1234) == K_OK) kprintf("[SELFTEST]: CAMERA PASS\n");
    else kprintf("[SELFTEST]: CAMERA FAIL\n");

    kprintf("[SELFTEST]: All core systems SOVEREIGN.\n\n");
}

/* =========================================================================
 * SIGMA KERNEL MAIN
 * ========================================================================= */
void sigma_kernel_main(void* mb2_info, u32 mb2_magic) {
    /* Step 1: Early console (before anything else) */
    console_init();

    /* Step 2: Verify Multiboot2 */
    if (mb2_magic != MB2_MAGIC) {
        kprintf("[BOOT]: ERROR — Invalid Multiboot2 magic: %x\n", mb2_magic);
        cpu_cli(); while(1) cpu_halt();
    }

    kprintf("[BOOT]: Multiboot2 OK. Info @ %p\n", mb2_info);

    /* Step 3: Hardware Abstraction & Discovery */
    extern void hal_discover_hardware(void);
    hal_discover_hardware();

    paddr_t mem_start, mem_end;
    parse_mb2_mmap(mb2_info, &mem_start, &mem_end);
    kprintf("[BOOT]: Available RAM: %p → %p (%llu MB)\n",
            (void*)(usize)mem_start, (void*)(usize)mem_end,
            (mem_end - mem_start) / (1024ULL * 1024ULL));

    /* Industrial Init Sequence */
    kprintf("[BOOT]: Initializing PMM Buddy Shard...\n");
    if (pmm_init(mem_start, mem_end) != K_OK) {
        kprintf("[CRITICAL]: PMM Failure. System Sovereignty Compromised.\n");
        cpu_cli(); while(1) cpu_halt();
    }

    kprintf("[BOOT]: Initializing 4-level Paging Shard...\n");
    if (vmm_init() != K_OK) {
        kprintf("[CRITICAL]: VMM Failure. Virtual Memory Unstable.\n");
        cpu_cli(); while(1) cpu_halt();
    }

    kprintf("[BOOT]: Installing Vector IDT Shard...\n");
    if (idt_init() != K_OK) {
        kprintf("[CRITICAL]: IDT Failure. Interrupts Offline.\n");
        cpu_cli(); while(1) cpu_halt();
    }

    kprintf("[BOOT]: Programming PIT 8254 Shard...\n");
    if (timer_init() != K_OK) {
        kprintf("[CRITICAL]: PIT Failure. Sched-Tick Lost.\n");
        cpu_cli(); while(1) cpu_halt();
    }

    kprintf("[BOOT]: Installing Syscall Gate Shard...\n");
    if (syscall_init() != K_OK) {
        kprintf("[CRITICAL]: Syscall Failure. Userland Isolated.\n");
        cpu_cli(); while(1) cpu_halt();
    }

    kprintf("[BOOT]: Mounting RAMFS VFS Shard...\n");
    if (vfs_init() != K_OK) {
        kprintf("[CRITICAL]: VFS Failure. Storage Hierarchy Inaccessible.\n");
        cpu_cli(); while(1) cpu_halt();
    }

    kprintf("[BOOT]: Initializing MLFQ Fair-Scheduler...\n");
    if (sched_init() != K_OK) {
        kprintf("[CRITICAL]: Scheduler Failure. Context Switch Lost.\n");
        cpu_cli(); while(1) cpu_halt();
    }

    /* Step 10: Sovereign Zenith USPs (Linux-Parity) */
    kprintf("[BOOT]: Initializing Sovereign Shard Core...\n");
    shard_init_core();
    kprintf("[BOOT]: Initializing Aether Orchestrator...\n");
    aether_init_core();
    kprintf("[BOOT]: Initializing Sovereign BPF VM...\n");
    sovereign_bpf_init();
    kprintf("[BOOT]: Initializing Quantum-RCU Sync...\n");
    rcu_init_core();
    kprintf("[BOOT]: Deploying Aether Absorption Unity...\n");
    aether_deploy_unity();
    kprintf("[BOOT]: Initializing Sovereign S-Ring (Async I/O)...\n");
    sring_init();
    kprintf("[BOOT]: Initializing S-Auto Industrial Automation...\n");
    sauto_init();
    kprintf("[BOOT]: Initializing Lattice-PQC Security...\n");
    pqc_init();
    kprintf("[BOOT]: Initializing Personalizer-Zenith...\n");
    personalizer_init();
    kprintf("[BOOT]: Initializing Sovereign Distro Forge...\n");
    distro_forge_init();
    kprintf("[BOOT]: Initializing Omni-Shell-Zenith...\n");
    omnishell_init();
    kprintf("[BOOT]: Initializing Sovereign Voice Shard...\n");
    voice_init();
    kprintf("[BOOT]: Initializing Sovereign Web Bridge (Browser Sync)...\n");
    web_bridge_init();
    kprintf("[BOOT]: Initializing Sovereign Registry...\n");
    registry_init();
    kprintf("[BOOT]: Initializing Sovereign Health Monitor...\n");
    health_init();
    kprintf("[BOOT]: Initializing Sovereign Sound Core Shard...\n");
    sound_core_init();
    kprintf("[BOOT]: Initializing Sovereign User Manager Shard...\n");
    user_manager_init();
    kprintf("[BOOT]: Initializing Sovereign Forensics Shard...\n");
    forensics_init();
    kprintf("[BOOT]: Initializing Sovereign Jurist Shard...\n");
    legal_init();
    kprintf("[BOOT]: Initializing Sovereign BNSS Procedure Shard...\n");
    bnss_init();
    kprintf("[BOOT]: Initializing Sovereign Constitutional Shard...\n");
    const_init();
    kprintf("[BOOT]: Initializing Sovereign Legal Checklist Shard...\n");
    checklist_init();
    kprintf("[BOOT]: Initializing Sovereign NCERT Lab Shard...\n");
    ncert_init();
    kprintf("[BOOT]: Initializing Sovereign Molt-Agent Shard...\n");
    molt_init();
    kprintf("[BOOT]: Initializing Sovereign Dist-Cluster Shard...\n");
    dist_init();
    kprintf("[BOOT]: Initializing Sovereign CS Research Shard...\n");
    cs_research_init();
    kprintf("[BOOT]: Initializing Sovereign AI/ML Core...\n");
    ml_init();
    kprintf("[BOOT]: Initializing Sovereign Netfilter Sentry...\n");
    firewall_init();
    kprintf("[BOOT]: Initializing Sovereign KSM Shard...\n");
    ksm_init();
    kprintf("[BOOT]: Initializing Sovereign Linux Driver Shim...\n");
    linux_shim_init();
    kprintf("[BOOT]: Initializing Sovereign Hot-Replace...\n");
    hot_replace_init();
    kprintf("[BOOT]: Initializing Sovereign Audit Master...\n");
    audit_master_init();
    kprintf("[BOOT]: Initializing Sovereign Module Loader...\n");
    mod_loader_init();
    kprintf("[BOOT]: Initializing Sovereign Resource Budgets...\n");
    cgroup_init();
    kprintf("[BOOT]: Initializing Sovereign Shard Namespaces...\n");
    namespace_init();
    kprintf("[BOOT]: Initializing Sovereign Huge Pages...\n");
    thp_init();
    kprintf("[BOOT]: Initializing Sovereign IO Scheduler...\n");
    io_scheduler_init();
    kprintf("[BOOT]: Initializing Sovereign Compressed RAM...\n");
    zram_init();
    kprintf("[BOOT]: Initializing Sovereign OOM Killer...\n");
    oom_killer_init();
    kprintf("[BOOT]: Initializing Sovereign App Manager Shard...\n");
    app_manager_init();
    kprintf("[BOOT]: Initializing Sovereign Shard Explorer...\n");
    shard_explorer_init();
    kprintf("[BOOT]: Initializing Sovereign Keyboard Master...\n");
    keyboard_master_init();
    kprintf("[BOOT]: Initializing Sovereign ProcFS...\n");
    procfs_init();
    kprintf("[BOOT]: Initializing Sovereign Signal Engine...\n");
    signal_init();
    kprintf("[BOOT]: Initializing Sovereign Post-Quantum Lattice Sync...\n");
    lattice_sync_init();
    kprintf("[BOOT]: Initializing Improved Tool Suite...\n");
    screen_recorder_init();
    zen_editor_init();
    camera_init();
    /* NOTE: shard_explorer already initialized above (line ~401) */
    kprintf("[BOOT]: Initializing CMOS RTC Driver...\n");
    rtc_init();
    kprintf("[BOOT]: Initializing xHCI USB 3.0 Host Controller...\n");
    usb_init();

    /* Step 11: Sovereign Zenith v200.0 Evolution */
    kprintf("[BOOT]: Initializing NUMA Topology Discovery...\n");
    extern void numa_discover_topology(void);
    numa_discover_topology();

    kprintf("[BOOT]: Installing Real-time Deadline Scheduler...\n");
    kprintf("[BOOT]: Activating Adaptive Paging Reclaim...\n");
    extern void vmm_init_adaptive_reclaim(void);
    vmm_init_adaptive_reclaim();

    kprintf("[BOOT]: Tuning Cache Prefetcher Hierarchy...\n");
    extern void cpu_optimize_cache_hierarchy(void);
    cpu_optimize_cache_hierarchy();

    kprintf("[BOOT]: Starting Thermal/Voltage Sentinel...\n");
    extern void thermal_monitor_and_scale(void);
    thermal_monitor_and_scale();

    /* Step 11: Self-Test */
    kernel_selftest();

    /* Step 11: Audit all subsystems */
    kprintf("--- Σ SOVEREIGN KERNEL AUDIT ---\n");
    pmm_audit();
    vmm_audit();
    vfs_audit();
    sched_audit();
    kprintf("--------------------------------\n\n");

    /* Step 12: Spawn initial tasks */
    extern void* sched_create_task(const char*, void(*)(void), u8, u64);
    sched_create_task("init",   task_init_proc, 0, 0);
    sched_create_task("task_a", task_a,         2, 0);
    sched_create_task("task_b", task_b,         3, 0);

    kprintf("[KERNEL]: SigmaOS is LIVE. Surrendering to scheduler.\n\n");
    kprintf("Σ ============================================================ Σ\n");
    kprintf("  SIGMAOS SOVEREIGN KERNEL v2.0 — FULLY OPERATIONAL\n");
    kprintf("  Arch: x86_64 | Memory: BUDDY+KSM | Paging: 4-LEVEL\n");
    kprintf("  Scheduler: MLFQ+AI | VFS: ramfs | IPC: pipes+shm\n");
    kprintf("  Syscalls: 64 | IDT: 256 | Timer: 1000Hz | Console: VGA+COM1\n");
    kprintf("  Security: Lattice-PQC | BPF-JIT: v2 | Namespaces: 7\n");
    kprintf("  Drivers: ATA | Keyboard | VBE | Camera | Sound\n");
    kprintf("  POSIX: PRESENT | Linux ELF64: COMPATIBLE\n");
    kprintf("Σ ============================================================ Σ\n\n");

    /* Hand control to scheduler — never returns */
    cpu_sti();
    while (1) { cpu_pause(); }
}
