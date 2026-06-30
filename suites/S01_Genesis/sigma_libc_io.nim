## SigmaOS: sigma_libc_io module
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
  IBackendIO* = object of RootObj
    initialized*: SigmaBool

proc newIBackendIO*(): IBackendIO =
  result = IBackendIO(initialized: false)

proc outb*(self: var IBackendIO) =
  self.initialized = true

proc print*(self: var IBackendIO) =
  self.initialized = true

var instance* = newIBackendIO()

proc outb*() {.exportc.} =
  instance.initialized = true

proc print*() {.exportc.} =
  instance.initialized = true

