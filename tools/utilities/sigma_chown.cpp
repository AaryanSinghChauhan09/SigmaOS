/*
 * Σ SigmaOS — sigma_chown: Sovereign File Ownership Modification
 * Zero-Dependency: Interacts directly with the Sovereign VFS.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);

/* VFS Stub */
extern "C" int sigma_vfs_set_owner(const char* path, u32 uid, u32 gid);

/* String to Int helper */
static u32 str_to_u32(const char* str) {
    u32 val = 0;
    while (*str >= '0' && *str <= '9') {
        val = val * 10 + (*str - '0');
        str++;
    }
    return val;
}

extern "C" int sigma_chown_main(int argc, char** argv) {
    if (argc < 3) {
        sigma_vga_printf("Usage: chown <uid>:<gid> <file>\n");
        return 1;
    }

    const char* owner_str = argv[1];
    const char* file = argv[2];
    
    u32 uid = 0, gid = 0;
    int i = 0;
    
    /* Parse UID */
    while (owner_str[i] && owner_str[i] != ':') {
        uid = uid * 10 + (owner_str[i] - '0');
        i++;
    }
    
    /* Parse GID */
    if (owner_str[i] == ':') {
        i++;
        while (owner_str[i]) {
            gid = gid * 10 + (owner_str[i] - '0');
            i++;
        }
    }
    
    sigma_vga_printf("Changing ownership of %s to UID:%u GID:%u...\n", file, uid, gid);
    
    /* Intended VFS interaction */
    sigma_vfs_set_owner(file, uid, gid);

    return 0;
}
