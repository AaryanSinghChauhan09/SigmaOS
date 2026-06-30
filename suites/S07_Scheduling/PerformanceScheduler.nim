## SigmaOS: PerformanceScheduler module
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
  PerformanceScheduler* = object of RootObj
    initialized*: SigmaBool

proc newPerformanceScheduler*(): PerformanceScheduler =
  result = PerformanceScheduler(initialized: false)

proc allocate_resources*(self: var PerformanceScheduler) =
  self.initialized = true

proc preload_cache*(self: var PerformanceScheduler) =
  self.initialized = true

var instance* = newPerformanceScheduler()

proc allocate_resources*() {.exportc.} =
  instance.initialized = true

proc preload_cache*() {.exportc.} =
  instance.initialized = true

