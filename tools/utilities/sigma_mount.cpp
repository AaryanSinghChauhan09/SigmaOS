/*
 * Σ SigmaOS Zenith — mount Utility
 * Absorbs: util-linux mount, busybox mount
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" bool sigma_ext2_mount(u32 partition_lba);
extern "C" bool sigma_fat32_mount(u32 partition_lba);

static bool sh_streq(const char* a, const char* b) {
    while (*a && *b && *a == *b) { a++; b++; }
    return *a == *b;
}

static u32 sigma_atou(const char* s) {
    u32 n = 0;
    while (*s >= '0' && *s <= '9') n = n * 10 + (*s++ - '0');
    return n;
}

extern "C" int sigma_mount_main(int argc, char** argv) {
    if (argc < 4) {
        sigma_vga_printf("Usage: mount -t <fstype> <lba>\n");
        sigma_vga_printf("  Supported: ext2, fat32\n");
        return 1;
    }

    if (!sh_streq(argv[1], "-t")) {
        sigma_vga_printf("mount: expected -t <fstype>\n");
        return 1;
    }

    const char* fstype = argv[2];
    u32 lba = sigma_atou(argv[3]);

    if (sh_streq(fstype, "ext2")) {
        if (sigma_ext2_mount(lba)) {
            sigma_vga_printf("mount: ext2 mounted at LBA %u\n", lba);
            return 0;
        }
    } else if (sh_streq(fstype, "fat32")) {
        if (sigma_fat32_mount(lba)) {
            sigma_vga_printf("mount: fat32 mounted at LBA %u\n", lba);
            return 0;
        }
    } else {
        sigma_vga_printf("mount: unknown filesystem '%s'\n", fstype);
        return 1;
    }

    sigma_vga_printf("mount: failed\n");
    return 1;
}
