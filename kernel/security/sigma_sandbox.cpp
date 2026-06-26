/*
 * Σ SigmaOS — sigma_sandbox: Sovereign Process Sandboxing
 * Zero-Dependency: No seccomp, no AppArmor, no SELinux.
 * Absorbs: Namespace isolation + capability restriction concepts from Linux.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);

typedef unsigned int u32;
typedef unsigned long long u64;

#define CAP_NET        (1 << 0)
#define CAP_FS         (1 << 1)
#define CAP_IPC        (1 << 2)
#define CAP_HW         (1 << 3)
#define CAP_ADMIN      (1 << 4)

/* Resource limits (cgroups-lite) */
struct ResourceLimits {
    u64 max_memory_bytes;
    u32 max_cpu_time_ms;
    u32 used_memory_bytes;
    u32 used_cpu_time_ms;
};

/* Namespace isolation identifiers */
struct Namespaces {
    u32 pid_ns;
    u32 net_ns;
    u32 mnt_ns;
    u32 ipc_ns;
};

/* 
 * The profile dictates what an isolated process is allowed to do.
 */
struct SandboxProfile {
    u32 capabilities;       /* Bitmask of allowed caps */
    u64 allowed_syscalls;   /* Bitmask of allowed syscall IDs (seccomp-lite) */
    ResourceLimits limits;
    Namespaces namespaces;
    char isolated_path[64]; /* Chroot-like path restriction */
    u32 communication_token; /* Token for zero-trust IPC */
};

#define MAX_SANDBOXES 32
static SandboxProfile sandboxes[MAX_SANDBOXES];
static int sandbox_count = 0;

static void str_copy(char* dst, const char* src, int max) {
    int i = 0;
    while (src[i] && i < max - 1) { dst[i] = src[i]; i++; }
    dst[i] = '\0';
}

extern "C" int sigma_sandbox_create(const char* name, u32 caps, u32 mem_kb, int max_procs) {
    if (sandbox_count >= MAX_SANDBOXES) return -1;

    SandboxProfile* s = &sandboxes[sandbox_count];
    s->capabilities = caps;
    s->limits.max_memory_bytes = mem_kb * 1024;
    
    sigma_vga_printf("[sandbox] Created profile '%s' caps=0x%x mem=%dKB\n",
        name, s->capabilities, mem_kb);

    sandbox_count++;
    return 0;
}

extern "C" int sigma_sandbox_init_profile(SandboxProfile* profile) {
    if (!profile) return -1;
    profile->capabilities = 0;
    profile->allowed_syscalls = 0; // Deny all by default
    
    // Default resource limits
    profile->limits.max_memory_bytes = 1024 * 1024 * 16; // 16MB default
    profile->limits.max_cpu_time_ms = 0; // Unlimited
    profile->limits.used_memory_bytes = 0;
    profile->limits.used_cpu_time_ms = 0;
    
    // Default to host namespaces (0)
    profile->namespaces.pid_ns = 0;
    profile->namespaces.net_ns = 0;
    profile->namespaces.mnt_ns = 0;
    profile->namespaces.ipc_ns = 0;
    
    profile->isolated_path[0] = '/';
    profile->isolated_path[1] = '\0';
    profile->communication_token = 0; // Invalid token
    return 0;
}

extern "C" int sigma_sandbox_check_cap(int sandbox_id, u32 cap) {
    if (sandbox_id < 0 || sandbox_id >= sandbox_count) return 0;
    if (sandboxes[sandbox_id].capabilities & cap) return 1;

    sigma_vga_printf("[sandbox] DENIED: Capability 0x%x not allowed\n", cap);
    return 0;
}

extern "C" int sigma_sandbox_list() {
    sigma_vga_printf("Active sandbox profiles (%d):\n", sandbox_count);
    for (int i = 0; i < sandbox_count; i++) {
        sigma_vga_printf("  [%d] %s  caps=0x%x  mem=%dKB  procs=%d\n",
            i, sandboxes[i].name, sandboxes[i].allowed_caps,
            sandboxes[i].memory_limit_kb, sandboxes[i].max_processes);
    }
    return 0;
}
