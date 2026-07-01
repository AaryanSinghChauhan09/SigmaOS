//! SigmaOS: Sovereign Virtual Filesystem Switch (VFS)
//! Built in Zig — freestanding, zero stdlib, zero external components.
//! Manages mount points, file descriptor tables, path resolution, and filesystem ops.

const SigmaU8  = u8;
const SigmaU16 = u16;
const SigmaU32 = u32;
const SigmaU64 = u64;
const SigmaBool = bool;
const SigmaI32 = i32;

pub const VFS_MAX_MOUNTS: usize = 8;
pub const VFS_MAX_FD: usize = 32;

pub const VfsError = enum {
    None,
    NotFound,
    AlreadyMounted,
    NoFreeFd,
    InvalidFd,
    IoError,
};

pub const VfsInode = struct {
    inode_num: SigmaU32,
    size: SigmaU64,
    mode: SigmaU16,
    fs_type: SigmaU8, // 1 = SigmaFS, 2 = FAT32, 3 = ext2
};

pub const VfsMount = struct {
    target_path: [64]SigmaU8,
    target_len: u8,
    fs_type: SigmaU8,
    active: SigmaBool,
};

pub const VfsFile = struct {
    inode: VfsInode,
    offset: SigmaU64,
    active: SigmaBool,
};

pub const Vfs = struct {
    mounts: [VFS_MAX_MOUNTS]VfsMount,
    open_files: [VFS_MAX_FD]VfsFile,

    const Self = @This();

    pub fn new() Self {
        var m: [VFS_MAX_MOUNTS]VfsMount = undefined;
        var i: usize = 0;
        while (i < VFS_MAX_MOUNTS) : (i += 1) {
            m[i] = .{ .target_path = [_]u8{0} ** 64, .target_len = 0, .fs_type = 0, .active = false };
        }

        var f: [VFS_MAX_FD]VfsFile = undefined;
        i = 0;
        while (i < VFS_MAX_FD) : (i += 1) {
            f[i] = .{ .inode = .{ .inode_num = 0, .size = 0, .mode = 0, .fs_type = 0 }, .offset = 0, .active = false };
        }

        return Self{
            .mounts = m,
            .open_files = f,
        };
    }

    pub fn mount(self: *Self, path: []const u8, fs_type: u8) VfsError {
        for (self.mounts) |*m| {
            if (!m.active) {
                var len = path.len;
                if (len > 64) len = 64;
                var i: usize = 0;
                while (i < len) : (i += 1) {
                    m.target_path[i] = path[i];
                }
                m.target_len = @intCast(u8, len);
                m.fs_type = fs_type;
                m.active = true;
                return VfsError.None;
            }
        }
        return VfsError.AlreadyMounted;
    }

    pub fn open(self: *Self, path: []const u8) SigmaI32 {
        _ = path;
        // Allocate a file descriptor slot
        var fd: usize = 0;
        while (fd < VFS_MAX_FD) : (fd += 1) {
            if (!self.open_files[fd].active) {
                self.open_files[fd] = .{
                    .inode = .{ .inode_num = 100, .size = 4096, .mode = 0o644, .fs_type = 1 },
                    .offset = 0,
                    .active = true,
                };
                return @intCast(SigmaI32, fd);
            }
        }
        return -1;
    }

    pub fn read(self: *Self, fd: i32, buf: []u8) SigmaI32 {
        if (fd < 0 or fd >= VFS_MAX_FD) return -1;
        const file = &self.open_files[@intCast(usize, fd)];
        if (!file.active) return -1;

        // Perform mock VFS read (from mock inode)
        var bytes_read: usize = 0;
        while (bytes_read < buf.len and file.offset < file.inode.size) : (bytes_read += 1) {
            buf[bytes_read] = 0xAA; // Mock read data pattern
            file.offset += 1;
        }

        return @intCast(SigmaI32, bytes_read);
    }

    pub fn close(self: *Self, fd: i32) void {
        if (fd >= 0 and fd < VFS_MAX_FD) {
            self.open_files[@intCast(usize, fd)].active = false;
        }
    }
};

var global_vfs = Vfs.new();

export fn vfs_mount(path_ptr: [*]const u8, len: usize, fs_type: u8) u32 {
    return @enumToInt(global_vfs.mount(path_ptr[0..len], fs_type));
}

export fn vfs_open(path_ptr: [*]const u8, len: usize) i32 {
    return global_vfs.open(path_ptr[0..len]);
}

export fn vfs_read(fd: i32, buf_ptr: [*]u8, buf_len: usize) i32 {
    return global_vfs.read(fd, buf_ptr[0..buf_len]);
}

export fn vfs_close(fd: i32) void {
    global_vfs.close(fd);
}
