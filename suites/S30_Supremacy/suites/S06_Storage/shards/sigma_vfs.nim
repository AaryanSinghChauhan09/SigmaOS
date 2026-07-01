## SigmaOS: =========================================================================
## Migrated from C/C++ to Nim — no stdlib import, no external packages.
## All types hand-defined. OOP via object hierarchy + method dispatch.
{.push raises: [].}

type
  SigmaU8*  = uint8
  SigmaU16* = uint16
  SigmaU32* = uint32
  SigmaU64* = uint64
  SigmaI32* = int32
  SigmaI64* = int64
  SigmaBool* = bool
  SigmaUsize* = uint

type
  sigma_inode_s* = object
    ino*: SigmaU64
    mode*: SigmaU64
    uid*: SigmaU64
    gid*: SigmaU64
    size*: SigmaU64
    atime*: SigmaU64
    mtime*: SigmaU64
    ctime*: SigmaU64
    nlink*: SigmaU64
    ref_count*: SigmaU64
    dirty*: SigmaU64

type
  sigma_dentry_s* = object
    child_count*: SigmaU64
    is_mountpoint*: SigmaU64

