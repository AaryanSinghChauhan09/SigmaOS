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
  vfs_node* = object
    type*: SigmaU64
    inode_id*: SigmaU32
    size*: SigmaU64
    read*: SigmaU64
    write*: SigmaU64
    open*: SigmaU64
    close*: SigmaU64

proc sigma_vfs_init*() {.exportc.} =
  discard

