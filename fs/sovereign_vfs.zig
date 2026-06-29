// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Virtual Filesystem (Zig, no stdlib)
//! Replaces: include/fs/sigma_vfs.h, include/fs/vfs.h, include/kernel/sigma_vfs.h
//! =========================================================================

pub const VFS_MAX_PATH: usize = 256;
pub const VFS_MAX_FILES: usize = 1024;

pub const FileType = enum(u8) {
    Regular   = 0,
    Directory = 1,
    Symlink   = 2,
    Device    = 3,
    Pipe      = 4,
};

pub const VfsInode = struct {
    ino:       u64,
    file_type: FileType,
    size:      u64,
    mode:      u32,
    uid:       u32,
    gid:       u32,

    pub fn is_dir(self: *const VfsInode) bool {
        return self.file_type == FileType.Directory;
    }

    pub fn class_name(self: *const VfsInode) []const u8 {
        _ = self;
        return "VfsInode";
    }
};

pub const VfsDentry = struct {
    name: [VFS_MAX_PATH]u8,
    name_len: usize,
    inode: VfsInode,

    pub fn class_name(self: *const VfsDentry) []const u8 {
        _ = self;
        return "VfsDentry";
    }
};

pub const VfsMount = struct {
    mount_point: [VFS_MAX_PATH]u8,
    dentry: VfsDentry,
    mounted: bool,

    pub fn new() VfsMount {
        return VfsMount{
            .mount_point = [_]u8{0} ** VFS_MAX_PATH,
            .dentry      = undefined,
            .mounted     = false,
        };
    }

    pub fn mount(self: *VfsMount, path: []const u8) bool {
        var i: usize = 0;
        while (i < path.len and i < VFS_MAX_PATH - 1) : (i += 1) {
            self.mount_point[i] = path[i];
        }
        self.mount_point[i] = 0;
        self.mounted = true;
        return true;
    }

    pub fn umount(self: *VfsMount) void {
        self.mounted = false;
    }
};
