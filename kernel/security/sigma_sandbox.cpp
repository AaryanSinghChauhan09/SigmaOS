/*
 * Σ SigmaOS — sigma_sandbox: Sovereign Process Sandboxing
 * Zero-Dependency: No seccomp, no AppArmor, no SELinux.
 * Absorbs: Namespace isolation + capability restriction concepts from Linux.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);

typedef unsigned int u32;

#define CAP_NET     (1 << 0)
#define CAP_FS      (1 << 1)
#define CAP_EXEC    (1 << 2)
#define CAP_IPC     (1 << 3)
#define CAP_HW      (1 << 4)
#define CAP_ALL     (CAP_NET | CAP_FS | CAP_EXEC | CAP_IPC | CAP_HW)

struct SandboxProfile {
    char name[32];
    u32  allowed_caps;
    u32  memory_limit_kb;
    int  max_processes;
    bool fs_readonly;
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
    str_copy(s->name, name, 32);
    s->allowed_caps = caps;
    s->memory_limit_kb = mem_kb;
    s->max_processes = max_procs;
    s->fs_readonly = !(caps & CAP_FS);

    sigma_vga_printf("[sandbox] Created profile '%s' caps=0x%x mem=%dKB procs=%d fs=%s\n",
        s->name, s->allowed_caps, s->memory_limit_kb, s->max_processes,
        s->fs_readonly ? "RO" : "RW");

    sandbox_count++;
    return 0;
}

extern "C" int sigma_sandbox_check_cap(int sandbox_id, u32 cap) {
    if (sandbox_id < 0 || sandbox_id >= sandbox_count) return 0;
    if (sandboxes[sandbox_id].allowed_caps & cap) return 1;

    sigma_vga_printf("[sandbox] DENIED: Capability 0x%x not allowed in '%s'\n",
        cap, sandboxes[sandbox_id].name);
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
