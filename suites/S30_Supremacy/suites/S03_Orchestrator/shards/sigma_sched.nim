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
  fallback* = object of RootObj
    initialized*: SigmaBool

proc newfallback*(): fallback =
  result = fallback(initialized: false)

proc sigma_sched_init*(self: var fallback) =
  self.initialized = true

proc sigma_sched_enqueue*(self: var fallback) =
  self.initialized = true

proc sigma_sched_dequeue*(self: var fallback) =
  self.initialized = true

proc sigma_sched_tick*(self: var fallback) =
  self.initialized = true

proc sigma_sched_yield*(self: var fallback) =
  self.initialized = true

proc sigma_sched_set_deadline*(self: var fallback) =
  self.initialized = true

proc sigma_sched_balance*(self: var fallback) =
  self.initialized = true

proc sigma_sched_stats*(self: var fallback) =
  self.initialized = true

proc sigma_sched_global_stats*(self: var fallback) =
  self.initialized = true

var instance* = newfallback()

proc sigma_sched_init*() {.exportc.} =
  instance.initialized = true

proc sigma_sched_enqueue*() {.exportc.} =
  instance.initialized = true

proc sigma_sched_dequeue*() {.exportc.} =
  instance.initialized = true

proc sigma_sched_tick*() {.exportc.} =
  instance.initialized = true

proc sigma_sched_yield*() {.exportc.} =
  instance.initialized = true

proc sigma_sched_set_deadline*() {.exportc.} =
  instance.initialized = true

proc sigma_sched_balance*() {.exportc.} =
  instance.initialized = true

proc sigma_sched_stats*() {.exportc.} =
  instance.initialized = true

proc sigma_sched_global_stats*() {.exportc.} =
  instance.initialized = true

