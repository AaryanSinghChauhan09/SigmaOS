/*
 * Σ SigmaOS — sigma_mount: Sovereign Filesystem Mount/Unmount Utility
 * Zero-Dependency: No util-linux mount, no libmount.
 * Absorbs: Linux mount(8) concepts — VFS superblock registration.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);

typedef unsigned int u32;

#define FS_TYPE_SIGMA   0
#define FS_TYPE_FAT32   1
#define FS_TYPE_EXT2    2

struct MountEntry {
    char device[32];
    char mountpoint[64];
    int  fs_type;
    u32  flags; // read-only, noexec, etc.
};

#define MAX_MOUNTS 64
static MountEntry mount_table[MAX_MOUNTS];
static int mount_count = 0;

static int str_eq(const char* a, const char* b) {
    int i = 0;
    while (a[i] && b[i]) { if (a[i] != b[i]) return 0; i++; }
    return a[i] == b[i];
}

static void str_copy(char* dst, const char* src, int max) {
    int i = 0;
    while (src[i] && i < max - 1) { dst[i] = src[i]; i++; }
    dst[i] = '\0';
}

static int detect_fs_type(const char* type_str) {
    if (str_eq(type_str, "sigmafs")) return FS_TYPE_SIGMA;
    if (str_eq(type_str, "fat32"))   return FS_TYPE_FAT32;
    if (str_eq(type_str, "vfat"))    return FS_TYPE_FAT32;
    if (str_eq(type_str, "ext2"))    return FS_TYPE_EXT2;
    return -1;
}

extern "C" int sigma_mount(const char* device, const char* mountpoint, const char* type) {
    if (mount_count >= MAX_MOUNTS) {
        sigma_vga_printf("[mount] ERROR: Mount table full.\n");
        return -1;
    }

    int fs = detect_fs_type(type);
    if (fs < 0) {
        sigma_vga_printf("[mount] ERROR: Unknown filesystem type '%s'.\n", type);
        return -1;
    }

    str_copy(mount_table[mount_count].device, device, 32);
    str_copy(mount_table[mount_count].mountpoint, mountpoint, 64);
    mount_table[mount_count].fs_type = fs;
    mount_table[mount_count].flags = 0;
    mount_count++;

    sigma_vga_printf("[mount] %s on %s type %s\n", device, mountpoint, type);
    return 0;
}

extern "C" int sigma_umount(const char* mountpoint) {
    for (int i = 0; i < mount_count; i++) {
        if (str_eq(mount_table[i].mountpoint, mountpoint)) {
            sigma_vga_printf("[umount] Unmounting %s from %s\n", mount_table[i].device, mountpoint);
            for (int j = i; j < mount_count - 1; j++) {
                mount_table[j] = mount_table[j + 1];
            }
            mount_count--;
            return 0;
        }
    }
    sigma_vga_printf("[umount] ERROR: %s not mounted.\n", mountpoint);
    return -1;
}

extern "C" int sigma_mount_list() {
    sigma_vga_printf("Active mounts (%d):\n", mount_count);
    for (int i = 0; i < mount_count; i++) {
        const char* type_str = "unknown";
        if (mount_table[i].fs_type == FS_TYPE_SIGMA) type_str = "sigmafs";
        if (mount_table[i].fs_type == FS_TYPE_FAT32) type_str = "vfat";
        if (mount_table[i].fs_type == FS_TYPE_EXT2)  type_str = "ext2";
        sigma_vga_printf("  %s on %s type %s\n", mount_table[i].device, mount_table[i].mountpoint, type_str);
    }
    return 0;
}

extern "C" int sigma_mount_main(int argc, char** argv) {
    if (argc == 1) return sigma_mount_list();
    if (argc == 4) return sigma_mount(argv[1], argv[2], argv[3]);
    sigma_vga_printf("Usage: mount [device] [mountpoint] [type]\n");
    return 1;
}
