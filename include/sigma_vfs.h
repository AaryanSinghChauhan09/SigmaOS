/* SPDX-License-Identifier: MIT */
/*
 * =========================================================================
 * Σ SIGMAOS: VIRTUAL FILESYSTEM KERNEL INTERFACE (S-VFS)
 * =========================================================================
 * VFS inode, dentry, file operations, variant symlinks (varsyms),
 * and mount flags definitions.
 * =========================================================================
 */

#ifndef SIGMA_VFS_H
#define SIGMA_VFS_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- File Type & Mode Flags --- */
#define SIGMA_S_IFMT   0170000
#define SIGMA_S_IFSOCK 0140000
#define SIGMA_S_IFLNK  0120000
#define SIGMA_S_IFREG  0100000
#define SIGMA_S_IFBLK  0060000
#define SIGMA_S_IFDIR  0040000
#define SIGMA_S_IFCHR  0020000
#define SIGMA_S_IFIFO  0010000

#define SIGMA_S_ISREG(m)  (((m) & SIGMA_S_IFMT) == SIGMA_S_IFREG)
#define SIGMA_S_ISDIR(m)  (((m) & SIGMA_S_IFMT) == SIGMA_S_IFDIR)
#define SIGMA_S_ISLNK(m)  (((m) & SIGMA_S_IFMT) == SIGMA_S_IFLNK)
#define SIGMA_S_ISCHR(m)  (((m) & SIGMA_S_IFMT) == SIGMA_S_IFCHR)
#define SIGMA_S_ISBLK(m)  (((m) & SIGMA_S_IFMT) == SIGMA_S_IFBLK)

/* --- VFS Mount Flags --- */
#define SIGMA_MS_RDONLY      1
#define SIGMA_MS_NOSUID      2
#define SIGMA_MS_NODEV       4
#define SIGMA_MS_NOEXEC      8
#define SIGMA_MS_SYNCHRONOUS 16
#define SIGMA_MS_REMOUNT     32
#define SIGMA_MS_MANDLOCK    64
#define SIGMA_MS_DIRSYNC     128
#define SIGMA_MS_NOATIME     1024
#define SIGMA_MS_NODIRATIME  2048
#define SIGMA_MS_BIND        4096

/* --- Inode Metadata Structure --- */
struct sigma_vfs_inode {
    sigma_u64 i_ino;
    sigma_u32 i_mode;
    sigma_u32 i_nlink;
    sigma_u32 i_uid;
    sigma_u32 i_gid;
    sigma_u64 i_size;
    sigma_u64 i_atime_sec;
    sigma_u64 i_mtime_sec;
    sigma_u64 i_ctime_sec;
    sigma_u32 i_flags;
    void     *i_private;
};

/* --- File Operations Function Table --- */
struct sigma_file_operations {
    sigma_i32 (*open)(struct sigma_vfs_inode *inode, void *file);
    sigma_i32 (*close)(struct sigma_vfs_inode *inode, void *file);
    sigma_s64 (*read)(void *file, char *buf, sigma_size_t count, sigma_u64 *offset);
    sigma_s64 (*write)(void *file, const char *buf, sigma_size_t count, sigma_u64 *offset);
    sigma_s64 (*ioctl)(void *file, sigma_u32 cmd, sigma_uintptr_t arg);
};

/* --- Variant Symlink (Varsym) Template Names --- */
#define SIGMA_VARSYM_SYS  "$SYS"
#define SIGMA_VARSYM_ARCH "$ARCH"
#define SIGMA_VARSYM_USER "$USER"
#define SIGMA_VARSYM_ZONE "$ZONE"

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VFS_H */
