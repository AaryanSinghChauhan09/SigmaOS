## SigmaOS: SIGMA_SEC_SANDBOX_H */
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
  SovereignSandbox* = object of RootObj
    initialized*: SigmaBool

proc newSovereignSandbox*(): SovereignSandbox =
  result = SovereignSandbox(initialized: false)

type
  SigmaSandboxConfig* = object
    mode*: uint8
    allow_network*: uint8
    allow_fs_write*: uint8
    allow_ipc*: uint8
    max_memory_mb*: SigmaI32

var instance* = newSovereignSandbox()

