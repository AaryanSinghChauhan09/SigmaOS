// sigma_fs.zig — SigmaFS: Sovereign Filesystem Driver (Zig)
// Language: Zig (no imports except @import("builtin") for safety)
// OOP: FileSystem "interface" via vtable struct; SigmaExt4 (concrete)
// Specification: docs/.kiro/specs/sigmaos-roadmap/design.md (SigmaFS section)

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 1. Primitive types (explicit — no std imports)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

const U8    = u8;
const U16   = u16;
const U32   = u32;
const U64   = u64;
const Usize = usize;
const Bool  = bool;
const Inode = U64;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 2. Fixed-capacity string (no heap)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

const NAME_MAX: Usize = 255;

pub const FixedStr = struct {
    bytes: [NAME_MAX]U8 = [_]U8{0} ** NAME_MAX,
    len:   Usize = 0,

    pub fn fromSlice(s: []const U8) FixedStr {
        var result = FixedStr{};
        const n = if (s.len < NAME_MAX) s.len else NAME_MAX;
        var i: Usize = 0;
        while (i < n) : (i += 1) {
            result.bytes[i] = s[i];
        }
        result.len = n;
        return result;
    }

    pub fn eql(self: FixedStr, other: FixedStr) Bool {
        if (self.len != other.len) return false;
        var i: Usize = 0;
        while (i < self.len) : (i += 1) {
            if (self.bytes[i] != other.bytes[i]) return false;
        }
        return true;
    }
};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 3. InodeFlags and FileKind
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

pub const FileKind = enum(U8) {
    Unknown   = 0,
    Regular   = 1,
    Directory = 2,
    SymLink   = 3,
    CharDev   = 4,
    BlockDev  = 5,
    Fifo      = 6,
    Socket    = 7,
};

pub const InodeFlags = packed struct {
    read_only:      Bool = false,
    immutable:      Bool = false,
    append_only:    Bool = false,
    encrypted:      Bool = false,
    cow_enabled:    Bool = false,
    _pad:           u3   = 0,
};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 4. DirEntry and InodeMeta
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

pub const DirEntry = struct {
    inode:    Inode,
    kind:     FileKind,
    name:     FixedStr,
};

pub const InodeMeta = struct {
    inode:      Inode,
    kind:       FileKind,
    size:       U64,
    link_count: U32,
    uid:        U32,
    gid:        U32,
    flags:      InodeFlags,
    atime:      U64,   // UNIX seconds
    mtime:      U64,
    ctime:      U64,
    block_size: U32,
};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 5. FileSystem vtable (OOP "interface" via comptime vtable)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

pub const FsError = error{
    NotFound,
    NoSpace,
    PermissionDenied,
    Corrupted,
    NotDirectory,
    NotFile,
    AlreadyExists,
    IoError,
};

pub const FileSystemVtable = struct {
    lookupFn:  *const fn (ctx: *anyopaque, parent: Inode, name: FixedStr) FsError!InodeMeta,
    readFn:    *const fn (ctx: *anyopaque, inode: Inode, offset: U64, buf: []U8) FsError!Usize,
    writeFn:   *const fn (ctx: *anyopaque, inode: Inode, offset: U64, data: []const U8) FsError!Usize,
    mkdirFn:   *const fn (ctx: *anyopaque, parent: Inode, name: FixedStr) FsError!Inode,
    unlinkFn:  *const fn (ctx: *anyopaque, parent: Inode, name: FixedStr) FsError!void,
    statFn:    *const fn (ctx: *anyopaque, inode: Inode) FsError!InodeMeta,
};

pub const FileSystem = struct {
    ctx:    *anyopaque,
    vtable: *const FileSystemVtable,

    pub fn lookup(self: FileSystem, parent: Inode, name: FixedStr) FsError!InodeMeta {
        return self.vtable.lookupFn(self.ctx, parent, name);
    }
    pub fn read(self: FileSystem, inode: Inode, offset: U64, buf: []U8) FsError!Usize {
        return self.vtable.readFn(self.ctx, inode, offset, buf);
    }
    pub fn write(self: FileSystem, inode: Inode, offset: U64, data: []const U8) FsError!Usize {
        return self.vtable.writeFn(self.ctx, inode, offset, data);
    }
    pub fn mkdir(self: FileSystem, parent: Inode, name: FixedStr) FsError!Inode {
        return self.vtable.mkdirFn(self.ctx, parent, name);
    }
    pub fn unlink(self: FileSystem, parent: Inode, name: FixedStr) FsError!void {
        return self.vtable.unlinkFn(self.ctx, parent, name);
    }
    pub fn stat(self: FileSystem, inode: Inode) FsError!InodeMeta {
        return self.vtable.statFn(self.ctx, inode);
    }
};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 6. SigmaExt4 — concrete filesystem implementation
//        (in-memory mock; production would use block device I/O)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

const MAX_INODES:   Usize = 256;
const MAX_CHILDREN: Usize = 16;
const MAX_DATA:     Usize = 4096;

pub const SigmaExt4Node = struct {
    meta:     InodeMeta,
    children: [MAX_CHILDREN]DirEntry,
    nchildren: Usize,
    data:     [MAX_DATA]U8,
    data_len: Usize,
    valid:    Bool,
};

pub const SigmaExt4 = struct {
    nodes:      [MAX_INODES]SigmaExt4Node,
    next_inode: Inode,

    pub fn init() SigmaExt4 {
        var fs = SigmaExt4{
            .nodes      = [_]SigmaExt4Node{.{
                .meta     = .{ .inode = 0, .kind = .Unknown, .size = 0, .link_count = 0,
                               .uid = 0, .gid = 0, .flags = .{}, .atime = 0, .mtime = 0,
                               .ctime = 0, .block_size = 4096 },
                .children = [_]DirEntry{.{.inode=0,.kind=.Unknown,.name=FixedStr{}}} ** MAX_CHILDREN,
                .nchildren = 0,
                .data     = [_]U8{0} ** MAX_DATA,
                .data_len = 0,
                .valid    = false,
            }} ** MAX_INODES,
            .next_inode = 2,  // 1 = root
        };
        // Create root inode (inode 1)
        fs.nodes[1] = SigmaExt4Node{
            .meta = .{
                .inode = 1, .kind = .Directory, .size = 0, .link_count = 2,
                .uid = 0, .gid = 0, .flags = .{}, .atime = 0, .mtime = 0, .ctime = 0,
                .block_size = 4096,
            },
            .children  = [_]DirEntry{.{.inode=0,.kind=.Unknown,.name=FixedStr{}}} ** MAX_CHILDREN,
            .nchildren = 0,
            .data      = [_]U8{0} ** MAX_DATA,
            .data_len  = 0,
            .valid     = true,
        };
        return fs;
    }

    fn allocInode(self: *SigmaExt4) ?Inode {
        if (self.next_inode >= MAX_INODES) return null;
        const id = self.next_inode;
        self.next_inode += 1;
        return id;
    }

    pub fn lookup(self: *SigmaExt4, parent: Inode, name: FixedStr) FsError!InodeMeta {
        if (parent >= MAX_INODES or !self.nodes[parent].valid) return FsError.NotFound;
        const pnode = &self.nodes[parent];
        if (pnode.meta.kind != .Directory) return FsError.NotDirectory;
        var i: Usize = 0;
        while (i < pnode.nchildren) : (i += 1) {
            if (pnode.children[i].name.eql(name)) {
                const cid = pnode.children[i].inode;
                if (cid >= MAX_INODES or !self.nodes[cid].valid) return FsError.Corrupted;
                return self.nodes[cid].meta;
            }
        }
        return FsError.NotFound;
    }

    pub fn mkdir(self: *SigmaExt4, parent: Inode, name: FixedStr) FsError!Inode {
        if (parent >= MAX_INODES or !self.nodes[parent].valid) return FsError.NotFound;
        const pnode = &self.nodes[parent];
        if (pnode.nchildren >= MAX_CHILDREN) return FsError.NoSpace;
        const newId = self.allocInode() orelse return FsError.NoSpace;
        self.nodes[newId] = SigmaExt4Node{
            .meta = .{
                .inode = newId, .kind = .Directory, .size = 0, .link_count = 2,
                .uid = 0, .gid = 0, .flags = .{}, .atime = 0, .mtime = 0, .ctime = 0,
                .block_size = 4096,
            },
            .children  = [_]DirEntry{.{.inode=0,.kind=.Unknown,.name=FixedStr{}}} ** MAX_CHILDREN,
            .nchildren = 0,
            .data      = [_]U8{0} ** MAX_DATA,
            .data_len  = 0,
            .valid     = true,
        };
        pnode.children[pnode.nchildren] = DirEntry{ .inode = newId, .kind = .Directory, .name = name };
        pnode.nchildren += 1;
        return newId;
    }

    pub fn write(self: *SigmaExt4, inode: Inode, offset: U64, data: []const U8) FsError!Usize {
        if (inode >= MAX_INODES or !self.nodes[inode].valid) return FsError.NotFound;
        const node = &self.nodes[inode];
        if (node.meta.kind != .Regular) return FsError.NotFile;
        const start = @as(Usize, @intCast(offset));
        if (start >= MAX_DATA) return FsError.NoSpace;
        var n = data.len;
        if (start + n > MAX_DATA) n = MAX_DATA - start;
        var i: Usize = 0;
        while (i < n) : (i += 1) node.data[start + i] = data[i];
        if (start + n > node.data_len) node.data_len = start + n;
        node.meta.size = node.data_len;
        return n;
    }

    pub fn read(self: *SigmaExt4, inode: Inode, offset: U64, buf: []U8) FsError!Usize {
        if (inode >= MAX_INODES or !self.nodes[inode].valid) return FsError.NotFound;
        const node = &self.nodes[inode];
        const start = @as(Usize, @intCast(offset));
        if (start >= node.data_len) return 0;
        var n = buf.len;
        const avail = node.data_len - start;
        if (n > avail) n = avail;
        var i: Usize = 0;
        while (i < n) : (i += 1) buf[i] = node.data[start + i];
        return n;
    }
};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 7. Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

test "SigmaExt4: mkdir and lookup" {
    var fs = SigmaExt4.init();
    const name = FixedStr.fromSlice("home");
    const newIno = try fs.mkdir(1, name);
    const meta = try fs.lookup(1, name);
    try @import("std").testing.expect(meta.inode == newIno);
    try @import("std").testing.expect(meta.kind == .Directory);
}

test "SigmaExt4: write and read" {
    var fs = SigmaExt4.init();
    // Manually create a regular file node (inode 2)
    fs.nodes[2] = SigmaExt4Node{
        .meta = .{
            .inode = 2, .kind = .Regular, .size = 0, .link_count = 1,
            .uid = 0, .gid = 0, .flags = .{}, .atime = 0, .mtime = 0, .ctime = 0,
            .block_size = 4096,
        },
        .children  = [_]DirEntry{.{.inode=0,.kind=.Unknown,.name=FixedStr{}}} ** MAX_CHILDREN,
        .nchildren = 0,
        .data      = [_]U8{0} ** MAX_DATA,
        .data_len  = 0,
        .valid     = true,
    };
    fs.next_inode = 3;
    const written = try fs.write(2, 0, "Hello, SigmaOS!");
    try @import("std").testing.expect(written == 15);
    var buf = [_]U8{0} ** 16;
    const n = try fs.read(2, 0, &buf);
    try @import("std").testing.expect(n == 15);
    try @import("std").testing.expectEqualSlices(U8, "Hello, SigmaOS!", buf[0..n]);
}
