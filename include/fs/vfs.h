/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * vfs.h — SigmaOS Virtual Filesystem Abstraction Layer
 *
 * All filesystem operations go through this layer.
 * Each registered filesystem runs as a user-space shard.
 * The VFS coordinator (init shard) routes path operations to the correct shard.
 *
 * Inspired by: Linux VFS (inode/dentry), Plan 9 9P, Genode VFS
 */

#pragma once
#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
namespace sigmaos { namespace fs {
#endif

/* ── VNode — in-memory inode representation ──────────────────────────────── */

#define VNODE_TYPE_REG   0   /* regular file */
#define VNODE_TYPE_DIR   1   /* directory */
#define VNODE_TYPE_LNK   2   /* symbolic link */
#define VNODE_TYPE_BLK   3   /* block device */
#define VNODE_TYPE_CHR   4   /* char device */
#define VNODE_TYPE_SOCK  5   /* socket */
#define VNODE_TYPE_FIFO  6   /* named pipe */

typedef struct sigma_vnode {
    uint64_t    inode;
    uint32_t    mode;       /* permissions (rwxrwxrwx) | type */
    uint32_t    nlink;
    uint32_t    uid;
    uint32_t    gid;
    uint64_t    size;
    uint64_t    blocks;     /* 512-byte blocks allocated */
    uint64_t    atime;      /* nanoseconds since epoch */
    uint64_t    mtime;
    uint64_t    ctime;
    uint32_t    fs_shard;   /* which filesystem shard owns this */
    uint32_t    rdev;       /* device ID for BLK/CHR */
} sigma_vnode_t;

/* ── Open file ────────────────────────────────────────────────────────────── */

#define O_RDONLY   0
#define O_WRONLY   1
#define O_RDWR     2
#define O_CREAT    (1u << 6)
#define O_TRUNC    (1u << 9)
#define O_APPEND   (1u << 10)
#define O_NONBLOCK (1u << 11)
#define O_CLOEXEC  (1u << 19)

typedef struct sigma_file {
    uint32_t    fd;
    uint64_t    offset;
    uint32_t    flags;
    uint32_t    ref_count;
    sigma_vnode_t vnode;
} sigma_file_t;

/* ── Directory entry ─────────────────────────────────────────────────────── */

typedef struct sigma_dirent {
    uint64_t    inode;
    uint64_t    offset;
    uint16_t    reclen;
    uint8_t     type;
    char        name[256];
} sigma_dirent_t;

/* ── Filesystem operations (C function pointer table) ────────────────────── */

typedef struct sigma_fs_ops {
    int (*mount)   (const char *source, const char *target, uint32_t flags,
                    void *data);
    int (*umount)  (const char *target);
    int (*lookup)  (uint64_t dir_inode, const char *name, sigma_vnode_t *out);
    int (*getattr) (uint64_t inode, sigma_vnode_t *out);
    int (*setattr) (uint64_t inode, const sigma_vnode_t *attrs, uint32_t mask);
    ssize_t (*read)(uint64_t inode, uint64_t offset, void *buf, size_t len);
    ssize_t (*write)(uint64_t inode, uint64_t offset, const void *buf, size_t len);
    int (*create)  (uint64_t dir_inode, const char *name, uint32_t mode,
                    sigma_vnode_t *out);
    int (*unlink)  (uint64_t dir_inode, const char *name);
    int (*mkdir)   (uint64_t dir_inode, const char *name, uint32_t mode);
    int (*rmdir)   (uint64_t dir_inode, const char *name);
    int (*rename)  (uint64_t old_dir, const char *old_name,
                    uint64_t new_dir, const char *new_name);
    int (*readdir) (uint64_t dir_inode, uint64_t *offset,
                    sigma_dirent_t *entries, size_t max);
    int (*symlink) (uint64_t dir_inode, const char *name, const char *target);
    int (*readlink)(uint64_t inode, char *buf, size_t len);
    int (*sync)    (void);
    int (*statfs)  (uint64_t dir_inode, struct sigma_statfs *out);
    int (*truncate)(uint64_t inode, uint64_t new_size);
    int (*chmod)   (uint64_t inode, uint32_t mode);
    int (*chown)   (uint64_t inode, uint32_t uid, uint32_t gid);
} sigma_fs_ops_t;

/* ── Filesystem statfs ───────────────────────────────────────────────────── */

typedef struct sigma_statfs {
    uint64_t    bsize;      /* block size */
    uint64_t    blocks;     /* total blocks */
    uint64_t    bfree;      /* free blocks */
    uint64_t    bavail;     /* available to non-root */
    uint64_t    files;      /* total inodes */
    uint64_t    ffree;      /* free inodes */
    char        fstype[16];
    char        mount_opts[64];
} sigma_statfs_t;

/* ── VFS public API ──────────────────────────────────────────────────────── */

int     sigma_vfs_mount   (const char *device, const char *path,
                            const char *fstype, uint32_t flags);
int     sigma_vfs_umount  (const char *path);
int     sigma_vfs_open    (const char *path, uint32_t flags, sigma_file_t *out);
ssize_t sigma_vfs_read    (sigma_file_t *f, void *buf, size_t count);
ssize_t sigma_vfs_write   (sigma_file_t *f, const void *buf, size_t count);
int     sigma_vfs_close   (sigma_file_t *f);
int     sigma_vfs_stat    (const char *path, sigma_vnode_t *out);
int     sigma_vfs_mkdir   (const char *path, uint32_t mode);
int     sigma_vfs_unlink  (const char *path);
int     sigma_vfs_rename  (const char *old_path, const char *new_path);
int     sigma_vfs_readdir (const char *path, sigma_dirent_t *out, size_t max);
off_t   sigma_vfs_lseek   (sigma_file_t *f, off_t offset, int whence);
int     sigma_vfs_sync    (void);
int     sigma_vfs_register_fs(const char *fstype, const sigma_fs_ops_t *ops);

#ifdef __cplusplus
} /* fs */ } /* sigmaos */
#endif
