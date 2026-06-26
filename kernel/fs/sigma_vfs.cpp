/*
 * Σ SigmaOS — sigma_vfs: Sovereign Virtual Filesystem Layer
 * Zero-Dependency: No POSIX VFS, no Linux struct inode.
 * Absorbs: Linux VFS superblock/inode/dentry architecture, Plan 9 namespace model.
 * Implements: Unified mountpoint table, file descriptor abstraction, path resolution.
 */

typedef unsigned int   u32;
typedef unsigned char  u8;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define SIGMA_VFS_MAX_MOUNTS 16
#define SIGMA_VFS_MAX_FDS    256
#define SIGMA_VFS_PATH_MAX   256

/* Filesystem operation function pointers (sovereign VFS ops table) */
struct SigmaFSOps {
    int  (*open)(const char* path, u32 flags);
    int  (*read)(int fd, u8* buf, u32 len);
    int  (*write)(int fd, const u8* buf, u32 len);
    int  (*close)(int fd);
    int  (*stat)(const char* path, u32* size_out, u32* mode_out);
    void* (*opendir)(const char* path);
    const char* (*readdir)(void* dir);
    void (*closedir)(void* dir);
    int  (*set_owner)(const char* path, u32 uid, u32 gid);
};

/* Mount Point */
struct SigmaMountPoint {
    char prefix[SIGMA_VFS_PATH_MAX];  /* e.g. "/" or "/mnt/usb" */
    u32  prefix_len;
    SigmaFSOps* ops;
    bool active;
};

/* File Descriptor */
struct SigmaFD {
    u32  mount_idx;
    u32  internal_fd;  /* FS-specific handle */
    u32  flags;
    u64  offset;
    bool active;
};

static SigmaMountPoint mount_table[SIGMA_VFS_MAX_MOUNTS];
static SigmaFD         fd_table[SIGMA_VFS_MAX_FDS];

/* String helpers */
static u32 sv_strlen(const char* s) { u32 n = 0; while (s[n]) n++; return n; }
static bool sv_starts_with(const char* str, const char* prefix, u32 plen) {
    for (u32 i = 0; i < plen; i++)
        if (str[i] != prefix[i]) return false;
    return true;
}

/* Mount a filesystem at a path prefix */
extern "C" int sigma_vfs_mount(const char* prefix, SigmaFSOps* ops) {
    for (int i = 0; i < SIGMA_VFS_MAX_MOUNTS; i++) {
        if (!mount_table[i].active) {
            u32 len = sv_strlen(prefix);
            if (len >= SIGMA_VFS_PATH_MAX) return -1;
            for (u32 j = 0; j <= len; j++) mount_table[i].prefix[j] = prefix[j];
            mount_table[i].prefix_len = len;
            mount_table[i].ops = ops;
            mount_table[i].active = true;
            sigma_vga_printf("[VFS] Mounted filesystem at %s\n", prefix);
            return 0;
        }
    }
    return -1;
}

/* Resolve path to the correct mount and delegate open */
extern "C" int sigma_vfs_open(const char* path, u32 flags) {
    /* Find longest-prefix mount */
    int best = -1;
    u32 best_len = 0;
    for (int i = 0; i < SIGMA_VFS_MAX_MOUNTS; i++) {
        if (mount_table[i].active &&
            sv_starts_with(path, mount_table[i].prefix, mount_table[i].prefix_len) &&
            mount_table[i].prefix_len > best_len) {
            best = i;
            best_len = mount_table[i].prefix_len;
        }
    }
    if (best < 0) return -1;

    /* Find free FD */
    for (int f = 0; f < SIGMA_VFS_MAX_FDS; f++) {
        if (!fd_table[f].active) {
            const char* rel_path = path + best_len;
            int ifd = mount_table[best].ops->open(rel_path, flags);
            if (ifd < 0) return -1;
            fd_table[f].mount_idx = best;
            fd_table[f].internal_fd = ifd;
            fd_table[f].flags = flags;
            fd_table[f].offset = 0;
            fd_table[f].active = true;
            return f;
        }
    }
    return -1;
}

/* Read through VFS */
extern "C" int sigma_vfs_read(int fd, u8* buf, u32 len) {
    if (fd < 0 || fd >= SIGMA_VFS_MAX_FDS || !fd_table[fd].active) return -1;
    return mount_table[fd_table[fd].mount_idx].ops->read(fd_table[fd].internal_fd, buf, len);
}

/* Write through VFS */
extern "C" int sigma_vfs_write(int fd, const u8* buf, u32 len) {
    if (fd < 0 || fd >= SIGMA_VFS_MAX_FDS || !fd_table[fd].active) return -1;
    return mount_table[fd_table[fd].mount_idx].ops->write(fd_table[fd].internal_fd, buf, len);
}

/* Close FD */
extern "C" int sigma_vfs_close(int fd) {
    if (fd < 0 || fd >= SIGMA_VFS_MAX_FDS || !fd_table[fd].active) return -1;
    mount_table[fd_table[fd].mount_idx].ops->close(fd_table[fd].internal_fd);
    fd_table[fd].active = false;
    return 0;
}

/* Opendir/readdir/closedir delegates */
extern "C" void* sigma_vfs_opendir(const char* path) {
    for (int i = 0; i < SIGMA_VFS_MAX_MOUNTS; i++) {
        if (mount_table[i].active &&
            sv_starts_with(path, mount_table[i].prefix, mount_table[i].prefix_len)) {
            if (mount_table[i].ops->opendir)
                return mount_table[i].ops->opendir(path + mount_table[i].prefix_len);
        }
    }
    return 0;
}
extern "C" const char* sigma_vfs_readdir(void* dir) {
    /* Stubbed: would need mount context tracking */
    return 0;
}
extern "C" void sigma_vfs_closedir(void* dir) {}

/* Ownership change delegate */
extern "C" int sigma_vfs_set_owner(const char* path, u32 uid, u32 gid) {
    for (int i = 0; i < SIGMA_VFS_MAX_MOUNTS; i++) {
        if (mount_table[i].active &&
            sv_starts_with(path, mount_table[i].prefix, mount_table[i].prefix_len)) {
            if (mount_table[i].ops->set_owner)
                return mount_table[i].ops->set_owner(path + mount_table[i].prefix_len, uid, gid);
        }
    }
    return -1;
}
