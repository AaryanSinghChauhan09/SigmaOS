## SigmaOS: SIGMA_MEM_TCACHE_H */
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
  free* = object of RootObj
    initialized*: SigmaBool

proc newfree*(): free =
  result = free(initialized: false)

proc tcache_init*(self: var free) =
  self.initialized = true

proc tcache_class*(self: var free) =
  self.initialized = true

proc tcache_free*(self: var free) =
  self.initialized = true

type
  SigmaTCacheFreeList* = object
    depth*: SigmaI32

type
  SigmaTCache* = object
    backing_used*: SigmaU64
    lock*: SigmaU64
    alloc_count*: SigmaU64
    free_count*: SigmaU64
    cache_hits*: SigmaU64

var instance* = newfree()

proc tcache_init*() {.exportc.} =
  instance.initialized = true

proc tcache_free*() {.exportc.} =
  instance.initialized = true

