## SigmaOS: sigma_libc_mem module
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
  MemoryOps* = object of RootObj
    initialized*: SigmaBool

proc newMemoryOps*(): MemoryOps =
  result = MemoryOps(initialized: false)

proc memcmp*(self: var MemoryOps) =
  self.initialized = true

proc secure_zero*(self: var MemoryOps) =
  self.initialized = true

var instance* = newMemoryOps()

proc secure_zero*() {.exportc.} =
  instance.initialized = true

