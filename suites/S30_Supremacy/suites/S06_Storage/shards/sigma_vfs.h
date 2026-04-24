/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S06_Storage/shards/sigma_vfs.h
 * =========================================================================
 * Sovereign Virtual Filesystem — gap-closes:
 *   Linux  : VFS layer (dentry, inode, file, superblock)
 *   macOS  : HFS+/APFS VFS switch layer
 *   Windows: IRP/IoStack, NTFS/ReFS filter drivers
 *   BSD    : vnode/vnops, namei, unionfs
 *   Plan 9 : 9P protocol, everything-is-a-file
 * =========================================================================
 */

#ifndef SIGMA_VFS_H
#define SIGMA_VFS_H

typedef unsigned long long vfs_u64;
typedef unsigned int       vfs_u32;
typedef signed   int       vfs_i32;
typedef unsigned short     vfs_u16;
typedef unsigned char      vfs_u8;
typedef unsigned char      vfs_bool;
#define VFS_TRUE  ((vfs_bool)1)
#define VFS_FALSE ((vfs_bool)0)
#define VFS_NULL  ((void*)0)
#define VFS_OK    ((vfs_i32) 0)
#define VFS_ERR   ((vfs_i32)-1)

/* -- Constants ------------------------------------------------------------ */
#define VFS_MAX_PATH      512
#define VFS_MAX_NAME       64
#define VFS_MAX_INODES   8192
#define VFS_MAX_DENTRIES 8192
#define VFS_MAX_FILES     512
#define VFS_MAX_FS_TYPES   16

/* -- File types (inode mode bits) ----------------------------------------- */
#define VFS_IFMT   0xF000
#define VFS_IFREG  0x8000   /* regular file   */
#define VFS_IFDIR  0x4000   /* directory      */
#define VFS_IFLNK  0xA000   /* symbolic link  */
#define VFS_IFSOCK 0xC000   /* socket         */
#define VFS_IFBLK  0x6000   /* block device   */
#define VFS_IFCHR  0x2000   /* char device    */
#define VFS_IFIFO  0x1000   /* FIFO/pipe      */

/* -- Permission bits ------------------------------------------------------- */
#define VFS_PERM_RUSR 0400
#define VFS_PERM_WUSR 0200
#define VFS_PERM_XUSR 0100
#define VFS_PERM_RGRP 0040
#define VFS_PERM_WGRP 0020
#define VFS_PERM_ROTH 0004
#define VFS_PERM_WOTH 0002

/* -- Open flags ------------------------------------------------------------ */
#define VFS_O_RDONLY  0x00
#define VFS_O_WRONLY  0x01
#define VFS_O_RDWR    0x02
#define VFS_O_CREAT   0x40
#define VFS_O_TRUNC   0x200
#define VFS_O_APPEND  0x400
#define VFS_O_NONBLOCK 0x800

/* -- Seek whence ----------------------------------------------------------- */
#define VFS_SEEK_SET  0
#define VFS_SEEK_CUR  1
#define VFS_SEEK_END  2

/* -- Filesystem type (superblock ops) ------------------------------------- */
typedef struct sigma_inode_s sigma_inode_t;
typedef struct sigma_dentry_s sigma_dentry_t;

typedef struct {
    char       name[VFS_MAX_NAME];   /* "sigmafs", "ext4", "tmpfs" etc. */
    vfs_i32  (*mount)(const char *dev, const char *mnt, vfs_u32 flags);
    void     (*unmount)(const char *mnt);
    vfs_i32  (*sync)(void);
    sigma_inode_t *(*root_inode)(void);
} sigma_fs_type_t;

/* -- Inode (Linux inode / BSD vnode equivalent) --------------------------- */
struct sigma_inode_s {
    vfs_u64  ino;           /* inode number                             */
    vfs_u16  mode;          /* type + permissions                       */
    vfs_u32  uid;
    vfs_u32  gid;
    vfs_u64  size;          /* file size in bytes                       */
    vfs_u64  atime;         /* access time (unix epoch ns)              */
    vfs_u64  mtime;         /* modification time                        */
    vfs_u64  ctime;         /* change time                              */
    vfs_u32  nlink;         /* hard link count                          */
    vfs_u32  ref_count;     /* in-memory reference count                */
    vfs_bool dirty;

    /* inode operations (vnops) */
    vfs_i32  (*read) (sigma_inode_t*, void*  buf, vfs_u64 off, vfs_u64 len);
    vfs_i32  (*write)(sigma_inode_t*, const void* buf, vfs_u64 off, vfs_u64 len);
    vfs_i32  (*truncate)(sigma_inode_t*, vfs_u64 new_size);
    void     (*destroy)(sigma_inode_t*);
};

/* -- Dentry (directory entry cache — Linux dcache / BSD namecache) -------- */
struct sigma_dentry_s {
    char             name[VFS_MAX_NAME];
    sigma_inode_t   *inode;
    sigma_dentry_t  *parent;
    sigma_dentry_t  *children[64];
    vfs_u32          child_count;
    vfs_bool         is_mountpoint;
};

/* -- Open file descriptor -------------------------------------------------- */
typedef struct {
    vfs_u32       fd;
    sigma_inode_t *inode;
    vfs_u64       offset;
    vfs_u32       flags;
    vfs_u32       owner_pid;
    vfs_bool      is_open;
} sigma_file_t;

/* -- Public API ----------------------------------------------------------- */
void     sigma_vfs_init(void);
vfs_i32  sigma_vfs_register_fs(sigma_fs_type_t *fs);
vfs_i32  sigma_vfs_mount(const char *dev, const char *mnt, const char *fs_name);
void     sigma_vfs_umount(const char *mnt);

vfs_i32  sigma_vfs_open(const char *path, vfs_u32 flags, vfs_u32 mode, vfs_u32 pid);
vfs_i32  sigma_vfs_close(vfs_u32 fd);
vfs_i32  sigma_vfs_read(vfs_u32 fd,  void *buf, vfs_u64 len);
vfs_i32  sigma_vfs_write(vfs_u32 fd, const void *buf, vfs_u64 len);
vfs_i64  sigma_vfs_seek(vfs_u32 fd,  vfs_i64 offset, vfs_i32 whence);
vfs_i32  sigma_vfs_stat(const char *path, sigma_inode_t *out);
vfs_i32  sigma_vfs_mkdir(const char *path, vfs_u16 mode);
vfs_i32  sigma_vfs_unlink(const char *path);
vfs_i32  sigma_vfs_rename(const char *old, const char *newp);
void     sigma_vfs_ls(const char *path);
void     sigma_vfs_df(void);   /* filesystem usage (/proc/mounts + df) */

typedef signed long long vfs_i64;

#endif /* SIGMA_VFS_H */
