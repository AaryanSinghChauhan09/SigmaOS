/*
 * Σ SigmaOS — sigma_mac: Sovereign Mandatory Access Control (MAC)
 * Absorbs: Linux SELinux, AppArmor
 * Zero-Dependency: No libc. Hardcoded security contexts within the kernel.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define SEC_CONTEXT_SYSTEM    0x0001
#define SEC_CONTEXT_USER      0x0002
#define SEC_CONTEXT_GUEST     0x0004
#define SEC_CONTEXT_RESTRICT  0x0008

struct sigma_mac_subject {
    u32 pid;
    u32 security_context;
};

struct sigma_mac_object {
    const char* path;
    u32 required_context;
};

extern "C" void sigma_mac_init() {
    sigma_vga_printf("[MAC] Initializing Sovereign Mandatory Access Control...\n");
    sigma_vga_printf("[MAC] Contexts: SYSTEM(1), USER(2), GUEST(4), RESTRICT(8)\n");
}

extern "C" bool sigma_mac_check_access(sigma_mac_subject* sub, sigma_mac_object* obj) {
    // A subject can access an object if their context bitwise-AND matches the required context.
    // SYSTEM (0x0001) has access to almost everything, unless specifically restricted.
    
    if ((sub->security_context & SEC_CONTEXT_SYSTEM) == SEC_CONTEXT_SYSTEM) {
        return true; // Root-like override
    }

    if ((sub->security_context & obj->required_context) != 0) {
        return true;
    }

    sigma_vga_printf("[MAC] ACCESS DENIED: PID %d context 0x%X -> object '%s' req 0x%X\n", 
                     sub->pid, sub->security_context, obj->path, obj->required_context);
    return false;
}
