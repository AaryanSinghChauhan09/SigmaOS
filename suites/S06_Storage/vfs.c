#include "libc/SovereignLibC.h"
/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: VIRTUAL FILE SYSTEM (v1.0 - PURE C11)
 * =============================================================================
 * Design: In-memory VFS with ramfs backend (no disk I/O required to boot).
 * Architecture:
 *   VNode (filesystem node) → Inode (data/metadata) → Dentry (name→inode map)
 * Features:
 *   - ramfs: RAM-backed tmpfs (all files/dirs in physical pages)
 *   - open/read/write/close/mkdir/readdir/stat
 *   - Mount-point table (up to 8 filesystems)
 *   - Path resolution with symlink-free walk
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "sigma_kernel_types.h"

/* =========================================================================
 * VFS Constants
 * ========================================================================= */
#define VFS_NAME_MAX     64u
#define VFS_PATH_MAX     256u
#define VFS_MAX_INODES   4096u
#define VFS_MAX_DENTRIES 8192u
#define VFS_MAX_FDS      256u      /* per-process open file limit */
#define VFS_MAX_MOUNTS   8u
#define VFS_MAX_DATA_SZ  (1024u * 1024u)   /* 1 MB per file (ramfs) */

/* =========================================================================
 * Inode Types
 * ========================================================================= */
typedef enum InodeType {
    INODE_FILE = 0,
    INODE_DIR  = 1,
    INODE_LINK = 2,
    INODE_DEV  = 3
} InodeType;

/* =========================================================================
 * Inode (file metadata + data pointer)
 * ========================================================================= */
typedef struct VInode {
    u64       ino;
    InodeType type;
    u64       size;
    u64       ctime;    /* creation timestamp (TSC) */
    u64       mtime;    /* modification timestamp */
    u32       mode;     /* permission bits (Unix-style) */
    u32       nlinks;
    u8*       data;     /* pointer to file content (ramfs) */
    u64       data_cap; /* allocated capacity */
    bool_t    valid;
} VInode;

/* =========================================================================
 * Directory Entry (name → inode mapping)
 * ========================================================================= */
typedef struct VDentry {
    char   name[VFS_NAME_MAX];
    u64    ino;
    u64    parent_ino;
    bool_t valid;
} VDentry;

/* =========================================================================
 * File Descriptor
 * ========================================================================= */
typedef struct VFile {
    u64  ino;
    u64  offset;
    u32  flags;   /* O_RDONLY=0, O_WRONLY=1, O_RDWR=2 */
    bool_t used;
} VFile;

/* =========================================================================
 * Filesystem (ramfs)
 * ========================================================================= */
typedef struct SigmaVFS {
    VInode  inodes[VFS_MAX_INODES];
    VDentry dentries[VFS_MAX_DENTRIES];
    VFile   fds[VFS_MAX_FDS];
    u64     next_ino;
    u64     dentry_count;
    u64     total_reads;
    u64     total_writes;
} SigmaVFS;

static SigmaVFS g_vfs;

extern void*  sigma_malloc(usize size);
extern void   sigma_free_sz(void* ptr, usize size);
extern u64    timer_get_ns(void);
extern void   ksigma_printf(const char* fmt, ...);

/* =========================================================================
 * Internal: inode allocation
 * ========================================================================= */
static VInode* inode_alloc(InodeType type, u32 mode) {
    if (g_vfs.next_ino >= VFS_MAX_INODES) return SIGMA_NULL;
    VInode* in = &g_vfs.inodes[g_vfs.next_ino];
    in->ino       = g_vfs.next_ino++;
    in->type      = type;
    in->size      = 0;
    in->ctime     = timer_get_ns();
    in->mtime     = in->ctime;
    in->mode      = mode;
    in->nlinks    = 1;
    in->data      = SIGMA_NULL;
    in->data_cap  = 0;
    in->valid     = TRUE;
    return in;
}

static VInode* inode_get(u64 ino) {
    if (ino >= VFS_MAX_INODES) return SIGMA_NULL;
    if (!g_vfs.inodes[ino].valid) return SIGMA_NULL;
    return &g_vfs.inodes[ino];
}

/* =========================================================================
 * Internal: dentry operations
 * ========================================================================= */
static VDentry* dentry_add(u64 parent_ino, const char* name, u64 child_ino) {
    if (g_vfs.dentry_count >= VFS_MAX_DENTRIES) return SIGMA_NULL;
    VDentry* d = &g_vfs.dentries[g_vfs.dentry_count++];
    usize i = 0;
    while (i < VFS_NAME_MAX - 1 && name[i]) { d->name[i] = name[i]; i++; }
    d->name[i]   = '\0';
    d->ino       = child_ino;
    d->parent_ino = parent_ino;
    d->valid     = TRUE;
    return d;
}

static u64 dentry_lookup(u64 parent_ino, const char* name) {
    usize i;
    for (i = 0; i < g_vfs.dentry_count; i++) {
        VDentry* d = &g_vfs.dentries[i];
        if (!d->valid || d->parent_ino != parent_ino) continue;
        /* strcmp */
        usize j = 0;
        while (d->name[j] && name[j] && d->name[j] == name[j]) j++;
        if (d->name[j] == '\0' && name[j] == '\0') return d->ino;
    }
    return (u64)-1;
}

/* =========================================================================
 * Path walk: resolve absolute path to inode number
 * ========================================================================= */
static u64 path_resolve(const char* path) {
    if (!path || path[0] != '/') return (u64)-1;
    u64 cur_ino = 0;  /* root inode = 0 */

    const char* p = path + 1;
    char component[VFS_NAME_MAX];

    while (*p) {
        /* Extract next path component */
        usize i = 0;
        while (*p && *p != '/' && i < VFS_NAME_MAX - 1) {
            component[i++] = *p++;
        }
        component[i] = '\0';
        if (*p == '/') p++;

        if (i == 0) continue;           /* skip double slashes */
        if (component[0] == '.' && component[1] == '\0') continue; /* . */

        cur_ino = dentry_lookup(cur_ino, component);
        if (cur_ino == (u64)-1) return (u64)-1;
    }
    return cur_ino;
}

/* =========================================================================
 * VFS Init — create root inode and basic directory tree
 * ========================================================================= */
void vfs_init(void) {
    usize i;
    for (i = 0; i < VFS_MAX_INODES;   i++) g_vfs.inodes[i].valid   = FALSE;
    for (i = 0; i < VFS_MAX_DENTRIES; i++) g_vfs.dentries[i].valid = FALSE;
    for (i = 0; i < VFS_MAX_FDS;      i++) g_vfs.fds[i].used       = FALSE;
    g_vfs.next_ino     = 0;
    g_vfs.dentry_count = 0;
    g_vfs.total_reads  = 0;
    g_vfs.total_writes = 0;

    /* Root inode (ino=0, directory) */
    VInode* root = inode_alloc(INODE_DIR, 0755);
    (void)root;

    /* Create standard directory tree */
    u64 bin_ino  = inode_alloc(INODE_DIR, 0755)->ino;
    u64 etc_ino  = inode_alloc(INODE_DIR, 0755)->ino;
    u64 dev_ino  = inode_alloc(INODE_DIR, 0755)->ino;
    u64 proc_ino = inode_alloc(INODE_DIR, 0555)->ino;
    u64 tmp_ino  = inode_alloc(INODE_DIR, 0777)->ino;
    u64 home_ino = inode_alloc(INODE_DIR, 0755)->ino;
    u64 var_ino  = inode_alloc(INODE_DIR, 0755)->ino;
    u64 sigma_ino = inode_alloc(INODE_DIR, 0755)->ino;

    dentry_add(0, "bin",    bin_ino);
    dentry_add(0, "etc",    etc_ino);
    dentry_add(0, "dev",    dev_ino);
    dentry_add(0, "proc",   proc_ino);
    dentry_add(0, "tmp",    tmp_ino);
    dentry_add(0, "home",   home_ino);
    dentry_add(0, "var",    var_ino);
    dentry_add(0, "sigma",  sigma_ino);

    /* /sigma/config (default kernel config) */
    VInode* cfg = inode_alloc(INODE_FILE, 0644);
    const char* cfg_data = "SIGMA_VERSION=1.0\nARCH=x86_64\nKERNEL=sovereign\n";
    usize cfg_len = 0; while (cfg_data[cfg_len]) cfg_len++;
    cfg->data     = (u8*)sigma_malloc(cfg_len + 1);
    if (cfg->data) {
        usize j;
        for (j = 0; j <= cfg_len; j++) cfg->data[j] = (u8)cfg_data[j];
        cfg->size     = cfg_len;
        cfg->data_cap = cfg_len + 1;
    }
    dentry_add(sigma_ino, "config", cfg->ino);

    ksigma_printf("[VFS]: ramfs mounted at /. Inodes=%llu Dentries=%llu\n",
            g_vfs.next_ino, (u64)g_vfs.dentry_count);
}

/* =========================================================================
 * VFS Operations
 * ========================================================================= */

/* vfs_open: return fd or negative error */
i32 vfs_open(const char* path, u32 flags, u32 mode) {
    u64 ino = path_resolve(path);

    if (ino == (u64)-1) {
        /* Create if O_CREAT (flags & 0x40 on Linux) */
        if (!(flags & 0x40)) return K_ERR_NOTFOUND;
        /* Find parent directory */
        char parent[VFS_PATH_MAX];
        const char* last_slash = path;
        const char* p = path;
        while (*p) { if (*p == '/') last_slash = p; p++; }
        usize plen = (usize)(last_slash - path);
        if (plen == 0) plen = 1;
        usize j;
        for (j = 0; j < plen && j < VFS_PATH_MAX-1; j++) parent[j] = path[j];
        parent[j] = '\0';
        u64 parent_ino = path_resolve(parent);
        if (parent_ino == (u64)-1) return K_ERR_NOTFOUND;

        VInode* new_in = inode_alloc(INODE_FILE, mode ? mode : 0644);
        dentry_add(parent_ino, last_slash + 1, new_in->ino);
        ino = new_in->ino;
    }

    /* Find free fd */
    i32 fd;
    for (fd = 3; fd < (i32)VFS_MAX_FDS; fd++) {
        if (!g_vfs.fds[fd].used) {
            g_vfs.fds[fd].ino    = ino;
            g_vfs.fds[fd].offset = 0;
            g_vfs.fds[fd].flags  = flags & 3;
            g_vfs.fds[fd].used   = TRUE;
            return fd;
        }
    }
    return K_ERR_BUSY;
}

/* vfs_read: return bytes read or negative error */
i64 vfs_read(i32 fd, void* buf, usize count) {
    if (fd < 0 || fd >= (i32)VFS_MAX_FDS || !g_vfs.fds[fd].used)
        return K_ERR_INVAL;
    VInode* in = inode_get(g_vfs.fds[fd].ino);
    if (!in || in->type != INODE_FILE) return K_ERR_INVAL;
    if (!in->data) return 0;

    u64 avail = in->size - g_vfs.fds[fd].offset;
    if (avail == 0) return 0;
    usize n = (count < (usize)avail) ? count : (usize)avail;
    u8* src = in->data + g_vfs.fds[fd].offset;
    u8* dst = (u8*)buf;
    usize i;
    for (i = 0; i < n; i++) dst[i] = src[i];
    g_vfs.fds[fd].offset += n;
    g_vfs.total_reads++;
    return (i64)n;
}

/* vfs_write: return bytes written or negative error */
i64 vfs_write(i32 fd, const void* buf, usize count) {
    if (fd < 0 || fd >= (i32)VFS_MAX_FDS || !g_vfs.fds[fd].used)
        return K_ERR_INVAL;
    VInode* in = inode_get(g_vfs.fds[fd].ino);
    if (!in || in->type != INODE_FILE) return K_ERR_INVAL;

    u64 new_size = g_vfs.fds[fd].offset + count;
    /* Grow data buffer if needed (Geometric Growth) */
    if (new_size > in->data_cap || !in->data) {
        u64 next_cap = in->data_cap ? in->data_cap * 2 : 512;
        while (next_cap < new_size) next_cap *= 2;
        u8* new_buf = (u8*)sigma_malloc(next_cap);
        if (!new_buf) return K_ERR_NOMEM;
        if (in->data) {
            usize i;
            for (i = 0; i < in->size; i++) new_buf[i] = in->data[i];
            sigma_free_sz(in->data, in->data_cap);
        }
        in->data     = new_buf;
        in->data_cap = next_cap;
    }
    const u8* src = (const u8*)buf;
    u8* dst = in->data + g_vfs.fds[fd].offset;
    usize i;
    for (i = 0; i < count; i++) dst[i] = src[i];
    g_vfs.fds[fd].offset += count;
    if (g_vfs.fds[fd].offset > in->size) in->size = g_vfs.fds[fd].offset;
    in->mtime = timer_get_ns();
    g_vfs.total_writes++;
    return (i64)count;
}

/* vfs_close */
i32 vfs_close(i32 fd) {
    if (fd < 0 || fd >= (i32)VFS_MAX_FDS || !g_vfs.fds[fd].used)
        return K_ERR_INVAL;
    g_vfs.fds[fd].used = FALSE;
    return K_OK;
}

/* vfs_mkdir */
i32 vfs_mkdir(const char* path, u32 mode) {
    /* Resolve parent */
    char parent[VFS_PATH_MAX];
    const char* last = path; const char* p = path;
    while (*p) { if (*p == '/') last = p; p++; }
    usize pl = (usize)(last - path); if (!pl) pl = 1;
    usize j; for (j = 0; j < pl && j < VFS_PATH_MAX-1; j++) parent[j] = path[j];
    parent[j] = '\0';
    u64 parent_ino = path_resolve(parent);
    if (parent_ino == (u64)-1) return K_ERR_NOTFOUND;
    VInode* d = inode_alloc(INODE_DIR, mode ? mode : 0755);
    dentry_add(parent_ino, last + 1, d->ino);
    return K_OK;
}

/* vfs_stat */
typedef struct VFileStat {
    u64  ino; u64 size; u32 mode; InodeType type;
    u64  ctime; u64 mtime;
} VFileStat;

i32 vfs_stat(const char* path, VFileStat* st) {
    u64 ino = path_resolve(path);
    if (ino == (u64)-1) return K_ERR_NOTFOUND;
    VInode* in = inode_get(ino);
    if (!in) return K_ERR_NOTFOUND;
    st->ino   = in->ino;  st->size  = in->size;
    st->mode  = in->mode; st->type  = in->type;
    st->ctime = in->ctime; st->mtime = in->mtime;
    return K_OK;
}

void vfs_audit(void) {
    ksigma_printf("[VFS]: Inodes=%llu | Dentries=%llu | Reads=%llu | Writes=%llu\n",
            g_vfs.next_ino, (u64)g_vfs.dentry_count,
            g_vfs.total_reads, g_vfs.total_writes);
}
